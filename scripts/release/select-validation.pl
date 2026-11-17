#!/usr/bin/env perl
# Blob impact policy only; Shared observes Git and admits/checkpoints the plan.
use strict;
use warnings;
use JSON::PP;
my ($mode, $baseline_path, $context_path) = @ARGV;
@ARGV == 3 && $mode =~ /\A(?:ci|release)\z/ or die "usage: select-validation.pl ci|release BASELINE CONTEXT\n";
my $json = JSON::PP->new->canonical;
sub read_json {
    open my $in, '<', $_[0] or die "read $_[0]: $!\n";
    my $value = $json->decode(do { local $/; <$in> });
    close $in or die "close $_[0]: $!\n";
    return $value;
}
my $policy = read_json('ci/release-validation.json');
my $baseline = read_json($baseline_path);
my $source = $ENV{RELEASE_SOURCE} // '';
$source =~ /\A[0-9a-f]{40,64}\z/ or die "admitted RELEASE_SOURCE required\n";
my (%required, %reason);
sub require_stages {
    my ($why, @stages) = @_;
    for (@stages) { $required{$_} = 1; $reason{$_} = $why; }
}
my @native = grep { /^check-/ && $_ ne 'check-common' } @{$policy->{stages}};
my @installed = grep { /^installed-/ } @{$policy->{stages}};
sub full {
    require_stages($_[0], grep { $_ ne 'test-release-metadata' } @{$policy->{stages}});
}
require_stages('Fresh tooling, formatting, declaration and documentation admission', 'check-common');
if ($mode eq 'ci') {
    require_stages('Repository CI retains the complete native roster', @native);
} elsif (!$baseline->{available}) {
    full("Full fallback: $baseline->{reason}");
    require_stages('No admitted baseline for metadata wiring', 'test-release-metadata');
} else {
    full('Full fallback: newly declared stage has no narrow impact mapping')
        if grep { !/\A(?:check-(?:common|library|cli|host|app|qualification|browser)|test-release-metadata|prepare-installed|installed-(?:standalone|capacity|endpoints|signed|application|enrollment|callbacks|management))\z/ } @{$policy->{stages}};
    my $context = read_json($context_path);
    full('Full fallback: host or Cargo configuration is outside the narrow Linux mapping')
        if ($context->{host} // 'Linux x86_64') ne 'Linux x86_64' || $context->{custom_cargo};
    for my $path (@{$baseline->{changes}}) {
        if ($path =~ m{\A(?:docs/.*\.md\z|(?:README|CHANGELOG|LICENSE|CONTRIBUTING)\.md\z|clients/browser/README\.md\z)}) {
            next; # Prose and links; no executable doc inputs in this repository.
        } elsif ($path eq 'crates/ic-blob-storage-cli/src/mainnet-root.der') {
            full('Changed shared trust fixture/public network binding');
        } elsif ($path =~ m{\Acrates/ic-blob-storage-cli/src/}) {
            require_stages('CLI implementation and its installed consumers', qw(check-cli installed-application installed-callbacks installed-management));
        } elsif ($path =~ m{\Aclients/browser/(?:src/|tests/|package(?:-lock)?\.json\z)}) {
            require_stages('Transfer/proof/custody and real catalog consumers', qw(check-browser installed-callbacks));
        } elsif ($path =~ m{\Acrates/ic-blob-storage-canister/src/}) {
            require_stages('Standalone Wasm and its installed reverse consumers', qw(check-host installed-standalone installed-capacity installed-endpoints installed-signed installed-application installed-enrollment installed-management));
        } elsif ($path =~ m{\Acrates/ic-blob-storage-reference-app/src/}) {
            require_stages('Reference catalog and its installed consumers', qw(check-app installed-application installed-callbacks));
        } elsif ($path =~ m{\Aqualification/ic-blob-storage-tests/tests/(standalone|capacity|endpoints|signed|application|enrollment)\.rs\z}) {
            require_stages('Changed installed test contract', "installed-$1");
        } elsif ($path =~ m{\Aqualification/ic-blob-storage-tests/tests/(callbacks|management)/}) {
            require_stages('Changed installed test contract', "installed-$1");
        } elsif ($path =~ m{\A(?:scripts/release/|make/validation\.mk\z|ci/release-validation\.json\z|Makefile\z|vendor/|\.toko-tooling.snapshot\z)}) {
            full("Full fallback: validation/tooling input $path changed");
            require_stages('Changed consumer metadata/tooling wiring', 'test-release-metadata');
        } else {
            # Library/protocol, locks, compiler, manifests, fixtures, shared helpers,
            # CI configuration and unknown paths can change any reverse consumer.
            full("Full fallback: shared or unclassified input $path");
        }
    }
}
require_stages('Prepare exactly the selected installed artifacts and test lint', 'prepare-installed')
    if grep { $required{$_} } @installed;
# Every ordinary source file is checkpointed. No cross-run cache or version normalization.
open my $files, '-|', 'git', 'ls-files', '--cached', '--others', '--exclude-standard', '-z' or die "inventory: $!\n";
my @files = split /\0/, do { local $/; <$files> };
close $files or die "inventory failed\n";
my %inputs = map { $_ => 1 } (@files, $baseline_path, $context_path);
my @stages = map { {target => $_, required => $required{$_} ? JSON::PP::true : JSON::PP::false,
    reason => $reason{$_} // 'No affected consumer in the admitted change; see selection policy'} } @{$policy->{stages}};
print $json->encode({schema => 1, source => $source, group => 'blob', scope => 'group',
    policy => 'ci/release-validation.json', inputs => [sort keys %inputs], stages => \@stages}), "\n";
