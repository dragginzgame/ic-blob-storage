#!/usr/bin/env perl
# Shared companions: docs/release-validation-reuse.md
use strict;
use warnings;
use Cwd qw(abs_path getcwd);
use JSON::PP;
@ARGV==3 or die "usage: check-cargo-stage-closure.pl ACTUAL CANDIDATE PRIMARY\n";
my ($actual,$candidate,$primary)=@ARGV;
$primary =~ /\A[A-Za-z0-9][A-Za-z0-9_-]*\z/ or die "invalid primary\n";
my $json=JSON::PP->new->canonical->utf8;
sub graph {
    my ($root)=@_; $root=abs_path($root)//die "missing closure root\n";
    my $original=getcwd(); chdir $root or die $!;
    open my $in,'-|','cargo','metadata','--offline','--locked','--all-features','--format-version','1' or die $!;
    local $/; my $raw=<$in>; close $in or die "closure metadata failed\n";
    chdir $original or die $!;
    # Rebase only Cargo-owned path/ID fields. Do not normalize arbitrary package
    # metadata strings, requirements or application configuration.
    my $encoded=$root; $encoded =~ s{([^A-Za-z0-9._~/-])}{sprintf('%%%02X',ord($1))}ge;
    my $id=sub { my ($value)=@_; $value =~ s{\Apath\+file://\Q$encoded\E/}{path+file://<source>/}; return $value; };
    my $path=sub { my ($value)=@_; $value =~ s{\A\Q$root\E/}{<source>/}; return $value; };
    my $meta=$json->decode($raw);
    for my $package (@{$meta->{packages}}) {
        $package->{id}=$id->($package->{id});
        $package->{manifest_path}=$path->($package->{manifest_path});
        $_->{src_path}=$path->($_->{src_path}) for @{$package->{targets}};
        for my $dependency (@{$package->{dependencies}}) {
            $dependency->{path}=$path->($dependency->{path}) if defined $dependency->{path};
        }
    }
    for my $node (@{$meta->{resolve}{nodes}}) {
        $node->{id}=$id->($node->{id});
        $node->{dependencies}=[map {$id->($_)} @{$node->{dependencies}}];
        $_->{pkg}=$id->($_->{pkg}) for @{$node->{deps}};
    }
    my %packages=map {$_->{id}=>$_} @{$meta->{packages}};
    my %nodes=map {$_->{id}=>$_} @{$meta->{resolve}{nodes}};
    my @roots=grep {$_->{name} eq $primary && !defined($_->{source})} values %packages;
    @roots==1 or die "ambiguous publication primary\n";
    my (@closure,%seen); my @queue=($roots[0]{id});
    while (@queue) {
        my $id=shift @queue; next if $seen{$id}++;
        my $node=$nodes{$id}//die "missing resolved closure node\n";
        push @closure,{package=>$packages{$id}//die("missing closure package\n"),node=>$node};
        push @queue,@{$node->{dependencies}};
    }
    return [sort {$a->{package}{id} cmp $b->{package}{id}} @closure];
}
$json->encode(graph($actual)) eq $json->encode(graph($candidate))
    or die "actual publication closure differs from qualified final candidate\n";
print "Publication package, target, feature and dependency closure matches final candidate: $primary\n";
