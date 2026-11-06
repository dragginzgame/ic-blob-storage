#!/usr/bin/env perl
# Bounded consumer selection/receipt data. Shared owns generic plan admission,
# checkpointing, exact candidate execution/reuse and failure retention.
package BlobValidation;
use strict;
use warnings;
use JSON::PP;
use Digest::SHA qw(sha256_hex);
use File::Temp qw(tempdir);
use File::Path qw(make_path);
use Cwd qw(abs_path getcwd);
use File::Basename qw(dirname);
my $json = JSON::PP->new->canonical->utf8;
my $policy_path = 'ci/release-validation.json';
my $packages_path = 'ci/release-package-scopes.json';

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
sub observation {
    open my $in, '-|', @_ or die "start validation observation: $!\n";
    binmode $in;
    my $raw = '';
    while (1) {
        my $n = read $in, my $chunk, 65536;
        defined $n or die "read validation observation: $!\n";
        last unless $n;
        $raw .= $chunk;
        length($raw) <= 4194304 or die "validation observation exceeds 4 MiB\n";
    }
    close $in or die "validation observation failed: @_\n";
    return $raw;
}
sub shared_observation { observation($^X,'scripts/ci/validation-evidence.pl',@_) }
sub workspace_members {
    my $root = abs_path($ENV{BLOB_RELEASE_SOURCE_ROOT}//'.');
    my $bytes = observation('cargo', 'metadata', '--offline', '--locked', '--no-deps',
        '--format-version', '1', '--manifest-path', "$root/Cargo.toml");
    my $metadata = $json->decode($bytes);
    $metadata->{workspace_root} eq $root or die "Cargo observed another workspace\n";
    my %ids = map { $_=>1 } @{$metadata->{workspace_members}};
    my %members;
    for my $package (@{$metadata->{packages}}) {
        next unless delete $ids{$package->{id}};
        my ($name,$path) = @{$package}{qw(name manifest_path)};
        $path =~ s{\A\Q$root\E/}{} or die "Cargo member outside workspace\n";
        $name =~ /\A[A-Za-z0-9][A-Za-z0-9_-]*\z/ && !exists($members{$name})
            && $path =~ m{\A[^\0\r\n]+/Cargo\.toml\z} && $path !~ m{(?:\A|/)\.{1,2}(?:/|\z)|//}
            or die "invalid Cargo member identity/path\n";
        $members{$name} = $path;
    }
    !keys(%ids) && keys(%members) or die "incomplete Cargo member observation\n";
    return \%members;
}
sub stage_policy {
    my ($source) = @_;
    my $bytes = defined($source) ? blob($source,$policy_path) : read_file($policy_path);
    my $policy = $json->decode($bytes);
    my $groups = $json->decode(defined($source) ? blob($source,'ci/cargo-release-groups.json') : read_file('ci/cargo-release-groups.json'));
    make_path('target');
    my $tmp = tempdir('blob-validation-roster.XXXXXX',DIR=>'target',CLEANUP=>1);
    write_file("$tmp/policy.json",$bytes);
    write_file("$tmp/groups.json",encode([map { $_->{name} } @{$groups->{groups}}]));
    shared_observation('roster',$tmp,'policy.json','groups.json');
    return $policy;
}
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
    my $os = observation('uname','-s'); my $arch = observation('uname','-m');
    s/\n\z// for $os,$arch;
    return {schema=>1,environment=>\%environment,cargo_config=>\%configs,
        workspace_members=>workspace_members(),host=>{os=>$os,arch=>$arch,
            compiler=>observation(($ENV{RUSTC} || 'rustc'),'--version','--verbose')}};
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
sub reuse_policy {
    my ($source) = @_;
    my $path='ci/release-stage-reuse.json';
    my $raw=defined($source) ? blob($source,$path) : read_file($path);
    my $policy=$json->decode($raw);
    encode($policy) eq $raw && ($policy->{schema}//0)==1
        && join(',',sort keys %$policy) eq 'contract,hosts,schema,stages'
        && ($policy->{contract}//'') eq 'blob-final-candidate/1'
        && ref($policy->{hosts}) eq 'ARRAY' && @{$policy->{hosts}}
        && ref($policy->{stages}) eq 'ARRAY'
        or die "invalid reusable-stage policy\n";
    my $full=stage_policy($source); my %full=map {$_=>1} @{$full->{stages}}; my %seen;
    !grep {!$full{$_} || $seen{$_}++} @{$policy->{stages}}
        or die "unknown or duplicate reusable stage\n";
    return $policy;
}
sub active_plan {
    my ($stage)=@_;
    my ($source,$group)=@ENV{qw(RELEASE_SOURCE RELEASE_GROUP)};
    my $path=$ENV{BLOB_RELEASE_VALIDATION_PLAN}//die "missing active plan\n";
    $path =~ m{\Atarget/blob-release-validation\.[A-Za-z0-9]+/plan\.json\z}
        or die "invalid active plan path\n";
    git('rev-parse','HEAD') eq "$source\n" or die "active source changed\n";
    system('bash','scripts/ci/check-release-source.sh')==0 or die "active source is dirty\n";
    (my $context_path=$path) =~ s{/plan\.json\z}{/configuration.json};
    my $context=$json->decode(read_file($context_path));
    encode($context) eq encode(configuration()) or die "active configuration changed\n";
    my $plan=selection($source,$group,$context_path,$context);
    read_file($path) eq encode($plan) or die "active selection changed\n";
    grep {$_->{target} eq $stage && $_->{required}} @{$plan->{stages}}
        or die "stage not required by active plan\n";
    return $plan;
}
sub stage_records {
    my ($attempt,$source,$group,$plan)=@_;
    my $policy=reuse_policy($source);
    my %selected=map {$_=>1} @{$policy->{stages}};
    my @targets=map {$_->{target}} grep {$_->{required} && $selected{$_->{target}}} @{$plan->{stages}};
    my (%records,$candidate);
    for my $stage (@targets) {
        my $record=$json->decode(read_file("$attempt/stages/$stage.json"));
        if (($record->{kind}//'') eq 'blob-direct-stage-execution') {
            ($record->{schema}//0)==1 && $record->{source} eq $source && $record->{group} eq $group
                && $record->{stage} eq $stage && $record->{mode} eq 'executed'
                or die "mismatched direct stage execution\n";
        } elsif (($record->{kind}//'') eq 'sealed-cargo-stage-admission') {
            $candidate //= $json->decode(read_file("$attempt/candidate.json"));
            my $admission=$record->{admission};
            ($record->{schema}//0)==1 && $admission->{source} eq $source && $admission->{group} eq $group
                && $admission->{stage} eq $stage && $admission->{contract} eq $policy->{contract}
                && $admission->{mode} =~ /\A(?:executed|reused)\z/
                && $record->{candidate_identity} eq $candidate->{identity}
                or die "mismatched reusable stage admission\n";
            # Re-seal the producer-owned physical admission at capture; do not
            # trust a target-side JSON object to declare its own evidence roster.
            my $sealed=$json->decode(observation($^X,'scripts/ci/seal-validation-stage.pl',
                abs_path("$attempt/stages/$stage-admission.json")//die("missing stage admission\n"),
                $source,$group,$candidate->{identity}));
            encode($sealed) eq encode({map {$_=>$record->{$_}} keys %$sealed})
                or die "stage custody changed before capture\n";
            $stage ne 'test-pocketic' || $record->{fresh_identity}
                or die "missing actual mixed-version identity qualification\n";
            check_candidate_scope($record,$plan,$stage);
        } else { die "unknown stage execution evidence\n"; }
        $records{$stage}=$record;
    }
    return (\%records,$candidate);
}

sub package_scope {
    my ($source,$members) = @_;
    $members //= workspace_members();
    my $read = sub { defined($source) ? blob($source,$_[0]) : read_file($_[0]) };
    my $scope_bytes = $read->($packages_path);
    my $scope = $json->decode($scope_bytes);
    encode($scope) eq $scope_bytes && ($scope->{schema}//0)==1
        && join(',',sort keys %$scope) eq 'consumers,schema,stages'
        or die "invalid package-scope policy\n";
    my $groups = $json->decode($read->('ci/cargo-release-groups.json'));
    my (%owner,%paths);
    my %tracked = defined($source) ? map { $_=>1 } source_files($source) : ();
    for my $group (@{$groups->{groups}}) {
        for my $member (@{$group->{members}}) {
            !exists($owner{$member}) or die "duplicate package owner\n";
            my $path = $members->{$member}//die "missing Cargo member: $member\n";
            $path =~ m{\A[^\0\r\n]+/Cargo\.toml\z} && $path !~ m{(?:\A|/)\.{1,2}(?:/|\z)|//}
                && (!defined($source) || $tracked{$path})
                or die "member manifest absent from selected source: $member\n";
            $path =~ s{/Cargo\.toml\z}{};
            $owner{$member} = $group->{name}; $paths{$path} = $group->{name};
        }
    }
    join(',',sort keys %owner) eq join(',',sort keys %$members)
        or die "Cargo/release member rosters differ\n";
    join(',',sort keys %{$scope->{consumers}}) eq join(',',sort map {$_->{name}} @{$groups->{groups}})
        && join(',',sort keys %{$scope->{stages}}) eq 'check,clippy,docs-check,test-native,wasm-check'
        or die "incomplete group/package-stage mapping\n";
    for my $group (keys %{$scope->{consumers}}) {
        my %seen;
        ref($scope->{consumers}{$group}) eq 'ARRAY' && @{$scope->{consumers}{$group}}
            && !grep { !exists($scope->{consumers}{$_}) || $seen{$_}++ } @{$scope->{consumers}{$group}}
            or die "invalid reverse-consumer mapping\n";
        $seen{$group} or die "package scope omits its own group\n";
    }
    for my $stage (keys %{$scope->{stages}}) {
        my $roots = $scope->{stages}{$stage};
        next if !ref($roots) && $roots eq 'members';
        my %seen;
        ref($roots) eq 'ARRAY' && @$roots && !grep { !exists($owner{$_}) || $seen{$_}++ } @$roots
            or die "invalid full package roster: $stage\n";
    }
    return ($scope,\%owner,\%paths);
}
sub package_args {
    my ($source,$stage,$changed,$members) = @_;
    my ($policy,$owner) = package_scope($source,$members);
    my $roots = $policy->{stages}{$stage} // die "unknown package stage: $stage\n";
    return ['--workspace'] if !defined($changed) && !ref($roots);
    my %affected;
    if (defined $changed) {
        for my $group (keys %$changed) { $affected{$_}=1 for @{$policy->{consumers}{$group}}; }
    } else { $affected{$_}=1 for values %$owner; }
    my @packages = grep { $affected{$owner->{$_}} } ref($roots) ? @$roots : sort keys %$owner;
    @packages or die "package stage has no applicable roots\n";
    return [map { ('-p',$_) } @packages];
}
sub candidate_scope {
    my ($plan,$stage)=@_;
    return unless defined($plan) && $plan->{scope} eq 'group';
    my ($entry)=grep {$_->{target} eq $stage} @{$plan->{stages}};
    defined($entry) && $entry->{required} or die "candidate stage is not required\n";
    if ($stage eq 'test-pocketic') {
        $entry->{reason} =~ /; installed partitions: (test-pocketic-client-run(?: test-pocketic-engine-run)?)\z/
            or die "missing planned installed partitions\n";
        return {schema=>1,stage=>$stage,partitions=>[split / /,$1]};
    }
    return unless $stage =~ /\A(?:clippy|check|docs-check|test-native|wasm-check)\z/;
    $entry->{reason} =~ /; package roots: (.+)\z/ or die "missing planned package roots\n";
    return {schema=>1,stage=>$stage,packages=>[split / /,$1]};
}
sub read_candidate_scope {
    my ($stage)=@_;
    my $path=$ENV{CARGO_STAGE_SCOPE};
    return unless defined $path;
    my $identity=$ENV{CARGO_STAGE_IDENTITY}//'';
    $identity =~ /\A[0-9a-f]{64}\z/
        && getcwd() =~ m{\A(.+)/target/release-stage-cache/candidates/sources/\Q$identity\E\z}
        or die "leaf scope is restricted to the qualified candidate\n";
    my $owner=$1;
    $path =~ m{\A\Q$owner\E/target/release-stage-cache/scopes/([0-9a-f]{64})\.json\z}
        or die "invalid candidate scope route\n";
    my $digest=$1;
    (abs_path($path)//'') eq $path or die "redirected candidate scope\n";
    my $raw=read_file($path); my $scope=$json->decode($raw);
    sha256_hex($raw) eq $digest && encode($scope) eq $raw
        && ($scope->{schema}//0)==1 && ($scope->{stage}//'') eq $stage
        or die "candidate scope identity differs\n";
    if ($stage eq 'test-pocketic') {
        join(',',sort keys %$scope) eq 'partitions,schema,stage'
            && ref($scope->{partitions}) eq 'ARRAY'
            && join(' ',@{$scope->{partitions}}) =~ /\Atest-pocketic-client-run(?: test-pocketic-engine-run)?\z/
            or die "invalid candidate installed partitions\n";
    } else {
        $stage =~ /\A(?:clippy|check|docs-check|test-native|wasm-check)\z/
            && join(',',sort keys %$scope) eq 'packages,schema,stage'
            && ref($scope->{packages}) eq 'ARRAY' && @{$scope->{packages}}
            && @{$scope->{packages}} % 2 == 0 or die "invalid candidate package scope\n";
        my %seen;
        for (my $i=0;$i<@{$scope->{packages}};$i+=2) {
            $scope->{packages}[$i] eq '-p' && $scope->{packages}[$i+1] =~ /\A[a-z0-9][a-z0-9_-]*\z/
                && !$seen{$scope->{packages}[$i+1]}++ or die "invalid candidate package flags\n";
        }
    }
    return $scope;
}
sub check_candidate_scope {
    my ($entry,$plan,$stage)=@_;
    my $scope=candidate_scope($plan,$stage);
    my @selected=grep {/\ACARGO_STAGE_SCOPE=/} @{$entry->{command}};
    if (!defined $scope) { @selected==0 or die "unexpected scoped candidate command\n"; return; }
    $entry->{cwd} =~ m{\A(.+)/target/release-stage-cache/candidates/sources/[0-9a-f]{64}\z}
        or die "invalid scoped candidate directory\n";
    my $expected="CARGO_STAGE_SCOPE=$1/target/release-stage-cache/scopes/".sha256_hex(encode($scope)).'.json';
    @selected==1 && $selected[0] eq $expected or die "candidate command differs from planned leaf scope\n";
}
sub selection {
    my ($source, $group, $context_path, $context) = @_;
    $source =~ /\A[0-9a-f]{40,64}\z/ && $group =~ /\A(?:contracts|client|engine)\z/
        or die "select exact release source/group\n";
    my $policy = stage_policy($source);
    my $groups = $json->decode(blob($source,'ci/cargo-release-groups.json'));
    my (undef,undef,$owned_paths) = package_scope($source,$context->{workspace_members});
    my $scope = 'workspace';
    my $reason = 'Shared, unknown or not yet classified inputs require the full roster';
    my $client_only = 0;
    my $package_regression = 1;
    my %changed_groups;
    # Admit the whole delta since this group's annotated release, never only the
    # last commit. New helpers/policies, missing history or any unknown path keep
    # the full gate. The first delivery of this mapping is therefore full.
    eval {
        keys(%{$context->{environment}}) == 0 && keys(%{$context->{cargo_config}}) == 0
            or die "ambient build configuration requires full validation";
        my ($entry) = grep { $_->{name} eq $group } @{$groups->{groups}};
        my ($version) = blob($source,$context->{workspace_members}{$entry->{primary}}) =~ /^version = "([0-9.]+)"$/m;
        defined $version or die "no baseline version";
        my $tag = "$group-v$version";
        my $baseline = $json->decode(shared_observation('baseline',abs_path($ENV{BLOB_RELEASE_SOURCE_ROOT}//'.'),$source,"refs/tags/$tag"));
        $baseline->{available} or die "no admitted annotated baseline";
        my $base = $baseline->{baseline};
        my %baseline_files = map { $_=>1 } source_files($base);
        $baseline_files{"releases/$group/release.json"} or die "missing baseline receipt";
        my $receipt = $json->decode(blob($base,"releases/$group/release.json"));
        ($receipt->{gate}//'') eq 'release-verify-stages-v1'
            && ($receipt->{validation}{kind}//'') eq 'blob-release-stages-v1'
            && ($receipt->{validation}{group}//'') eq $group
            or die "baseline predates qualified plan receipts";
        encode($receipt->{validation}{policy}) eq encode($policy)
            or die "baseline predates the current stage roster";
        encode($receipt->{validation}{configuration}) eq encode($context)
            or die "baseline configuration differs";
        ($receipt->{validation}{evidence}{inputs}{$packages_path}//'') eq sha256_hex(blob($source,$packages_path))
            or die "baseline predates qualified package scopes";
        my @changed = @{$baseline->{changes}};
        @changed or die "no new release inputs";
        my $only_client = 1;
        for my $path (@changed) {
            next if $path =~ m{\Areleases/(?:contracts|client|engine)/CHANGELOG\.md\z};
            next if $path eq 'docs/status/current.md';
            my ($owner_path) = grep { index($path,"$_/src/")==0 && $path =~ /\.rs\z/ } keys %$owned_paths;
            defined($owner_path) or die "unclassified release input: $path";
            $changed_groups{$owned_paths->{$owner_path}} = 1;
            $only_client = 0 unless $path =~ m{\Acrates/ic-blob-storage-client/src/tenants/};
        }
        keys(%changed_groups) or die "no classified implementation change";
        $scope = 'group';
        $client_only = $group eq 'client' && $only_client;
        # The verifier is a standalone CLI binary. Conservatively include all
        # CLI implementation; wrappers, manifests/locks, tools, compiler, policy
        # and unknown inputs already force full validation above.
        $package_regression = scalar grep {m{\Acrates/ic-blob-storage-cli/src/}} @changed;
        $reason = "Complete source delta from $tag at $base leaves release infrastructure unchanged";
    };
    my @stages = map {
        my $skip = $scope eq 'group' && ($_ eq 'release-check' || ($_ eq 'package-regression' && !$package_regression));
        my $target = $_;
        my $package_reason = $scope eq 'group' && $target =~ /\A(?:clippy|check|docs-check|test-native|wasm-check)\z/
            ? '; package roots: '.join(' ',@{package_args($source,$target,\%changed_groups,$context->{workspace_members})}) : '';
        my $installed_reason = $target eq 'test-pocketic'
            ? '; installed partitions: test-pocketic-client-run'.($client_only ? '' : ' test-pocketic-engine-run') : '';
        {target=>$target, required=>$skip ? JSON::PP::false : JSON::PP::true,
            reason=>$skip ? ($_ eq 'release-check'
                ? 'Unchanged release machinery; real group renderer/rollback checks remain required'
                : 'Unchanged archive verifier/CLI and qualification contract; actual package verification remains required')
                : $reason.$package_reason.$installed_reason}
    } @{$policy->{stages}};
    return {schema=>1,source=>$source,group=>$group,scope=>$scope,policy=>$policy_path,
        inputs=>[sort(@{inputs($source)},$context_path)],stages=>\@stages};
}
sub verify {
    my ($record,$source,$group) = @_;
    ref($record) eq 'HASH' && ($record->{kind}//'') eq 'blob-release-stages-v1'
        && ($record->{schema}//0)==1 && ($record->{source}//'') eq $source && ($record->{group}//'') eq $group
        or die "missing or mismatched structured validation proof\n";
    my $path = $record->{plan_path}//'';
    $path =~ m{\Atarget/blob-release-validation\.[A-Za-z0-9]+/plan\.json\z} or die "invalid retained plan path\n";
    (my $context_path = $path) =~ s{/plan\.json\z}{/configuration.json};
    my $context = $record->{configuration};
    ref($context) eq 'HASH' && ($context->{schema}//0)==1
        && ref($context->{environment}) eq 'HASH' && ref($context->{cargo_config}) eq 'HASH'
        && ref($context->{workspace_members}) eq 'HASH'
        && ref($context->{host}) eq 'HASH'
        && join(',',sort keys %$context) eq 'cargo_config,environment,host,schema,workspace_members'
        or die "missing validation configuration\n";
    for my $hash (values %{$context->{environment}},values %{$context->{cargo_config}}) {
        defined($hash) && !ref($hash) && $hash =~ /\A[0-9a-f]{64}\z/ or die "invalid validation configuration digest\n";
    }
    encode($record->{plan}) eq encode(selection($source,$group,$context_path,$context)) or die "validation selection changed\n";
    encode($record->{policy}) eq blob($source,$policy_path) or die "validation policy changed\n";
    # Shared admits the physical run before this self-contained receipt exists.
    # Blob re-admits its source/configuration/obligations after preparation or in
    # a fresh publication checkout, where original absolute log paths cannot run.
    my $evidence = $record->{evidence};
    ref($evidence) eq 'HASH' && ($evidence->{schema}//0)==1
        && ($evidence->{kind}//'') eq 'group-validation-evidence'
        && ($evidence->{source}//'') eq $source && ($evidence->{group}//'') eq $group
        && ($evidence->{scope}//'') eq $record->{plan}{scope}
        && ($evidence->{plan_sha256}//'') eq sha256_hex(encode($record->{plan}))
        && ($evidence->{policy_sha256}//'') eq sha256_hex(encode($record->{policy}))
        && ($evidence->{context_sha256}//'') eq sha256_hex(encode($context))
        or die "missing or mismatched Shared execution evidence\n";
    (my $attempt = $path) =~ s{/plan\.json\z}{};
    ($evidence->{run}//'') =~ m{\A\Q$attempt\E/validation\.[A-Za-z0-9]+\z}
        or die "execution evidence belongs to another attempt\n";
    my @expected = sort ($path,@{$record->{plan}{inputs}});
    ref($evidence->{inputs}) eq 'HASH' && join("\0",sort keys %{$evidence->{inputs}}) eq join("\0",@expected)
        or die "incomplete validation input checkpoint\n";
    for my $input (@expected) {
        my $bytes = $input eq $path ? encode($record->{plan}) : $input eq $context_path ? encode($context) : blob($source,$input);
        ($evidence->{inputs}{$input}//'') eq sha256_hex($bytes) or die "validation input proof changed: $input\n";
    }
    !grep { !defined($_) || !/\S/ } @{$context->{host}}{qw(os arch compiler)}
        or die "missing validation host/compiler\n";
    my $stages = $evidence->{stages};
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
    my $reuse=reuse_policy($source);
    my %reusable=map {$_=>1} @{$reuse->{stages}};
    my @required=sort map {$_->{target}} grep {$_->{required} && $reusable{$_->{target}}} @{$record->{plan}{stages}};
    ref($record->{stage_admissions}) eq 'HASH'
        && join(',',sort keys %{$record->{stage_admissions}}) eq join(',',@required)
        or die "incomplete stage admission roster\n";
    for my $stage (@required) {
        my $entry=$record->{stage_admissions}{$stage};
        if (($entry->{kind}//'') eq 'blob-direct-stage-execution') {
            $entry->{source} eq $source && $entry->{group} eq $group && $entry->{stage} eq $stage
                && $entry->{mode} eq 'executed' or die "direct stage identity differs\n";
        } elsif (($entry->{kind}//'') eq 'sealed-cargo-stage-admission') {
            my $admitted=$entry->{admission};
            ($entry->{schema}//0)==1 && $admitted->{source} eq $source && $admitted->{group} eq $group
                && $admitted->{stage} eq $stage && $admitted->{contract} eq $reuse->{contract}
                && $admitted->{mode} =~ /\A(?:executed|reused)\z/
                && ($entry->{candidate_identity}//'') =~ /\A[0-9a-f]{64}\z/
                && $entry->{candidate_identity} eq $record->{candidate}{identity}
                && $record->{candidate}{source} eq $source
                or die "candidate stage identity differs\n";
            ref($entry->{files}) eq 'HASH' && keys(%{$entry->{files}})==4
                && ($admitted->{context_sha256}//'') =~ /\A[0-9a-f]{64}\z/
                && ($entry->{environment_sha256}//'') =~ /\A[0-9a-f]{64}\z/
                && ($entry->{files}{$admitted->{execution_proof}}//'') eq $admitted->{execution_proof_sha256}
                && ref($entry->{command}) eq 'ARRAY' && @{$entry->{command}}
                && $entry->{cwd} eq $record->{candidate}{directory}
                or die "incomplete sealed stage custody\n";
            for my $digest (values %{$entry->{files}}) {
                $digest =~ /\A[0-9a-f]{64}\z/ or die "invalid retained stage digest\n";
            }
            $stage ne 'test-pocketic' || $entry->{fresh_identity}
                or die "missing fresh version-sensitive integration\n";
            check_candidate_scope($entry,$record->{plan},$stage);
        } else { die "unknown sealed stage kind\n"; }
    }
    return 1;
}
sub main {
    my ($command,@args) = @ARGV;
    my ($source,$group) = @ENV{qw(RELEASE_SOURCE RELEASE_GROUP)};
    if ($command eq 'stages') {
        @args == 0 or die "unexpected stage-roster arguments\n";
        print join(' ',@{stage_policy(undef)->{stages}}),"\n";
    } elsif ($command eq 'pocketic-targets') {
        @args==0 or die "unexpected installed-partition arguments\n";
        my $targets='test-pocketic-client-run test-pocketic-engine-run';
        if (defined $ENV{BLOB_RELEASE_VALIDATION_PLAN}) {
            my $plan=active_plan('test-pocketic');
            my ($entry)=grep {$_->{target} eq 'test-pocketic'} @{$plan->{stages}};
            $entry->{reason} =~ /; installed partitions: (test-pocketic-client-run(?: test-pocketic-engine-run)?)\z/
                or die "missing planned installed partitions\n";
            $targets=$1;
        } elsif (my $scope=read_candidate_scope('test-pocketic')) { $targets=join(' ',@{$scope->{partitions}}); }
        print "$targets\n";
    } elsif ($command eq 'packages') {
        @args == 1 or die "expected package stage\n";
        my ($stage) = @args;
        my $path = $ENV{BLOB_RELEASE_VALIDATION_PLAN};
        my $flags;
        if (defined $path) {
            my $plan=active_plan($stage);
            my ($entry)=grep {$_->{target} eq $stage} @{$plan->{stages}};
            if ($plan->{scope} eq 'group') {
                $entry->{reason} =~ /; package roots: (.+)\z/ or die "missing planned package roots\n";
                my @flags = split / /,$1;
                @flags % 2 == 0 && !grep { $_ !~ /\A(?:-p|[a-z0-9_-]+)\z/ } @flags
                    or die "invalid planned package flags\n";
                $flags = \@flags;
            } else { $flags = package_args($source,$stage,undef,workspace_members()); }
        } elsif (my $scope=read_candidate_scope($stage)) { $flags=$scope->{packages}; }
        else { $flags = package_args(undef,$stage,undef); }
        print join(' ',@$flags),"\n";
    } elsif ($command eq 'plan') {
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
        @args == 1 or die "expected owned evidence directory\n";
        my @dirs = glob "$args[0]/validation.*";
        @dirs == 1 or die "expected one retained runner attempt\n";
        my $dir = $dirs[0];
        my $plan = $json->decode(read_file("$dir/parsed-inputs.json.plan.json"));
        my $context = $json->decode(read_file("$args[0]/configuration.json"));
        encode($context) eq encode(configuration()) or die "release configuration changed during validation\n";
        my $evidence = $json->decode(shared_observation('verify',abs_path('.'),
            "$args[0]/plan.json",$source,$group,$dir,"$args[0]/configuration.json"));
        my ($stage_admissions,$candidate)=stage_records($args[0],$source,$group,$plan);
        my $record = {schema=>1,kind=>'blob-release-stages-v1',stage_admissions=>$stage_admissions,candidate=>$candidate,source=>$source,group=>$group,plan=>$plan,
            configuration=>$context,
            plan_path=>"$args[0]/plan.json",policy=>$json->decode(read_file("$dir/parsed-inputs.json.policy.json")),
            evidence=>$evidence};
        verify($record,$source,$group);
        print encode($record);
    } else { die "expected stages, plan, packages, check or capture\n"; }
}
main() unless caller;
1;
