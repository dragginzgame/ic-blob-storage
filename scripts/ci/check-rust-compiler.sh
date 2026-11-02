#!/usr/bin/env bash
set -euo pipefail
export RUSTUP_AUTO_INSTALL=0
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
cd "$root"
# Read the one owned channel field, refusing missing, moving or duplicate pins.
expected="$(perl -ne '
    if (/^\s*\[([^]]+)\]\s*(?:#.*)?$/) { $section = $1 }
    if ($section eq "toolchain" && /^\s*channel\s*=/) {
        /^\s*channel\s*=\s*"(\d+\.\d+\.\d+)"\s*(?:#.*)?$/ or die "Compiler pin must be an exact stable version\n";
        push @pins, $1;
    }
    END { @pins == 1 or die "Expected one toolchain channel\n"; print $pins[0] }
' rust-toolchain.toml)"
cargo_identity="$(cargo -vV)"
rustc_identity="$("${RUSTC:-rustc}" -vV)"
printf '%s\n%s\n' "$cargo_identity" "$rustc_identity"
for identity in "$cargo_identity" "$rustc_identity"; do
    actual="$(sed -n 's/^release: //p' <<< "$identity")"
    [[ "$actual" == "$expected" ]] || {
        printf 'Compiler admission refused: expected %s, observed %s\n' "$expected" "$actual" >&2
        exit 1
    }
done
cargo metadata --offline --locked --no-deps --format-version 1 |
    perl -MJSON::PP -0777 -e '
        my $expected = shift;
        my $metadata = decode_json(<>);
        my %members = map { $_ => 1 } @{$metadata->{workspace_members}};
        my @packages = grep { $members{$_->{id}} } @{$metadata->{packages}};
        @packages == keys(%members) && @packages or die "Incomplete workspace roster\n";
        for my $package (@packages) {
            ($package->{rust_version} // "") eq $expected
                or die "$package->{name}: rust-version must match compiler pin $expected\n";
        }
        print "Compiler policy admitted: $expected; ", scalar(@packages), " members\n";
    ' "$expected"
