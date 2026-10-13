#!/usr/bin/env bash
# Shared companions: scripts/dev/tool-store.sh scripts/ci/verify-file-checksum.sh
set -euo pipefail
ROOT="${BASH_SOURCE[0]}"
[[ "$ROOT" == /* ]] || ROOT="$PWD/$ROOT"
ROOT="$(cd -P "${ROOT%/*}/../.." && printf '%s/.' "$PWD")"; ROOT="${ROOT%/.}"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/tool-downloads.XXXXXX")"
server=''
fixture_complete=false
finish() {
    local status=$?
    [[ "$fixture_complete" == true || "$status" != 0 ]] || status=1
    if [[ -n "$server" ]]; then kill "$server" 2>/dev/null || true; wait "$server" 2>/dev/null || true; fi
    if [[ "$status" == 0 ]]; then rm -rf "$fixture"
    else printf 'Download fixtures retained: %s\n' "$fixture" >&2; fi
    exit "$status"
}
trap finish EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
real_curl="$(command -v curl)"
# Core Perl loopback server: exercise curl's actual retry classifier and output
# truncation, not a second implementation of its retry algorithm.
perl -MIO::Socket::INET -e '
    my $root=shift;
    my $s=IO::Socket::INET->new(LocalAddr=>"127.0.0.1",LocalPort=>0,Listen=>8,ReuseAddr=>1) or die $!;
    open my $p,">","$root/port" or die $!; print $p $s->sockport; close $p or die $!;
    my %count;
    while (1) {
        alarm 30;
        my $c=$s->accept or die $!;
        my $line=<$c>; defined $line or die "missing request";
        $line =~ m{^GET /([a-z0-9-]+) HTTP/} or die "unexpected request";
        my $route=$1;
        while (my $header=<$c>) { last if $header eq "\r\n"; }
        my $n=++$count{$route};
        open my $log,">>","$root/requests" or die $!; print $log "$route\n"; close $log or die $!;
        my $status=$route eq "exhaust" ? 504 : $route eq "permanent" ? 404 :
            $route eq "rate-limit" && $n==1 ? 429 : $route eq "recover" && $n==1 ? 500 : 200;
        my $body=$route eq "corrupt" ? "wrong" : $status==200 ? "abc" : "failed attempt";
        print $c "HTTP/1.1 $status Fixture\r\nContent-Length: ".length($body)."\r\nConnection: close\r\n\r\n$body";
        close $c;
    }
' "$fixture" > "$fixture/server.log" 2>&1 &
server=$!
for ((attempt=0; attempt<50; attempt++)); do
    [[ ! -s "$fixture/port" ]] || break
    kill -0 "$server" 2>/dev/null || { cat "$fixture/server.log" >&2; exit 1; }
    sleep 0.1
done
[[ -s "$fixture/port" ]] || { echo 'fixture server did not start' >&2; exit 1; }
port="$(cat "$fixture/port")"
curl() {
    local args=() protocols=0 tls=0
    # Substitute only transport to avoid adding test certificates/TLS packages.
    # Production must still supply HTTPS-only restrictions and TLS 1.2.
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --proto|--proto-redir)
                [[ "$2" == '=https' ]] || return 90
                protocols=$((protocols + 1)); args+=("$1" '=http'); shift 2 ;;
            --tlsv1.2) tls=1; shift ;;
            https://fixture.invalid/*)
                args+=("http://127.0.0.1:$port/${1##*/}"); shift ;;
            *) args+=("$1"); shift ;;
        esac
    done
    [[ "$protocols" == 2 && "$tls" == 1 ]] || return 90
    "$real_curl" -q --noproxy '*' "${args[@]}"
}
# shellcheck source=scripts/dev/tool-store.sh
source "$ROOT/scripts/dev/tool-store.sh"
digest=ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad
for route in recover rate-limit exhaust permanent corrupt; do
    status=0
    tool_store_download "https://fixture.invalid/$route" "$fixture/$route" > "$fixture/$route.log" 2>&1 || status=$?
    expected=0; count=2
    case "$route" in exhaust) expected=22; count=4 ;; permanent) expected=22; count=1 ;; corrupt) count=1 ;; esac
    [[ "$status" == "$expected" && "$(grep -c "^$route$" "$fixture/requests")" -eq "$count" ]] || exit 1
    if [[ "$route" == recover || "$route" == rate-limit ]]; then
        bash "$ROOT/scripts/ci/verify-file-checksum.sh" sha256 "$digest" "$fixture/$route"
    elif [[ "$route" == corrupt ]]; then
        if bash "$ROOT/scripts/ci/verify-file-checksum.sh" sha256 "$digest" "$fixture/$route" > "$fixture/checksum.log" 2>&1; then exit 1; fi
        [[ "$(cat "$fixture/$route")" == wrong ]] || exit 1
    fi
done
fixture_complete=true
echo 'Real curl transient recovery, exhaustion, permanent failure and checksum refusal passed (loopback HTTP transport)'
