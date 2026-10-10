#!/usr/bin/env perl
use strict;
use warnings;
use JSON::PP;
use Digest::SHA qw(sha256_hex);

sub read_file {
    my ($path, $commit) = @_;
    my $fh;
    if (defined $commit) {
        open $fh, '-|', 'git', 'show', "$commit:$path" or die "$path: $!\n";
    } else {
        open $fh, '<', $path or die "$path: $!\n";
    }
    binmode $fh;
    local $/;
    my $text = <$fh>;
    close $fh or die "cannot read $path" . (defined $commit ? " at $commit" : '') . "\n";
    return $text;
}

sub write_file {
    my ($path, $text) = @_;
    open my $fh, '>', $path or die "$path: $!\n";
    print {$fh} $text;
    close $fh or die "$path: $!\n";
}

sub version {
    my ($commit) = @_;
    my $text = read_file('Cargo.toml', $commit);
    $text =~ /^\[workspace\.package\]\n(.*?)(?=^\[|\z)/ms
        or die "missing workspace.package\n";
    my $section = $1;
    $section =~ /^version = "([^"]+)"$/m or die "missing workspace version\n";
    return $1;
}

sub parts {
    my ($value) = @_;
    $value =~ /\A(0|[1-9][0-9]{0,8})\.(0|[1-9][0-9]{0,8})\.(0|[1-9][0-9]{0,8})\z/
        or die "expected canonical x.y.z with components below 1 billion\n";
    return ($1, $2, $3);
}

sub changelog {
    die "expected target, saved previous version and date\n" unless @_ == 3;
    my ($target, $previous, $date) = @_;
    parts($target);
    parts($previous);
    $date =~ /\A[0-9]{4}-[0-9]{2}-[0-9]{2}\z/ or die "invalid release date\n";
    my $text = read_file('CHANGELOG.md');
    my @sections = $text =~ /^## \[([^\]]+)\](?: - [0-9]{4}-[0-9]{2}-[0-9]{2})?$/mg;
    my %seen;
    for my $section (@sections) {
        die "duplicate changelog section: $section\n" if $seen{$section}++;
    }
    # The saved intent remains authoritative after package metadata is bumped.
    # Recovery verifies prepared bytes; it never re-finalizes an already dated
    # candidate. Note content is maintained separately, not a release gate.
    open my $finalizer, '-|', 'awk', '-v', "version=$target", '-v',
        "previous=$previous", '-v', "date=$date", '-v', 'allow_finalized=0',
        '-f', 'scripts/ci/finalize-release-changelog.awk', 'CHANGELOG.md'
        or die "cannot start changelog finalizer: $!\n";
    my $candidate = do { local $/; <$finalizer> };
    close $finalizer or die "changelog finalization failed\n";
    defined($candidate) && length($candidate) or die "empty finalizer output\n";
    return $candidate;
}

