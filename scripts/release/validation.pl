#!/usr/bin/env perl
# Bounded consumer selection/receipt data. Shared owns generic plan admission,
# checkpointing, dispatch and failure retention; no validation result is reused.
package BlobValidation;
use strict;
use warnings;
use JSON::PP;
use Digest::SHA qw(sha256_hex);
use File::Temp qw(tempdir);
use File::Path qw(make_path);
use Cwd qw(abs_path);
use File::Basename qw(dirname);
my $json = JSON::PP->new->canonical->utf8;
my $policy_path = 'ci/release-validation.json';

sub read_file {
    my ($path) = @_;
    -f $path && !-l $path or die "missing regular validation evidence: $path\n";
    open my $fh, '<:raw', $path or die "$path: $!\n";
    local $/; my $bytes = <$fh>;
    close $fh or die "$path: $!\n";
    return $bytes;
}
sub write_file {
    my ($path, $bytes) = @_;
    open my $fh, '>:raw', $path or die "$path: $!\n";
    print {$fh} $bytes or die "$path: $!\n";
    close $fh or die "$path: $!\n";
}
sub encode { $json->encode($_[0])."\n" }
sub git {
    my (@args) = @_;
    open my $fh, '-|', 'git', '-C', $ENV{BLOB_RELEASE_SOURCE_ROOT}//'.', @args or die $!;
    local $/; my $bytes = <$fh>;
    close $fh or die "validation source observation failed: @args\n";
    return $bytes;
}
sub blob { git('show', "$_[0]:$_[1]") }
sub source_files { split /\0/, git('ls-tree', '-rz', '--name-only', $_[0]) }
sub configuration {
    # Hash values, never retain credentials or arbitrary configuration contents.
    # Do not interpret Cargo configuration or project flags locally. Any such
    # override keeps the full roster; only the standard configuration may narrow.
    my %environment = map { $_=>sha256_hex($ENV{$_}) } grep {
        /\A(?:RUST|CARGO_|CC\z|CXX\z|AR\z|LD\z|CFLAGS\z|CXXFLAGS\z|CPPFLAGS\z|LDFLAGS\z|PKG_CONFIG|OPENSSL|BINDGEN|LIBCLANG|SDKROOT\z|MACOSX_DEPLOYMENT_TARGET\z)/
        && !/\A(?:CARGO_HOME|CARGO_TARGET_DIR|CARGO_NET_OFFLINE|CARGO_TERM_COLOR|RUSTUP_HOME|RUSTUP_AUTO_INSTALL)\z/
        && length $ENV{$_}
    } keys %ENV;
    my %configs;
    my $root = abs_path($ENV{BLOB_RELEASE_SOURCE_ROOT}//'.');
    my $depth = 0;
    while (1) {
        for my $name ('config','config.toml') {
            my $path = "$root/.cargo/$name";
            $configs{"ancestor-$depth/$name"} = sha256_hex(read_file($path)) if -e $path || -l $path;
        }
        last if $root eq '/';
        $root = dirname($root); ++$depth;
    }
    my $cargo_home = $ENV{CARGO_HOME}//"$ENV{HOME}/.cargo";
    for my $name ('config','config.toml') {
        my $path = "$cargo_home/$name";
        $configs{"cargo-home/$name"} = sha256_hex(read_file($path)) if -e $path || -l $path;
    }
    return {schema=>1,environment=>\%environment,cargo_config=>\%configs};
}
sub inputs {
    my ($source) = @_;
    # Source admission also checks the entire checkout before/after execution.
    # The immutable source binds product code; checkpoint every selection,
    # recipe, supplier, feature and generated-input configuration independently.
    return [sort grep {
        m{\A(?:scripts/|make/|ci/|\.github/|\.cargo/|releases/)} ||
        m{(?:\A|/)Cargo\.(?:toml|lock)\z} ||
        /\A(?:Makefile|rust-toolchain\.toml|\.shared-tooling\.snapshot)\z/ ||
        m{\Atests/browser/(?:\.nvmrc|package(?:-lock)?\.json)\z}
    } source_files($source)];
}
sub selection {
    my ($source, $group, $context_path, $context) = @_;
    $source =~ /\A[0-9a-f]{40,64}\z/ && $group =~ /\A(?:contracts|client|engine)\z/
        or die "select exact release source/group\n";
    my $policy = $json->decode(blob($source,$policy_path));
    blob($source,$policy_path) eq encode($policy) or die "noncanonical validation policy\n";
    my $groups = $json->decode(blob($source,'ci/cargo-release-groups.json'));
    join(',',sort @{$policy->{groups}}) eq join(',',sort map { $_->{name} } @{$groups->{groups}})
        or die "validation/release group rosters differ\n";
    my @rosters = blob($source,'Makefile') =~ /^CI_TARGETS := (.*)$/mg;
    @rosters == 1 && join(' ',@{$policy->{stages}}) eq $rosters[0]
        or die "validation/full-gate stage rosters differ\n";
    my $scope = 'workspace';
    my $reason = 'Shared, unknown or not yet classified inputs require the full roster';
    my $client_only = 0;
    # Admit the whole delta since this group's annotated release, never only the
    # last commit. New helpers/policies, missing history or any unknown path keep
    # the full gate. The first delivery of this mapping is therefore full.
    eval {
        keys(%{$context->{environment}}) == 0 && keys(%{$context->{cargo_config}}) == 0
            or die "ambient build configuration requires full validation";
        my ($entry) = grep { $_->{name} eq $group } @{$groups->{groups}};
        my ($version) = blob($source,"crates/$entry->{primary}/Cargo.toml") =~ /^version = "([0-9.]+)"$/m;
        defined $version or die "no baseline version";
        my $tag = "$group-v$version";
        git('for-each-ref','--format=%(objecttype)',"refs/tags/$tag") eq "tag\n" or die "no annotated baseline";
        my $base = git('rev-parse',"refs/tags/$tag^{commit}"); $base =~ s/\n\z//;
        git('merge-base','--is-ancestor',$base,$source);
        my %baseline_files = map { $_=>1 } source_files($base);
        $baseline_files{"releases/$group/release.json"} or die "missing baseline receipt";
        my $receipt = $json->decode(blob($base,"releases/$group/release.json"));
        ($receipt->{gate}//'') eq 'release-verify-plan-v1'
            && ($receipt->{validation}{kind}//'') eq 'blob-release-plan-v1'
            && ($receipt->{validation}{group}//'') eq $group
            or die "baseline predates qualified plan receipts";
        encode($receipt->{validation}{configuration}) eq encode($context)
            or die "baseline configuration differs";
        my @changed = split /\0/, git('diff','--no-renames','--name-only','-z',$base,$source);
        @changed or die "no new release inputs";
        my $only_client = 1;
        for my $path (@changed) {
            next if $path =~ m{\Areleases/(?:contracts|client|engine)/CHANGELOG\.md\z};
            next if $path eq 'docs/status/current.md';
            if ($path =~ m{\Acrates/ic-blob-storage-client/src/tenants/} && $path =~ /\.rs\z/) { next; }
            $only_client = 0;
            next if $path =~ m{\Acrates/ic-blob-storage-(?:client|cli)/src/} && $path =~ /\.rs\z/;
            next if $path =~ m{\A(?:crates/ic-blob-storage(?:-contracts)?/src/|canisters/(?:standalone|test)/|tests/(?:pocketic|protocol)/)}
                && $path =~ /\.rs\z/;
            die "unclassified release input: $path";
        }
        $scope = 'group';
        $client_only = $group eq 'client' && $only_client;
        $reason = "Complete source delta from $tag at $base leaves release infrastructure unchanged";
    };
    my @stages = map {
        my $skip = $scope eq 'group' && ($_ eq 'release-check' || ($client_only && $_ eq 'test-pocketic-engine'));
        {target=>$_, required=>$skip ? JSON::PP::false : JSON::PP::true,
            reason=>$skip ? ($_ eq 'release-check'
                ? 'Unchanged release machinery; real group renderer/rollback checks remain required'
                : 'Tenant-client-only implementation; installed Client/CLI/host and replicated tenant-client boundaries remain required') : $reason}
    } @{$policy->{stages}};
    return {schema=>1,source=>$source,group=>$group,scope=>$scope,policy=>$policy_path,
        inputs=>[sort(@{inputs($source)},$context_path)],stages=>\@stages};
}
sub verify {
    my ($record,$source,$group) = @_;
    ref($record) eq 'HASH' && ($record->{kind}//'') eq 'blob-release-plan-v1'
        && ($record->{schema}//0)==1 && ($record->{source}//'') eq $source && ($record->{group}//'') eq $group
        or die "missing or mismatched structured validation proof\n";
    my $path = $record->{plan_path}//'';
    $path =~ m{\A\.tmp/blob-release-validation\.[A-Za-z0-9]+/plan\.json\z} or die "invalid retained plan path\n";
    (my $context_path = $path) =~ s{/plan\.json\z}{/configuration.json};
    my $context = $record->{configuration};
    ref($context) eq 'HASH' && ($context->{schema}//0)==1
        && ref($context->{environment}) eq 'HASH' && ref($context->{cargo_config}) eq 'HASH'
        && join(',',sort keys %$context) eq 'cargo_config,environment,schema'
        or die "missing validation configuration\n";
    for my $hash (values %{$context->{environment}},values %{$context->{cargo_config}}) {
        defined($hash) && !ref($hash) && $hash =~ /\A[0-9a-f]{64}\z/ or die "invalid validation configuration digest\n";
    }
    encode($record->{plan}) eq encode(selection($source,$group,$context_path,$context)) or die "validation selection changed\n";
    encode($record->{policy}) eq blob($source,$policy_path) or die "validation policy changed\n";
    my @expected = sort ($path,@{$record->{plan}{inputs}});
    ref($record->{checkpoint}) eq 'HASH' && join("\0",sort keys %{$record->{checkpoint}}) eq join("\0",@expected)
        or die "incomplete validation input checkpoint\n";
    for my $input (@expected) {
        my $bytes = $input eq $path ? encode($record->{plan}) : $input eq $context_path ? encode($context) : blob($source,$input);
        ($record->{checkpoint}{$input}//'') eq sha256_hex($bytes) or die "validation input proof changed: $input\n";
    }
    # Reuse Shared's strict schema/roster/path admission for embedded documents.
    my $tmp = tempdir('blob-plan-proof.XXXXXX',TMPDIR=>1,CLEANUP=>1);
    make_path("$tmp/ci");
    write_file("$tmp/plan.json",encode($record->{plan}));
    write_file("$tmp/$policy_path",encode($record->{policy}));
    write_file("$tmp/checkpoint.json","{}\n");
    open my $admission, '-|', 'perl', 'scripts/ci/release-validation-plan.pl', $tmp,'plan.json',$source,$group,
        "$tmp/checkpoint.json","$tmp/admitted.json" or die $!;
    { local $/; <$admission>; }
    close $admission or die "embedded plan admission refused\n";
    ref($record->{host}) eq 'HASH' && !grep { !defined($_) || !/\S/ } @{$record->{host}}{qw(os arch compiler)}
        or die "missing validation host/compiler\n";
    my $stages = $record->{stages};
    ref($stages) eq 'ARRAY' && @$stages == @{$record->{plan}{stages}} or die "missing validation results\n";
    for my $i (0..$#$stages) {
        my $expected = $record->{plan}{stages}[$i]; my $actual = $stages->[$i];
        ($actual->{target}//'') eq $expected->{target} or die "wrong validation result target\n";
        ($actual->{state}//'') eq ($expected->{required} ? 'PASS' : 'not-applicable') or die "unfulfilled validation obligation\n";
        if ($expected->{required}) {
            ($actual->{log_sha256}//'') =~ /\A[0-9a-f]{64}\z/ && ($actual->{seconds}//'') =~ /\A[0-9]+\z/
                or die "missing execution log identity/timing\n";
        }
    }
    return 1;
}
sub main {
    my ($command,@args) = @ARGV;
    my ($source,$group) = @ENV{qw(RELEASE_SOURCE RELEASE_GROUP)};
    if ($command eq 'plan') {
        @args == 1 or die "expected output plan path\n";
        $args[0] =~ m{/plan\.json\z} or die "expected plan.json\n";
        (my $context_path = $args[0]) =~ s{/plan\.json\z}{/configuration.json};
        my $context = configuration();
        write_file($context_path,encode($context));
        write_file($args[0],encode(selection($source,$group,$context_path,$context)));
    } elsif ($command eq 'check') {
        @args == 1 or die "expected validation record\n";
        my $record = $json->decode(read_file($args[0]));
        encode($record->{configuration}) eq encode(configuration()) or die "release configuration changed after validation\n";
        verify($record,$source,$group);
    } elsif ($command eq 'capture') {
        @args == 2 or die "expected evidence directory and record path\n";
        my @dirs = glob "$args[0]/validation.*";
        @dirs == 1 or die "expected one retained runner attempt\n";
        my $dir = $dirs[0];
        my $plan = $json->decode(read_file("$dir/parsed-inputs.json.plan.json"));
        my $context = $json->decode(read_file("$args[0]/configuration.json"));
        encode($context) eq encode(configuration()) or die "release configuration changed during validation\n";
        my %required = map { $_->{target}=>1 } grep { $_->{required} } @{$plan->{stages}};
        my %events;
        for (split /\n/,read_file("$dir/plan-events.tsv")) {
            my ($target,$state) = split /\t/; $events{$target} = $state;
        }
        my %runs;
        for (split /\n/,read_file("$dir/timings.tsv")) {
            my ($target,$result,$seconds,$log) = split /\t/;
            next if $target eq 'target';
            $required{$target} && !$runs{$target} && $result eq 'PASS' && ($events{$target}//'') eq 'PASS'
                or die "failed or duplicate validation execution\n";
            $runs{$target} = {target=>$target,state=>$result,seconds=>$seconds,log_sha256=>sha256_hex(read_file($log))};
        }
        my @stages = map {
            $_->{required} ? ($runs{$_->{target}} // die "missing execution\n") :
                {target=>$_->{target},state=>($events{$_->{target}}//'blocked')}
        } @{$plan->{stages}};
        open my $compiler_info, '-|', ($ENV{RUSTC} || 'rustc'), '--version', '--verbose' or die $!;
        my $compiler = do { local $/; <$compiler_info> };
        close $compiler_info or die "compiler observation failed\n";
        my $os = `uname -s`; my $arch = `uname -m`; s/\n\z// for $os,$arch;
        my $record = {schema=>1,kind=>'blob-release-plan-v1',source=>$source,group=>$group,plan=>$plan,
            configuration=>$context,
            plan_path=>"$args[0]/plan.json",policy=>$json->decode(read_file("$dir/parsed-inputs.json.policy.json")),
            checkpoint=>$json->decode(read_file("$dir/plan-inputs.json")),
            host=>{os=>$os,arch=>$arch,compiler=>$compiler},stages=>\@stages};
        verify($record,$source,$group);
        write_file("$args[1].candidate",encode($record));
        rename "$args[1].candidate",$args[1] or die "retain validation proof: $!\n";
    } else { die "expected plan, check or capture\n"; }
}
main() unless caller;
1;
