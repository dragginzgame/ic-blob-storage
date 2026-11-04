#!/usr/bin/env perl
use strict;
use warnings;
use JSON::PP;
use Digest::SHA qw(sha256_hex);

sub read_file {
    my ($path, $commit) = @_;
    my $fh;
    if (defined $commit) { open $fh, '-|', 'git', 'show', "$commit:$path" or die "$path: $!\n"; }
    else { open $fh, '<', $path or die "$path: $!\n"; }
    binmode $fh;
    local $/;
    my $text = <$fh>;
    close $fh or die "cannot read $path\n";
    return $text;
}
sub plan {
    my ($commit) = @_;
    local $ENV{RELEASE_COMMIT} = $commit if defined $commit;
    open my $fh, '-|', 'bash', 'scripts/release/group.sh', 'plan' or die $!;
    local $/;
    my $text = <$fh>;
    close $fh or die "release group admission failed\n";
    return decode_json($text);
}
sub receipt_path {
    my $group = $ENV{RELEASE_GROUP} // '';
    die "select release group\n" unless $group =~ /\A(?:contracts|client|engine)\z/;
    return "releases/$group/release.json";
}
my ($command, @args) = @ARGV;
$command //= '';
if ($command eq 'package-version') {
    # Descriptive Make/package inputs; release admission independently checks
    # every catalog requirement against its member's literal package version.
    my ($name) = @args;
    die "invalid package name\n" unless defined $name && $name =~ /\A[a-z0-9_-]+\z/;
    my $root = read_file('Cargo.toml');
    $root =~ /^\[workspace\.dependencies\]\n(.*?)(?=^\[|\z)/ms or die "missing catalog\n";
    my $catalog = $1;
    $catalog =~ /^\Q$name\E = \{[^\n]*version = "([0-9]+\.[0-9]+\.[0-9]+)"/m or die "missing package version: $name\n";
    print "$1\n";
    exit;
}
if ($command eq 'publication-archive') {
    die "expected archive, checksum, VCS, source and package\n" unless @args == 5;
    my ($archive,$checksum,$vcs_path,$head,$package) = @args;
    die "registry archive checksum mismatch\n" unless sha256_hex(read_file($archive)) eq $checksum;
    my $vcs = decode_json(read_file($vcs_path));
    die "registry archive has another source identity\n" unless
        ref($vcs->{git}) eq 'HASH' && ($vcs->{git}{sha1} // '') eq $head &&
        (!exists $vcs->{git}{dirty} || (JSON::PP::is_bool($vcs->{git}{dirty}) && !$vcs->{git}{dirty})) &&
        ($vcs->{path_in_vcs} // '') eq "crates/$package";
    exit;
}
my $commit;
if ($command eq 'verify-commit') {
    die "expected commit, source, version and date\n" unless @args == 4;
    $commit = shift @args;
    die "invalid release commit\n" unless $commit =~ /\A[0-9a-f]{40,64}\z/;
}
my $plan = plan($commit);
my $receipt_path = receipt_path();
my @payload = sort grep { $_ ne $receipt_path } @{$plan->{files}};
if ($command eq 'version') {
    print "$plan->{previous}\n";
} elsif ($command eq 'index-check') {
    my @paths = sort split /\0/, read_file($args[0]);
    die "unexpected release index\n" unless join("\0",@paths) eq join("\0",sort @{$plan->{files}});
} elsif ($command eq 'receipt') {
    my ($source,$date) = @args;
    die "invalid receipt source/date\n" unless defined($source) && defined($date) &&
        $source =~ /\A[0-9a-f]{40,64}\z/ && $date =~ /\A[0-9]{4}-[0-9]{2}-[0-9]{2}\z/;
    my %hashes = map { $_ => sha256_hex(read_file($_)) } @payload;
    require './scripts/release/validation.pl';
    my $validation = decode_json(read_file($ENV{BLOB_RELEASE_VALIDATION_RECORD} // die "missing release validation record\n"));
    BlobValidation::verify($validation,$source,$plan->{group});
    my $receipt = {schema=>1,status=>'released',group=>$plan->{group},version=>$plan->{previous},
        source=>$source,date=>$date,gate=>'release-verify-stages-v1',validation=>$validation,files=>\%hashes,
        policy=>sha256_hex(read_file('ci/cargo-release-groups.json'))};
    open my $fh, '>', $receipt_path or die $!;
    print {$fh} JSON::PP->new->canonical->pretty->encode($receipt);
    close $fh or die $!;
} elsif ($command eq 'verify' || $command eq 'verify-commit' || $command eq 'source') {
    my $receipt = decode_json(read_file($receipt_path,$commit));
    die "no completed release for selected group\n" unless ($receipt->{schema}//0)==1 &&
        ($receipt->{status}//'') eq 'released' && ($receipt->{group}//'') eq $plan->{group};
    die "release version mismatch\n" unless ($receipt->{version}//'') eq $plan->{previous};
    die "invalid release source/date/gate\n" unless ($receipt->{source}//'') =~ /\A[0-9a-f]{40,64}\z/ &&
        ($receipt->{date}//'') =~ /\A[0-9]{4}-[0-9]{2}-[0-9]{2}\z/ && ($receipt->{gate}//'') eq 'release-verify-stages-v1';
    require './scripts/release/validation.pl';
    BlobValidation::verify($receipt->{validation},$receipt->{source},$plan->{group});
    die "release policy changed\n" unless ($receipt->{policy}//'') eq sha256_hex(read_file('ci/cargo-release-groups.json',$commit));
    if (@args) {
        die "expected source, version and date\n" unless @args==3;
        die "receipt differs from saved release identity\n" unless $receipt->{source} eq $args[0] &&
            $receipt->{version} eq $args[1] && $receipt->{date} eq $args[2];
    }
    die "unexpected receipt files\n" unless ref($receipt->{files}) eq 'HASH' &&
        join("\0",sort keys %{$receipt->{files}}) eq join("\0",@payload);
    for my $path (@payload) {
        die "release file changed after validation: $path\n" unless sha256_hex(read_file($path,$commit)) eq $receipt->{files}{$path};
    }
    my $notes=read_file($plan->{changelog},$commit);
    my $heading="## [$receipt->{version}] - $receipt->{date}";
    die "dated changelog missing\n" unless $notes =~ /^\Q$heading\E$/m;
    print "$receipt->{source}\n" if $command eq 'source';
} else { die "unknown release-data command\n"; }
