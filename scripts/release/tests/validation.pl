#!/usr/bin/env perl
# Exercise Blob's real selection/Make leaves; substitutes never contact a provider.
use strict;
use warnings;
use Test::More;
use JSON::PP;
use File::Temp qw(tempdir);
use File::Path qw(make_path);
use File::Copy qw(copy);
use File::Find;
use Cwd qw(getcwd);
use Time::HiRes qw(time);
my $root = getcwd();
make_path('target/validation-tests');
my $work = tempdir('caller.XXXXXX', DIR => "$root/target/validation-tests", CLEANUP => 0);
my $json = JSON::PP->new->canonical;
sub write_file {
    my ($path, $bytes) = @_;
    $path =~ m{\A(.+)/[^/]+\z} and make_path($1);
    open my $out, '>', $path or die "$path: $!";
    print {$out} $bytes or die "$path: $!";
    close $out or die "$path: $!";
}
sub read_json { open my $in, '<', $_[0] or die $!; return $json->decode(do {local $/; <$in>}); }
# Copy just the maintained caller and installed engine into isolated validation scratch.
for my $file ('Makefile', 'ci/release-validation.json') { make_path("$work/ci", "$work/scripts"); copy($file,"$work/$file") or die $!; }
for my $dir (qw(make scripts/release vendor/toko-tooling)) {
    find({no_chdir=>1,wanted=>sub {
        return if -d $_;
        -f $_ && !-l $_ or die "non-regular fixture input $_";
        my $dest = "$work/$_";
        $dest =~ m{\A(.+)/[^/]+\z}; make_path($1);
        copy($_,$dest) or die "copy $_: $!";
    }}, $dir);
}
chdir $work or die $!;
write_file('.gitignore', "/target/\n/.tools/\n");
write_file('Cargo.toml', "[workspace]\n[workspace.package]\nversion = \"0.103.3\"\n");
for my $file (qw(crates/ic-blob-storage/src/lib.rs crates/ic-blob-storage-cli/src/main.rs crates/ic-blob-storage-canister/src/lib.rs crates/ic-blob-storage-reference-app/src/state.rs clients/browser/src/index.js Cargo.lock)) {
    write_file($file, "fixture\n");
}
sub git { system('git', @_); $? == 0 or die "fixture Git failed"; }
git('init','-q'); git('config','user.email','fixture@example.invalid'); git('config','user.name','Blob qualification');
git('add','.'); git('commit','-qm','Fixture baseline'); git('tag','-a','v0.103.3','-m','qualified fixture baseline');
my $base = `git rev-parse HEAD`; chomp $base;
make_path('target/bin');
# Simulate common admission, simulator setup and metadata projection. Product recipes
# and artifact selection run through the actual Makefile, with recorded Cargo argv.
write_file('target/bin/make', <<'SH');
#!/usr/bin/env bash
set -euo pipefail
case " $* " in
  *' -f - '*) exec /usr/bin/make "$@" ;;
esac
for arg in "$@"; do
  case "$arg" in
    check-common|toko-tooling-check|qualification-tooling-check|prepare-test-server|test-release-metadata)
      if [[ "${!#}" == "$arg" ]]; then
        printf 'fixture %s\n' "$arg" >> "$FIXTURE_LOG"
        [[ "${FAIL_TARGET:-}" != "$arg" ]]
        exit
      fi ;;
  esac
done
exec /usr/bin/make -f Makefile -f target/admission.mk "$@" MAKE="$PWD/target/bin/make"
SH
# Overrides only setup/admission dependencies inside the actual artifact invocation.
write_file('target/admission.mk', <<'MAKE');
.PHONY: toko-tooling-check qualification-tooling-check prepare-test-server
toko-tooling-check qualification-tooling-check prepare-test-server:
	@:
MAKE
write_file('target/bin/cargo', <<'SH');
#!/usr/bin/env bash
set -euo pipefail
if [[ "$1" == --version ]]; then echo 'fixture Cargo'; exit; fi
printf 'cargo' >> "$FIXTURE_LOG"; printf '\t%s' "$@" >> "$FIXTURE_LOG"; printf '\n' >> "$FIXTURE_LOG"
package=''; verb="$1"; previous=''
for arg in "$@"; do [[ "$previous" != -p ]] || package="$arg"; previous="$arg"; done
if [[ "${DRIFT:-}" == 1 ]]; then echo drift >> crates/ic-blob-storage/src/lib.rs; fi
if [[ "${MOVE_BASELINE:-}" == 1 ]]; then git tag -f v0.103.3 >/dev/null; fi
if [[ "${FAIL_PACKAGE:-}" == "$package" && -n "$package" ]]; then exit 23; fi
if [[ "$verb" == build && " $* " == *' --message-format=json '* ]]; then
  case "$package" in
    ic-blob-storage-canister) name=ic_blob_storage_canister ;;
    ic-blob-storage-reference-app) name=ic_blob_storage_reference_app ;;
    ic-blob-storage-callback-probe) name=ic_blob_storage_callback_probe ;;
    ic-blob-storage-enrollment-probe) name=ic_blob_storage_enrollment_probe ;;
    ic-blob-storage-reply-probe) name=ic_blob_storage_reply_probe ;;
    ic-blob-storage-cli) name=blob ;;
    *) exit 2 ;;
  esac
  mkdir -p target/artifacts
  path="$PWD/target/artifacts/$name.wasm"; printf 'fixture' > "$path"
  jq -cn --arg name "$name" --arg path "$path" '{reason:"compiler-artifact",target:{name:$name},filenames:[$path],executable:$path}'
