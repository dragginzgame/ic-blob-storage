#!/usr/bin/env perl
use strict;
use warnings;
use File::Path qw(make_path);
use Fcntl qw(O_WRONLY O_CREAT O_EXCL);

# Capture only this command's output, including attempt logs printed by Toko's
# installers. Never crawl retained tool stores, homes or provider custody.
my ($phase, @command) = @ARGV;
defined($phase) && $phase =~ /\A[a-z][a-z0-9-]*\z/ && @command
    or die "usage: run-step.pl PHASE COMMAND [ARG...]\n";
my @secrets = grep { length($_) >= 4 } map { $ENV{$_} }
    grep { /token|secret|password|credential|private.?key|(?:access|api)[_-]?key/i } keys %ENV;
my ($tail, $pending, $pem, $omitted, $printed) = ('', '', 0, 0, 0);
sub diagnostic {
    my ($line) = @_;
    $line =~ s/\e\[[0-?]*[ -\/]*[\@-~]//g;
    $line =~ s/[^\x09\x20-\x7e]/?/g;
    if ($line =~ /-----BEGIN .*PRIVATE KEY-----/) { $pem = 1 }
    if ($pem) {
        $pem = 0 if $line =~ /-----END .*PRIVATE KEY-----/;
        return '[private key material omitted]';
    }
    return '[sensitive field or environment assignment omitted]'
        if $line =~ /(?:authorization|cookie|token|secret|password|credential|private.?key|(?:access|api)[ _-]?key|signature|envelope|payload)\s*["']?\s*[:=]/i
        || $line =~ /\bBearer\s+\S+/i
        || $line =~ /^\s*(?:export\s+)?[A-Z_][A-Z0-9_]*=/;
    for my $secret (@secrets) { $line =~ s/\Q$secret\E/[redacted]/g }
    $line =~ s{https?://\S+}{[URL omitted]}g;
    $line =~ s/\b(?:gh[pousr]_[A-Za-z0-9_]+|github_pat_[A-Za-z0-9_]+)\b/[token omitted]/g;
    # Avoid interpreting child output as GitHub workflow commands.
    $line =~ s/^::/[child output] ::/;
    return $line;
}
sub retain {
    my ($line) = @_;
    my $safe = diagnostic($line)."\n";
    if ($printed + length($safe) <= 65536) { print "[$phase] $safe" }
    elsif ($printed <= 65536) { print "[$phase] Further console output suppressed; retaining bounded tail.\n" }
    $printed += length($safe);
    $tail .= $safe;
    if (length($tail) > 65536) {
        substr($tail, 0, length($tail) - 65536, '');
        $omitted = 1;
    }
}
$| = 1;
my $pid = open my $child, '-|';
defined($pid) or die "cannot start CI command: $!\n";
if (!$pid) {
    open STDERR, '>&', STDOUT or die "cannot capture stderr\n";
    exec { $command[0] } @command or die "cannot execute CI command: $!\n";
}
my $discard = 0;
while (read($child, my $chunk, 4096)) {
    $pending .= $chunk;
    while ($pending =~ s/\A([^\n]*)\n//) {
        my $line = $1;
        if ($discard || length($line) > 4096) {
            # An omitted opening delimiter still governs subsequent lines.
            $pem = 1 if $line =~ /-----BEGIN /;
        }
        if (!$discard) {
            if (length($line) <= 4096) { retain($line) }
            else { retain('[line exceeding 4096 bytes omitted]'); $omitted = 1 }
        }
        $discard = 0;
    }
    if (length($pending) > 4096) {
        retain('[line exceeding 4096 bytes omitted]') unless $discard;
        $pem = 1 if $pending =~ /-----BEGIN /;
        # Keep enough suffix to recognize a delimiter split across reads.
        $pending = substr($pending, -16); $discard = 1; $omitted = 1;
    }
}
retain($pending) if length($pending) && !$discard;
close $child;
my $status = $? & 127 ? 128 + ($? & 127) : $? >> 8;
exit 0 unless $status;
my $root = $ENV{RUNNER_TEMP};
if (!defined($root)) {
    print STDERR "RUNNER_TEMP missing; evidence unavailable; original command status=$status\n";
    exit $status;
}
my $path = "$root/blob-ci-evidence/$phase.txt";
my $summary = "phase=$phase tool=$command[0] original_status=$status\n".
    "Only sanitized command output retained; raw files, environment and stores omitted.\n".
    ($omitted ? "Output truncated or oversized lines omitted.\n" : '').
    (length($tail) ? '' : "No command diagnostics were emitted.\n");
print STDERR $summary;
if ($printed > 65536) {
    print STDERR "Failure excerpt (last 16 KiB):\n";
    print STDERR "[$phase] $_\n" for split /\n/, substr($tail, -16384);
}
eval {
    -d $root && !-l $root or die "runner temp is unavailable or redirected\n";
    !-l "$root/blob-ci-evidence" or die "evidence directory is redirected\n";
    make_path("$root/blob-ci-evidence", {mode => 0700});
    sysopen my $file, $path, O_WRONLY | O_CREAT | O_EXCL, 0600 or die "cannot create evidence: $!\n";
    print {$file} $summary, $tail or die "cannot write evidence: $!\n";
    close $file or die "cannot close evidence: $!\n";
    print STDERR "Sanitized failure evidence: $path\n";
    1;
} or print STDERR "Evidence collection failed; original status preserved: $@";
exit $status;
