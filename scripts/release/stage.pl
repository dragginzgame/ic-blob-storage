#!/usr/bin/env perl
# Shared companions: scripts/ci/prepare-cargo-stage-source.pl scripts/ci/run-validation-stage.pl scripts/ci/observe-cargo-stage.pl scripts/ci/seal-validation-stage.pl scripts/ci/check-cargo-stage-closure.pl
use strict;
use warnings;
use Cwd qw(abs_path getcwd);
use Digest::SHA qw(sha256_hex);
use File::Path qw(make_path);
use JSON::PP;
use Time::HiRes qw(clock_gettime CLOCK_MONOTONIC);
require './scripts/release/validation.pl';
my $json=JSON::PP->new->canonical->utf8;
my $started=clock_gettime(CLOCK_MONOTONIC);
my ($stage,@args)=@ARGV;
my $publication=$stage eq '--publication-clippy';
my ($primary,$publication_run)=@args if $publication;
$stage='clippy' if $publication;
$stage =~ /\A[a-z][a-z0-9-]*\z/ or die "invalid Blob stage\n";
my $policy=BlobValidation::reuse_policy();
my $enabled=grep {$_ eq $stage} @{$policy->{stages}};
my $root=getcwd();
my $prepared=abs_path($ENV{BLOB_PREPARED_TOOLS_ROOT}//$root)//die "missing prepared tools root\n";
my @search_path=split /:/,$ENV{PATH},-1;
my $qualified_path=!grep {!length($_) || $_ !~ m{\A/}} @search_path;
sub observation { BlobValidation::observation(@_) }
sub document { $json->decode(BlobValidation::read_file($_[0])) }
sub write_record {
    my ($path,$value)=@_;
    !-e $path && !-l $path or die "occupied stage record: $path\n";
    BlobValidation::write_file($path,$json->encode($value)."\n");
}
sub run {
    system @_;
    if ($?) { exit($? & 127 ? 128+($? & 127) : $? >> 8); }
}
sub safe_directory {
    my ($directory)=@_;
    $directory =~ m{\Atarget/[A-Za-z0-9_./-]+\z} && $directory !~ m{(?:\A|/)\.{1,2}(?:/|\z)|//}
        or die "invalid stage directory\n";
    my $cursor='';
    for my $part (split m{/},$directory) {
        $cursor.="/$part"; !-l "$root$cursor" or die "redirected stage directory\n";
    }
    make_path($directory);
}
my ($source,$group,$plan,$destination);
if (!$publication && !defined($ENV{BLOB_RELEASE_VALIDATION_PLAN})) {
    exec 'make','--no-print-directory',"$stage-run"; die $!;
}
if (!$enabled) {
    BlobValidation::active_plan($stage) unless $publication;
    exec 'make','--no-print-directory',"$stage-run"; die $!;
}
if ($publication) {
    @args==2 && defined($publication_run) or die "expected primary and publication evidence\n";
    $group=$ENV{RELEASE_GROUP}//'';
    $source=observation('git','rev-parse','HEAD'); $source =~ s/\n\z//;
    run('bash','scripts/release/release.sh','tag-check');
    my $selected=document('ci/cargo-release-groups.json');
    my @selected=grep {$_->{name} eq $group && $_->{primary} eq $primary} @{$selected->{groups}};
    @selected==1 or die "publication primary is outside selected group\n";
    $publication_run =~ s{\A\Q$root\E/}{};
    $destination="$publication_run/stages";
} else {
    @args==0 or die "unexpected stage arguments\n";
    ($source,$group)=@ENV{qw(RELEASE_SOURCE RELEASE_GROUP)};
    $plan=BlobValidation::active_plan($stage);
    $destination=$ENV{BLOB_RELEASE_STAGE_ADMISSIONS}//die "missing current group stage custody\n";
}
safe_directory($destination);
my $temporary=$ENV{TMPDIR}//"$root/target/tmp";
$temporary=abs_path($temporary)//$temporary if $temporary =~ m{\A/} && -d $temporary;
my $configuration=BlobValidation::configuration();
my $host="$configuration->{host}{os}/$configuration->{host}{arch}";
my $eligible=(!defined($plan) || $plan->{scope} eq 'workspace')
    && !keys(%{$configuration->{environment}}) && !keys(%{$configuration->{cargo_config}})
    && grep {$_ eq $host} @{$policy->{hosts}};
$eligible=0 unless $qualified_path;
# PocketIC embeds Unix socket paths below its temporary root (SUN_LEN).
$eligible=0 if $temporary !~ m{\A/} || length($temporary)>60
    || (defined($ENV{TMPDIR}) && !-d $temporary);
$eligible=0 if ($ENV{MAKEFLAGS}//'') =~ /--jobserver|(?:\A|\s)-?j(?:\d|\s|\z)/;
$eligible=0 if defined($ENV{CARGO_TARGET_DIR}) && $ENV{CARGO_TARGET_DIR} ne "$root/target";
# Explicit simulator selections are not silently converted to managed defaults.
$eligible=0 if defined $ENV{POCKET_IC_BIN};
if (!$eligible) {
    run('make','--no-print-directory',"$stage-run");
    write_record("$destination/$stage.json",{schema=>1,kind=>'blob-direct-stage-execution',
        source=>$source,group=>$group,stage=>$stage,mode=>'executed',
        reason=>'No qualified candidate projection for this scope, host or build configuration'});
    exit;
}
if ($publication) {
    # Consume the completed historical qualification. This is not a new test
    # execution on today's host: source/tag, compiler and archives remain fresh.
    my $admitted=eval {
        my $receipt=document("releases/$group/release.json");
        my $validation=$receipt->{validation};
        my $qualified=$validation->{stage_admissions}{clippy};
        ($qualified->{kind}//'') eq 'sealed-cargo-stage-admission'
            or die "release has no retained candidate Clippy qualification\n";
        my $projection=$validation->{candidate};
        $projection->{root} eq $root && $projection->{source} eq $receipt->{source}
            && $projection->{directory} =~ m{\A\Q$root\E/target/release-stage-cache/candidates/sources/[0-9a-f]{64}\z}
            or die "candidate is unavailable in this checkout\n";
        my $projection_path="$root/$destination/qualified-candidate.json";
        write_record($projection_path,$projection);
        observation($^X,'scripts/ci/prepare-cargo-stage-source.pl','--verify-projection',$projection_path);
        my $admission_path=(grep {/\/clippy-admission\.json\z/} keys %{$qualified->{files}})[0]
            //die "missing original Clippy admission\n";
        my $sealed=$json->decode(observation($^X,'scripts/ci/seal-validation-stage.pl',
            $admission_path,$receipt->{source},$group,$projection->{identity}));
        $json->encode($sealed) eq $json->encode({map {$_=>$qualified->{$_}} keys %$sealed})
            or die "retained Clippy custody differs from completed release\n";
        observation($^X,'scripts/ci/check-cargo-stage-closure.pl',$root,$projection->{directory},$primary);
        write_record("$destination/clippy.json",{schema=>1,kind=>'blob-publication-stage-qualification',
            source=>$source,group=>$group,primary=>$primary,validated_source=>$receipt->{source},
            qualification=>$sealed,elapsed_seconds=>clock_gettime(CLOCK_MONOTONIC)-$started});
        1;
    };
    if (!$admitted) {
        print STDERR "Candidate lint unavailable; running current-source Clippy: $@";
        run('make','--no-print-directory','clippy-run');
        write_record("$destination/clippy.json",{schema=>1,kind=>'blob-direct-stage-execution',
            source=>$source,group=>$group,stage=>'clippy',mode=>'executed',
            reason=>'No admitted retained publication qualification'});
    } else { print "Publication Clippy: retained final-candidate qualification admitted\n"; }
    exit;
}
make_path($temporary) unless -d $temporary;
$temporary=abs_path($temporary)//die "missing physical stage temporary root\n";
# Nested Make adds duplicate and nonexistent managed selectors. In this bounded
# absolute PATH, dropping them preserves lookup and unifies both entrypoints.
my %path_seen;
$ENV{PATH}=join(':',grep {-d $_ && !$path_seen{$_}++} @search_path);
my $parent=$destination; $parent =~ s{/stages\z}{};
my $projection_path="$parent/candidate.json";
my $projection;
if (-e $projection_path) { $projection=document($projection_path); }
else {
    my $editor=observation('make','--no-print-directory','--silent','release-editor-check'); $editor =~ s/\n\z//;
    my $raw=observation($^X,'scripts/ci/prepare-cargo-stage-source.pl','--root',$root,
        '--state','target/release-stage-cache/candidates','--source',$source,'--editor',$editor);
    $projection=$json->decode($raw);
    write_record($projection_path,$projection);
}
($projection->{kind}//'') eq 'cargo-final-candidate-source' && $projection->{source} eq $source
    && $projection->{root} eq $root or die "candidate belongs to another admitted source\n";
my $candidate=$projection->{directory};
$candidate =~ m{\A\Q$root\E/target/release-stage-cache/candidates/sources/[0-9a-f]{64}\z}
    or die "candidate escaped its owned source cache\n";
my $tools_path="$parent/tools.json";
my $tools;
if (-e $tools_path) { $tools=document($tools_path); }
else {
    my %files;
    for my $name (qw(bash cargo cc c++ ld ar make perl git jq yq tar awk sed grep sort head tail xargs cp mv mkdir rm rmdir mktemp tee shasum wc chmod cmp find uname shellcheck cargo-sort cargo-sort-derives)) {
        my ($file)=grep {-f $_ && -x $_} map {"$_/$name"} split /:/,$ENV{PATH};
        defined $file or die "missing admitted stage tool: $name\n";
        $files{$file}=1;
    }
    $files{$projection->{editor}{path}}=1;
    my $server=observation('bash','scripts/dev/testkit-server.sh','check'); $server =~ s/\n\z//;
    $files{$server}=1;
    my $testkit=observation('bash','scripts/dev/install-rust-tools.sh','--consumer',$prepared,
        '--package','ic-testkit','--lockfile',"$root/Cargo.lock",'--bin','ic-testkit-server',
        '--profile','release','--features','cli','--check');
    $testkit =~ s/\n\z//;
    $files{$testkit}=1;
    $tools={files=>[sort keys %files]};
    my $raw=$json->encode($tools)."\n";
    my $stable='target/release-stage-cache/tools/'.sha256_hex($raw).'.json';
    safe_directory('target/release-stage-cache/tools');
    if (-e $stable) { BlobValidation::read_file($stable) eq $raw or die "prepared tool inventory drift\n"; }
    else { write_record($stable,$tools); }
    $tools->{inventory}="$root/$stable";
    write_record($tools_path,$tools);
}
my %environment=(PATH=>$ENV{PATH},HOME=>$ENV{HOME},LC_ALL=>'C',RUSTUP_AUTO_INSTALL=>'0',CARGO_NET_OFFLINE=>'true',
    BLOB_PREPARED_TOOLS_ROOT=>$prepared,
    TMPDIR=>$temporary,CARGO_STAGE_TOOLS=>$tools->{inventory},CARGO_STAGE_IDENTITY=>$projection->{identity});
$environment{$_}=$ENV{$_} for grep {defined $ENV{$_}} qw(CARGO_HOME RUSTUP_HOME);
my $environment_path="$parent/$stage-environment.json";
write_record($environment_path,\%environment);
my $admission="$root/$destination/$stage-admission.json";
chdir $candidate or die $!;
run($^X,"$root/scripts/ci/run-validation-stage.pl",'--state',"$root/target/release-stage-cache/execution",
    '--stage',$stage,'--contract',$policy->{contract},'--source',$source,'--group',$group,
    '--observe',"$root/scripts/ci/observe-cargo-stage.pl",'--environment',"$root/$environment_path",
    '--max-bytes','536870912','--admission-output',$admission,'--',
    'make','--no-print-directory',"CARGO_TARGET_DIR=$root/target/release-stage-build","$stage-run");
chdir $root or die $!;
my $sealed=$json->decode(observation($^X,'scripts/ci/seal-validation-stage.pl',$admission,$source,$group,$projection->{identity}));
if (!$publication && $stage eq 'test-pocketic') {
    # The reusable suite tests exact final versions. This fresh, small sweep
    # separately qualifies the actual intermediate Engine/Contracts/CLI mix.
    run('make','--no-print-directory','test-release-identity');
    $sealed->{fresh_identity}=JSON::PP::true;
}
$sealed->{elapsed_seconds}=clock_gettime(CLOCK_MONOTONIC)-$started;
write_record("$destination/$stage.json",$sealed);
