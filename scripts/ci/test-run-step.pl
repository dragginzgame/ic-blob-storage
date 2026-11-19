#!/usr/bin/env perl
use strict;
use warnings;
use Test::More;
use File::Copy qw(copy);
use File::Temp qw(tempdir);
use FindBin;

my $root = tempdir(CLEANUP => 1);
my $wrapper = "$FindBin::Bin/run-step.pl";
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
        exec $^X, $wrapper, $phase, $^X, '-e', $code, @args;
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
ok(-s "$root/large.console" < 90000, 'console prefix and final excerpt stay bounded including phase labels');
cmp_ok(scalar(() = $large =~ /^context$/mg), '<=', 99, 'failure artifact uses the shared excerpt line budget');
like($large, qr/earlier output or oversized lines omitted/, 'shared formatter marks the evicted lines');
is(run('success', q{
    $| = 1;
    print STDERR "Authorization: Bearer fixture-sensitive-value\n";
    print "short progress line\n";
    for (1..100) {
        open my $console, '<', $ARGV[0] or die $!;
        exit 0 if grep { /short progress line/ } <$console>;
        select undef, undef, undef, 0.05;
    }
    exit 42;
}, "$root/success.console"), 0, 'short sanitized progress reaches the console before the child exits');
unlike(read_file("$root/success.console"), qr/fixture-sensitive-value/, 'successful stderr is also sanitized');
ok(!-e "$root/blob-ci-evidence/success.txt", 'no success artifact');
symlink "$root/.attempt/install.log", "$root/blob-ci-evidence/refused.txt" or die $!;
my $original = read_file("$root/.attempt/install.log");
is(run('refused', 'print "failure\n"; exit 29'), 29, 'collector refusal cannot mask original failure');
is(read_file("$root/.attempt/install.log"), $original, 'collector never follows output symlink');
like(read_file("$root/refused.console"), qr/Evidence collection failed/, 'collection failure explicit');
is(run('early-setup', 'print "later failure\n"; exit 43'), 43, 'existing phase evidence preserves the later failure status');
is(read_file("$root/blob-ci-evidence/early-setup.txt"), $saved, 'exclusive creation preserves the original phase file');
{
    my $redirected = "$root/redirected";
    mkdir $redirected or die $!;
    symlink "$root/.attempt", "$redirected/blob-ci-evidence" or die $!;
    local $ENV{RUNNER_TEMP} = $redirected;
    is(run('directory', 'exit 41'), 41, 'redirected evidence directory preserves child status');
    ok(!-e "$root/.attempt/directory.txt", 'redirected directory receives no evidence');
}
{
    local $ENV{RUNNER_TEMP};
    delete $ENV{RUNNER_TEMP};
    is(run('missing-root', 'exit 31'), 31, 'missing evidence root preserves original failure');
}
my @artifacts = glob "$root/blob-ci-evidence/*";
is(scalar @artifacts, 4, 'only selected diagnostic files and refused fixture link present');
# Simulate an incomplete checkout. The snapshot verifier must be able to report
# its own failure before any code from the selected diagnostics module is loaded.
mkdir "$root/checkout" or die $!;
$wrapper = "$root/checkout/run-step.pl";
copy("$FindBin::Bin/run-step.pl", $wrapper) or die $!;
is(run('admit', 'print "snapshot entry missing\n"; exit 17'), 17, 'bootstrap preserves verifier failure without the helper');
like(read_file("$root/blob-ci-evidence/admit.txt"), qr/snapshot entry missing/, 'bootstrap retains the verifier diagnostic');
done_testing();
