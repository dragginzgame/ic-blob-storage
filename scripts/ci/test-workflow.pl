#!/usr/bin/env perl
# Execute Blob's actual action run bodies; hosted cache/upload effects stay outside this fixture.
use strict;
use warnings;
use Test::More;
use JSON::PP;
use File::Temp qw(tempdir);
use File::Path qw(make_path);
use Cwd qw(getcwd);
my $repository = getcwd();
local $ENV{PATH} = "$repository/.tools/host/bin:$ENV{PATH}";
make_path('target/ci-cleanup');
my $root = tempdir('wiring.XXXXXX', DIR => "$repository/target/ci-cleanup", CLEANUP => 0);
my $json = JSON::PP->new;
sub read_file { open my $f, '<', $_[0] or die "$_[0]: $!"; return do {local $/; <$f>} // ''; }
sub write_file { open my $f, '>', $_[0] or die "$_[0]: $!"; print {$f} $_[1] or die $!; close $f or die $!; }
sub observe {
    open my $f, '-|', @_ or die "start observation: $!";
    my $bytes = do {local $/; <$f>}; close $f or die "observation failed: @_";
    return $bytes;
}
sub yaml { return $json->decode(observe('yq', '-o=json', '.', $_[0])); }
my $action = yaml('.github/actions/retain-failure-logs/action.yml');
my $shared = yaml('vendor/toko-tooling/.github/actions/retain-failure-evidence/action.yml');
my $workflow = yaml('.github/workflows/ci.yml');
my %scripts;
for my $owner (['blob', $action], ['shared', $shared]) {
    my ($name, $definition) = @$owner;
    write_file("$root/$name.json", $json->encode($definition->{runs}{steps}));
    for my $step (@{$definition->{runs}{steps}}) {
        next unless exists $step->{run};
        my $id = $step->{id} // next;
        my $script = observe('bash', 'vendor/toko-tooling/scripts/ci/extract-workflow-step.sh', "$root/$name.json", $id);
        $scripts{"$name/$id"} = "$root/$name-$id.sh";
        write_file($scripts{"$name/$id"}, $script);
    }
}
sub execute {
    my ($name, $directory, $environment, @command) = @_;
    local %ENV = (%ENV, RUNNER_TEMP => $directory, GITHUB_OUTPUT => "$directory/$name.outputs", %$environment);
    my $pid = fork(); defined $pid or die $!;
    if (!$pid) {
        open STDOUT, '>', "$directory/$name.console" or die $!;
        open STDERR, '>&', \*STDOUT or die $!;
        exec @command; die "exec: $!";
    }
    waitpid($pid, 0);
    return $? & 127 ? 128 + ($? & 127) : $? >> 8;
}
sub outputs {
    my ($directory, $name) = @_;
    return {map {split /=/, $_, 2} split /\n/, read_file("$directory/$name.outputs")};
}
sub prepare {
    my ($directory, $environment) = @_;
    return execute('prepare', $directory, $environment, 'bash', '-e', '-o', 'pipefail', $scripts{'blob/prepare'});
}
sub select_archive {
    my ($directory) = @_;
    my $paths = outputs($directory, 'prepare');
    is(execute('archive', $directory, {
        EVIDENCE_PREPARED_ARCHIVE => $paths->{archive}, EVIDENCE_COLLECTION_LOG => $paths->{log},
    }, 'bash', '-e', '-o', 'pipefail', $scripts{'shared/archive'}), 0, 'shared prepared-file selection executes');
    return read_file("$directory/archive.outputs");
}
my $valid = "$root/early-setup"; make_path("$valid/.attempt");
write_file("$valid/.attempt/install.log", "underlying compiler failure\nAuthorization: Bearer fixture-sensitive-value\n-----BEGIN PRIVATE KEY-----\nfixture-key-material\n-----END PRIVATE KEY-----\n");
is(execute('original', $valid, {FIXTURE_TOKEN => 'fixture-sensitive-value'}, $^X, 'scripts/ci/run-step.pl', 'linux-setup', $^X, '-e', 'open my $f, "<", $ARGV[0] or die $!; print while <$f>; exit 37', "$valid/.attempt/install.log"), 37, 'actual early setup failure preserves its original status');
write_file("$valid/blob-ci-evidence/unselected.txt", 'unselected-sensitive-file');
is(prepare($valid, {}), 0, 'actual Blob selector archives sanitized command output');
my $paths = outputs($valid, 'prepare');
ok(-f $paths->{archive} && -s $paths->{archive} < 500000, 'bounded archive is published');
is(observe('tar', '-tzf', $paths->{archive}), "./linux-setup.txt\n", 'only the whitelisted sanitized diagnostic is archived');
my $diagnostic = observe('tar', '-xOzf', $paths->{archive}, './linux-setup.txt');
like($diagnostic, qr/underlying compiler failure.*original_status=37|original_status=37.*underlying compiler failure/s, 'archive preserves cause and original outcome');
unlike($diagnostic, qr/fixture-sensitive-value|fixture-key-material|unselected-sensitive-file/, 'archive excludes secrets and unrelated evidence');
my $selection = select_archive($valid);
like($selection, qr/collection-complete=true/, 'shared action admits complete prepared evidence');
like($selection, qr/\Q$paths->{archive}\E\n\Q$paths->{log}\E\n/, 'shared action selects exactly the archive and collection log');
my %report = (
    PREPARE_OUTCOME => 'success', UPLOAD_OUTCOME => 'success', ARTIFACT_ID => '123',
    GITHUB_SERVER_URL => 'https://github.example.invalid', GITHUB_REPOSITORY => 'fixture/blob',
    GITHUB_RUN_ID => '42', GITHUB_STEP_SUMMARY => "$valid/summary",
);
is(execute('report', $valid, \%report, 'bash', '-e', $scripts{'blob/report'}), 0, 'successful hosted-result fixture links the artifact');
like(read_file("$valid/summary"), qr{https://github.example.invalid/fixture/blob/actions/runs/42/artifacts/123}, 'summary points to the exact reported artifact');
is(execute('upload-failed', $valid, {%report, UPLOAD_OUTCOME => 'failure'}, 'bash', '-e', $scripts{'blob/report'}), 1, 'upload failure remains explicit');
is(execute('partial-upload', $valid, {%report, PREPARE_OUTCOME => 'failure', GITHUB_STEP_SUMMARY => "$valid/partial-summary"}, 'bash', '-e', $scripts{'blob/report'}), 1, 'partial diagnostic upload cannot claim complete retention');
like(read_file("$valid/partial-summary"), qr{artifacts/123.*collection/upload failed}s, 'an available diagnostic artifact stays linked even when collection was incomplete');
is(execute('missing-upload', $valid, {%report, ARTIFACT_ID => ''}, 'bash', '-e', $scripts{'blob/report'}), 1, 'missing upload result cannot report success');
for my $case (qw(missing redirected oversized archiver)) {
    my $directory = "$root/$case"; make_path($directory);
    if ($case ne 'missing') {
        make_path("$directory/blob-ci-evidence");
        write_file("$directory/source", 'private fixture source');
        if ($case eq 'redirected') { symlink "$directory/source", "$directory/blob-ci-evidence/linux-setup.txt" or die $!; }
        else { write_file("$directory/blob-ci-evidence/linux-setup.txt", $case eq 'oversized' ? 'x' x 67001 : 'sanitized diagnostic'); }
    }
    my %environment;
    if ($case eq 'archiver') {
        make_path("$directory/bin");
        write_file("$directory/bin/tar", "#!/bin/sh\nprintf 'partial archive'\nexit 23\n");
        chmod 0755, "$directory/bin/tar";
        $environment{PATH} = "$directory/bin:$ENV{PATH}";
    }
    isnt(prepare($directory, \%environment), 0, "$case evidence cannot report successful collection");
    my $failed = outputs($directory, 'prepare');
    ok(!-e $failed->{archive} && -s $failed->{log}, "$case retains the diagnostic without publishing an invalid archive");
    my $selected = select_archive($directory);
    like($selected, qr/collection-complete=false/, "$case remains incomplete at the shared boundary");
    unlike($selected, qr/\Q$failed->{archive}\E/, "$case uploads only the collection log");
    is(execute('incomplete', $directory, {}, 'bash', '-e', $scripts{'shared/completeness'}), 1, "$case shared completeness check fails after log retention");
    is(read_file("$directory/source"), 'private fixture source', 'redirected target is unchanged') if $case eq 'redirected';
    ok(-s "$failed->{log}" && -s ($failed->{archive} =~ s/evidence\.tar\.gz$/evidence.partial.tar.gz/r), 'failed archive attempt is preserved privately') if $case eq 'archiver';
}
my ($upload) = grep {($_->{id} // '') eq 'upload'} @{$action->{runs}{steps}};
is($upload->{uses}, './vendor/toko-tooling/.github/actions/retain-failure-evidence', 'Blob delegates upload to the installed shared action');
is($upload->{if}, 'always()', 'upload/log retention still runs after collection failure');
is($upload->{with}{'prepared-archive'}, '${{ steps.prepare.outputs.archive }}', 'shared upload receives the published archive output');
is($upload->{with}{'collection-log'}, '${{ steps.prepare.outputs.log }}', 'shared upload receives the collection diagnostic output');
ok(!exists $upload->{with}{'repository-root'} && !exists $upload->{with}{'temp-root'}, 'prepared path never selects raw tool stores');
my ($shared_upload) = grep {($_->{id} // '') eq 'upload'} @{$shared->{runs}{steps}};
is($shared_upload->{with}{'retention-days'}, 14, 'reported retention matches the shared uploader');
for my $job (sort keys %{$workflow->{jobs}}) {
    my $steps = $workflow->{jobs}{$job}{steps};
    my @admit = grep {($steps->[$_]{id} // '') eq 'admit'} 0..$#$steps;
    my @cache = grep {($steps->[$_]{uses} // '') eq './vendor/toko-tooling/.github/actions/cache-tool-downloads'} 0..$#$steps;
    is(scalar(@cache), 2, "$job restores and saves through one shared cache owner");
    ok(@admit == 1 && $admit[0] < $cache[0] && $cache[0] < $cache[1], "$job admits before cache and restores before save");
    my @setup = grep {($steps->[$_]{run} // '') =~ /run-step\.pl (?:workflow|linux|macos)-setup /} 0..$#$steps;
    ok(@setup == 1 && $cache[0] < $setup[0] && $setup[0] < $cache[1], "$job installs between cache restore and save");
    ok(!exists $steps->[$cache[1]]{if}, "$job saves only after successful setup");
    is($steps->[$cache[1]]{with}{mode}, 'save', "$job saves only through the shared guarded save branch");
    my $checks = join "\n", map {$_->{run} // ''} @$steps[$cache[1]+1..$#$steps];
    like($checks, qr/(?:make ci build-host build-reference-app|make test-references test-caffeine test-local test-cli test-example-local|gitleaks" git)/, "$job saves before its original validation coverage");
    ok(!grep({exists $_->{'continue-on-error'}} @$steps), "$job preserves setup/check failures");
}
my $linux = join "\n", map {$_->{run} // ''} @{$workflow->{jobs}{linux}{steps}};
unlike($linux, qr/make install-browser-tools/, 'browser installation remains with check-browser instead of duplicate CI setup');
diag "Local wiring evidence: $root; hosted cache/upload effects were not performed.";
done_testing();