my ($command, @args) = @ARGV;
$command //= '';
if ($command eq 'version') {
    my $value = version();
    parts($value);
    print "$value\n";
} elsif ($command eq 'changelog-check' || $command eq 'finalize') {
    my $text = changelog(@args);
    write_file('CHANGELOG.md', $text) if $command eq 'finalize';
} elsif ($command eq 'set-version') {
    my ($target, $metadata_path) = @args;
    parts($target);
    my $old = version();
    my $metadata = decode_json(read_file($metadata_path));
    my %ids = map { $_ => 1 } @{$metadata->{workspace_members}};
    my %names;
    for my $package (@{$metadata->{packages}}) {
        next unless $ids{$package->{id}};
        die "workspace member has independent version or source\n"
            unless $package->{version} eq $old && !defined($package->{source});
        die "duplicate workspace package name\n" if $names{$package->{name}}++;
    }
    die "incomplete workspace metadata\n"
        unless keys(%names) && keys(%names) == keys(%ids);
    open my $rewriter, '-|', $^X, 'scripts/ci/rewrite-local-lock-versions.pl',
        'Cargo.lock', $old, $target, sort keys %names
        or die "cannot start lockfile transformer: $!\n";
    my $lock = do { local $/; <$rewriter> };
    close $rewriter or die "lockfile transformation failed\n";
    defined($lock) && length($lock) or die "empty transformed lockfile\n";
    my $text = read_file('Cargo.toml');
    # Bounded root catalog entries only; reject unsupported or stale local requirements.
    # Cargo owns dependency ordering and packaging, not another release state machine.
    $text =~ s!^([A-Za-z0-9_-]+ = \{[^\n]*\bpath = "[^"\n]+"[^\n]*\})$!
        my ($entry, $name) = ($1, $1);
        $name =~ s/ = .*//;
        die "local catalog entry is not a workspace member: $name\n" unless $names{$name};
        if ($entry =~ /\bversion = "([^"\n]+)"/) {
            die "local catalog version mismatch: $name\n" unless $1 eq $old;
            $entry =~ s/(\bversion = ")[^"\n]+("[ ,}])/$1$target$2/;
        }
        $entry;
    !gme;
    $text =~ s/(^\[workspace\.package\]\n(?:(?!^\[).)*?^version = ")[^"]+(")$/$1$target$2/ms
        or die "cannot update workspace version\n";
    write_file('Cargo.toml', $text);
    write_file('Cargo.lock', $lock);
} elsif ($command eq 'package-lock-check') {
    # Cargo resolves the extracted standalone payload; every external selection must
    # already be in the frozen workspace lock, with exactly its original checksum.
    my ($entry_path, $payload_path) = @args;
    sub registry_rows {
        my ($path) = @_;
        my %rows;
        for my $section (split /(?=^\[\[package\]\]$)/m, read_file($path)) {
            next unless $section =~ /^source = "([^"\n]+)"$/m;
            my $source = $1;
            $section =~ /^name = "([^"\n]+)"$/m or die "missing package name\n";
            my $name = $1;
            $section =~ /^version = "([^"\n]+)"$/m or die "missing package version\n";
            my $version = $1;
            my $checksum = '';
            if ($source =~ /^registry\+/) {
                $section =~ /^checksum = "([0-9a-f]{64})"$/m or die "missing package checksum\n";
                $checksum = $1;
            }
            my $key = join(' ', $name, $version, $source);
            die "duplicate package selection\n" if exists $rows{$key};
            $rows{$key} = $checksum;
        }
        return \%rows;
    }
    my $entry = registry_rows($entry_path);
    my $payload = registry_rows($payload_path);
    die "empty package external graph\n" unless keys %$payload;
    for my $key (sort keys %$payload) {
        die "packaging changed external selection: $key\n"
            unless exists $entry->{$key} && $entry->{$key} eq $payload->{$key};
    }
    print "packaged external selections match frozen workspace lock\n";
} elsif ($command eq 'publication-archive') {
    die "expected archive, checksum, VCS record, source and package\n" unless @args == 5;
    my ($archive, $checksum, $vcs_path, $head, $package) = @args;
    die "registry archive checksum mismatch\n" unless sha256_hex(read_file($archive)) eq $checksum;
    my $vcs = decode_json(read_file($vcs_path));
    die "registry archive has another source identity\n"
        unless ref($vcs->{git}) eq 'HASH' && defined $vcs->{git}->{sha1}
            && $vcs->{git}->{sha1} eq $head
            && (!exists $vcs->{git}->{dirty} || (JSON::PP::is_bool($vcs->{git}->{dirty}) && !$vcs->{git}->{dirty}))
            && defined $vcs->{path_in_vcs} && $vcs->{path_in_vcs} eq "crates/$package";
} elsif ($command eq 'index-check') {
    my @paths = split /\0/, read_file($args[0]);
    die "unexpected release index\n" unless join(',', sort @paths)
        eq 'CHANGELOG.md,Cargo.lock,Cargo.toml,docs/release.json';
} elsif ($command eq 'receipt') {
    my ($source, $date) = @args;
    my %hashes = map { $_ => sha256_hex(read_file($_)) }
        qw(Cargo.toml Cargo.lock CHANGELOG.md);
    my $receipt = {
        schema => 1, version => version(), source => $source, date => $date,
        gate => 'release-verify', files => \%hashes,
    };
    write_file('docs/release.json', JSON::PP->new->canonical->pretty->encode($receipt));
} elsif ($command eq 'verify' || $command eq 'verify-commit' || $command eq 'source') {
    # Recovery checks the original committed payload, even after newer fixes.
    my $commit;
    if ($command eq 'verify-commit') {
        die "expected commit, source, version and date\n" unless @args == 4;
        $commit = shift @args;
        die "invalid release commit\n" unless $commit =~ /\A[0-9a-f]{40,64}\z/;
    }
    my $receipt = decode_json(read_file('docs/release.json', $commit));
    die "unsupported release receipt\n" unless $receipt->{schema} == 1;
    die "release version mismatch\n" unless $receipt->{version} eq version($commit);
    die "invalid release source\n" unless $receipt->{source} =~ /\A[0-9a-f]{40,64}\z/;
    die "invalid release gate\n" unless $receipt->{gate} eq 'release-verify';
    die "invalid release date\n" unless $receipt->{date} =~ /\A[0-9]{4}-[0-9]{2}-[0-9]{2}\z/;
    if (@args) {
        die "expected source, version and date\n" unless @args == 3;
        die "receipt differs from saved release identity\n"
            unless $receipt->{source} eq $args[0] && $receipt->{version} eq $args[1]
                && $receipt->{date} eq $args[2];
    }
    my @files = sort keys %{$receipt->{files}};
    die "unexpected receipt files\n"
        unless join(',', @files) eq 'CHANGELOG.md,Cargo.lock,Cargo.toml';
    for my $path (@files) {
        die "release file changed after validation: $path\n"
            unless sha256_hex(read_file($path, $commit)) eq $receipt->{files}{$path};
    }
    my $notes = read_file('CHANGELOG.md', $commit);
    my $heading = "## [$receipt->{version}] - $receipt->{date}";
    die "dated changelog missing\n" unless $notes =~ /^\Q$heading\E$/m;
    print "$receipt->{source}\n" if $command eq 'source';
} else {
    die "unknown release-data command\n";
}
