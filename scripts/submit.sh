#!/usr/bin/env bash
# examples/ の解答を、ライブラリを埋め込んだ提出用の1ファイルにして標準出力に出す。
# 解答側の `#[path = ...] mod sorted_multiset;` を、scripts/paste.sh の出力に置き換える。
# 使い方: scripts/submit.sh examples/abc241_d.rs | pbcopy
set -euo pipefail
grep -v -e '^#\[path = "../src/sorted_multiset.rs"\]$' -e '^mod sorted_multiset;$' "$1"
"$(dirname "$0")/paste.sh"