fi
SH
for my $tool (qw(rustc node npm)) { write_file("target/bin/$tool", "#!/bin/sh\nprintf 'fixture $tool\\n'\n"); }
chmod 0755, glob('target/bin/*');
local $ENV{PATH} = "$work/target/bin:$ENV{PATH}";
local $ENV{RELEASE_PREVIOUS} = '0.103.3';
local $ENV{FIXTURE_LOG} = "$work/target/commands.log";
{
    local $ENV{BLOB_VALIDATION_PLAN} = 'target/incomplete-plan.json';
    write_file($ENV{BLOB_VALIDATION_PLAN}, $json->encode({
        stages => [{target => 'installed-standalone', required => JSON::PP::true}],
    }) . "\n{");
    my $status = system('bash', '-c', 'bash scripts/release/prepare-installed.sh > target/incomplete-plan.log 2>&1');
    isnt($status, 0, 'truncated plan refuses installed preparation');
    open my $diagnostic, '<', 'target/incomplete-plan.log' or die $!;
    like(do { local $/; <$diagnostic> }, qr/parse error/i, 'preparation reports the JSON parse failure');
    ok(!-e $ENV{FIXTURE_LOG}, 'plan refusal precedes lint and artifact builds');
}
# GNU Make's inherited generated environment is maintained by actual Make children.
sub select_case {
    my ($path) = @_;
    git('checkout','-q','--detach',$base);
    write_file($path, "changed\n"); git('add',$path); git('commit','-qm','Selected change');
    $ENV{RELEASE_SOURCE} = `git rev-parse HEAD`; chomp $ENV{RELEASE_SOURCE};
    system('bash','-c','perl vendor/toko-tooling/scripts/ci/validation-evidence.pl baseline "$PWD" "$RELEASE_SOURCE" refs/tags/v0.103.3 > target/base.json'); $? == 0 or die 'baseline';
    write_file('target/context.json',"{}\n");
    system('bash','-c','perl scripts/release/select-validation.pl release target/base.json target/context.json > target/plan.json'); $? == 0 or die 'selector';
    return read_json('target/plan.json');
}
sub required { return [map {$_->{target}} grep {$_->{required}} @{$_[0]->{stages}}]; }
my %cases = (
    'crates/ic-blob-storage-cli/src/main.rs' => [qw(check-common check-cli prepare-installed installed-application installed-callbacks installed-management)],
    'clients/browser/src/index.js' => [qw(check-common check-browser prepare-installed installed-callbacks)],
    'crates/ic-blob-storage-reference-app/src/state.rs' => [qw(check-common check-app prepare-installed installed-application installed-callbacks)],
    'crates/ic-blob-storage-canister/src/lib.rs' => [qw(check-common check-host prepare-installed installed-standalone installed-capacity installed-endpoints installed-signed installed-application installed-enrollment installed-management)],
    'docs/guide.md' => ['check-common'],
);
for my $path (sort keys %cases) { is_deeply(required(select_case($path)),$cases{$path},"scope $path"); }
for my $path ('Cargo.lock','crates/ic-blob-storage/src/lib.rs','new-unknown-input') {
    my $selected = required(select_case($path));
    ok((grep {$_ eq 'installed-capacity'} @$selected) && (grep {$_ eq 'check-library'} @$selected),"full reverse-consumer scope $path");
}
for my $context ({host=>'Darwin arm64'}, {host=>'Linux x86_64',custom_cargo=>JSON::PP::true}) {
    select_case('crates/ic-blob-storage-cli/src/main.rs');
    write_file('target/context.json',$json->encode($context)."\n");
    system('bash','-c','perl scripts/release/select-validation.pl release target/base.json target/context.json > target/plan.json');
    ok((grep {$_ eq 'installed-capacity'} @{required(read_json('target/plan.json'))}), 'unqualified host/config broadens scope');
}
select_case('crates/ic-blob-storage-cli/src/main.rs');
write_file('target/base.json', $json->encode({available=>JSON::PP::false,reason=>'missing fixture baseline'})."\n");
system('bash','-c','perl scripts/release/select-validation.pl release target/base.json target/context.json > target/plan.json');
ok((grep {$_ eq 'test-release-metadata'} @{required(read_json('target/plan.json'))}), 'missing baseline selects complete fallback');
sub execute {
    unlink $ENV{FIXTURE_LOG};
    my $start = time;
    my $status = system('bash','-c','bash scripts/release/validate.sh release > target/run.log 2>&1');
    my $elapsed = time - $start;
    open my $in, '<', 'target/run.log' or die $!;
    my $log = do {local $/; <$in>};
    my ($plan) = $log =~ /^Validation plan: (.+)$/m;
    return ($status, $plan, $elapsed, $log);
}
select_case('crates/ic-blob-storage-cli/src/main.rs');
my ($status,$plan,$selected_seconds,$log) = execute();
is($status,0,'actual CLI Make dispatch through shared runner') or diag $log;
if ($status == 0) {
    open my $in, '<', $ENV{FIXTURE_LOG} or die $!;
    my $commands = do {local $/; <$in>};
    like($commands,qr/cargo\tclippy\t.*-p\tic-blob-storage-cli/,'CLI lint reaches package leaf');
    unlike($commands,qr/--test\t(?:standalone|capacity|signed|enrollment|endpoints)(?:\t|\n)/,'unrelated installed suites omitted');
    like($commands,qr/--test\tapplication/,'application reverse consumer dispatched');
    like($commands,qr/--test\tcallbacks/,'callback reverse consumer dispatched');
    like($commands,qr/--test\tmanagement/,'management reverse consumer dispatched');
    my $proof = $plan; $proof =~ s{plan.json$}{proof.json};
    ok(-s $proof,'shared evidence verified for exact caller run');
}
{
    local $ENV{FAIL_PACKAGE} = 'ic-blob-storage';
    ($status,undef,undef,$log) = execute();
    is($status,0,'excluded library failure cannot block CLI selection') or diag $log;
}
{
    local $ENV{FAIL_PACKAGE} = 'ic-blob-storage-canister';
    ($status,undef,undef,$log) = execute();
    isnt($status,0,'required Wasm build failure blocks installed consumers');
}
{
    local $ENV{FAIL_PACKAGE} = 'ic-blob-storage-cli';
    ($status,undef,undef,$log) = execute();
    isnt($status,0,'required CLI failure blocks gate');
}
my $good_plan;
($status,$good_plan,undef,$log) = execute();
is($status,0,'fresh recovery after failed phase succeeds without old proof') or diag $log;
if ($status == 0) {
    (my $proof_path = $good_plan) =~ s{plan.json$}{proof.json};
    (my $context_path = $good_plan) =~ s{plan.json$}{context.json};
    my $proof = read_json($proof_path);
    my @check = ('perl','vendor/toko-tooling/scripts/ci/validation-evidence.pl','check',$work,
        $good_plan,$ENV{RELEASE_SOURCE},'blob',$proof->{run},$context_path,$proof_path);
    my $attempt = 0;
    my $check = sub {
        my $pid = fork(); defined($pid) or die $!;
        if (!$pid) {
            open STDOUT, '>', 'target/evidence-check.'.++$attempt.'.log' or die $!;
            open STDERR, '>&', \*STDOUT or die $!;
            exec @check; die "exec shared check: $!";
        }
        ++$attempt;
        waitpid($pid,0); return $?;
    };
    is($check->(),0,'completed attempt re-admits its original evidence');
    my $stage = (grep {$_->{state} eq 'PASS'} @{$proof->{stages}})[0];
    my $log_path = "$proof->{run}/$stage->{log}";
    open my $out, '>>', $log_path or die $!; print {$out} "tampered\n"; close $out;
    isnt($check->(),0,'tampered retained log refuses recovery admission');
    rename $proof_path,"$proof_path.saved" or die $!;
    isnt($check->(),0,'missing proof cannot authorize recovery');
}
{
    local $ENV{DRIFT} = 1;
    ($status,undef,undef,$log) = execute();
    isnt($status,0,'source input drift blocks later stages');
    git('checkout','--','crates/ic-blob-storage/src/lib.rs');
}
{
    local $ENV{MOVE_BASELINE} = 1;
    ($status,undef,undef,$log) = execute();
    isnt($status,0,'moved baseline tag refuses final gate admission');
    git('tag','-d','v0.103.3'); git('tag','-a','v0.103.3',$base,'-m','qualified fixture baseline');
}
select_case('Cargo.lock');
my $full_seconds;
($status,undef,$full_seconds,$log) = execute();
is($status,0,'full fallback dispatches full product roster') or diag $log;
if ($status == 0) {
    open my $in, '<', $ENV{FIXTURE_LOG} or die $!;
    my $commands = do {local $/; <$in>};
    for my $suite (qw(standalone capacity endpoints signed application enrollment callbacks management)) {
        like($commands, qr/--test\t\Q$suite\E(?:\t|\n)/, "full dispatch retains $suite");
    }
    my @cli_lint = $commands =~ /^(cargo\tclippy[^\n]*-p\tic-blob-storage-cli[^\n]*)$/mg;
    is(scalar(@cli_lint),1,'full plan lints native CLI once before artifact build');
}
diag sprintf('Disposable simulated-workload timings: selected %.3fs; full %.3fs. No product speedup claim.', $selected_seconds,$full_seconds);
diag "Evidence retained: $work";
done_testing();
