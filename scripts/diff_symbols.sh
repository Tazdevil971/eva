#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 2 ]]; then
    echo "Usage: $0 <old.elf> <new.elf>"
    exit 1
fi

OLD="$1"
NEW="$2"

TMP_OLD=$(mktemp)
TMP_NEW=$(mktemp)

cleanup() {
    rm -f "$TMP_OLD" "$TMP_NEW"
}
trap cleanup EXIT

extract_symbols() {
    arm-none-eabi-objdump -t "$1" \
        | awk '
            NF >= 6 && ($3 == "F" || $3 == "O") {
                # name + size
                printf "%s %s\n", $NF, strtonum("0x"$5)
            }
        ' \
        | c++filt \
        | rustfilt \
        | sort -k1,1 -u
}

extract_symbols "$OLD" > "$TMP_OLD"
extract_symbols "$NEW" > "$TMP_NEW"

echo "=== Added symbols (sorted by size) ==="
join -v2 "$TMP_OLD" "$TMP_NEW" \
    | sort -k2,2nr

echo
echo "=== Removed symbols (sorted by size) ==="
join -v1 "$TMP_OLD" "$TMP_NEW" \
    | sort -k2,2nr