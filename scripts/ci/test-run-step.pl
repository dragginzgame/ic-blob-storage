#!/usr/bin/env perl
use strict;
use warnings;
use Test::More;
use File::Temp qw(tempdir);
use FindBin;

my $root = tempdir(CLEANUP => 1);
local $ENV{RUNNER_TEMP} = $root;
local $ENV{FIXTURE_TOKEN} = 'fixture-sensitive-value';
local $ENV{FIXTURE_ACCESS_KEY} = 'synthetic-access-value';
sub read_file {
    open my $file, '<', $_[0] or die "read fixture: $!";
    local $/;
    return <$file> // '';
}
sub run {
    my ($phase, $code, @args) = @_;
    my $pid = fork();
    defined $pid or die "fork fixture: $!";
    if (!$pid) {
        open STDOUT, '>', "$root/$phase.console" or die $!;
        open STDERR, '>&', STDOUT or die $!;
        exec $^X, "$FindBin::Bin/run-step.pl", $phase, $^X, '-e', $code, @args;
        die "exec fixture: $!";
    }
    waitpid($pid, 0);
    return $? >> 8;
}
mkdir "$root/.attempt" or die $!;
open my $attempt, '>', "$root/.attempt/install.log" or die $!;
print {$attempt} "compiler failed: missing fixture dependency\n",
    "Authorization: Bearer fixture-sensitive-value\n",
    "-----BEGIN PRIVATE KEY-----\nfixture-key-material\n-----END PRIVATE KEY-----\n",
    "PATH=/fixture/environment\nhttps://example.invalid/?secret=fixture-sensitive-value\n",
    "request failed with Bearer synthetic-bearer-value\n",
    "embedded=prefix-synthetic-access-value-suffix\n",
    qq({"api_key": "synthetic-labelled-value"}\n),
    ('x' x 9000), "-----BEGIN PRIVATE KEY-----\nsynthetic-long-line-key\n-----END PRIVATE KEY-----\n",
    "final harmless diagnostic\n";
close $attempt;
# As the Toko Rust installer does, print the explicitly owned attempt log on
# compilation failure. No tree scan or symlink traversal is needed to retain it.
is(run('early-setup', 'open my $f, "<", $ARGV[0] or die $!; print while <$f>; exit 37',
    "$root/.attempt/install.log"), 37, 'early setup retains original failure');
my $saved = read_file("$root/blob-ci-evidence/early-setup.txt");
like($saved, qr/compiler failed: missing fixture dependency/, 'underlying attempt diagnostic retained');
like($saved, qr/phase=early-setup .*original_status=37/, 'phase and original status retained');
for my $bytes ($saved, read_file("$root/early-setup.console")) {
    unlike($bytes, qr/fixture-sensitive-value|fixture-key-material|\/fixture\/environment/,
        'artifact and console exclude credentials, keys and environment');
    unlike($bytes, qr/synthetic-(?:bearer-value|access-value|labelled-value|long-line-key)/,
        'both outputs redact bearer, access-key and continued private-key material');
    like($bytes, qr/final harmless diagnostic/, 'ordinary diagnostics resume after a private key');
}
is(run('empty', 'exit 19'), 19, 'missing diagnostic still fails');
like(read_file("$root/blob-ci-evidence/empty.txt"), qr/No command diagnostics/, 'missing diagnostics explicit');
is(run('large', 'print "context\n" x 50000; print "x" x 50000; print "\nfinal diagnostic\n"; exit 23'),
    23, 'oversized output preserves failure');
my $large = read_file("$root/blob-ci-evidence/large.txt");
ok(length($large) < 67000, 'artifact has an explicit size bound');
like($large, qr/Output truncated.*final diagnostic/s, 'truncation explicit and final diagnostic survives');
ok(-s "$root/large.console" < 250000, 'console output bounded');
is(run('success', 'print "ok\n"'), 0, 'successful command stays successful');
ok(!-e "$root/blob-ci-evidence/success.txt", 'no success artifact');
symlink "$root/.attempt/install.log", "$root/blob-ci-evidence/refused.txt" or die $!;
my $original = read_file("$root/.attempt/install.log");
is(run('refused', 'print "failure\n"; exit 29'), 29, 'collector refusal cannot mask original failure');
is(read_file("$root/.attempt/install.log"), $original, 'collector never follows output symlink');
like(read_file("$root/refused.console"), qr/Evidence collection failed/, 'collection failure explicit');
{
    local $ENV{RUNNER_TEMP};
    delete $ENV{RUNNER_TEMP};
    is(run('missing-root', 'exit 31'), 31, 'missing evidence root preserves original failure');
}
my @artifacts = glob "$root/blob-ci-evidence/*";
is(scalar @artifacts, 4, 'only selected diagnostic files and refused fixture link present');
done_testing();
