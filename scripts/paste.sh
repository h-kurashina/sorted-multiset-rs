#!/usr/bin/env bash
# 提出コードの末尾に貼り付ける形（mod sorted_multiset { ... }）で本体を標準出力に出す。
# テスト専用の #[cfg(test)] ブロック（ファイル末尾）は取り除く。
# 使い方: scripts/paste.sh >> main.rs  または  scripts/paste.sh | pbcopy
set -euo pipefail
src="$(dirname "$0")/../src/sorted_multiset.rs"

echo '#[allow(dead_code)]'
echo 'mod sorted_multiset {'
awk '/^#\[cfg\(test\)\]$/ { exit } { print }' "$src" | sed -e 's/^\(.\)/    \1/' | sed -e :a -e '/^\n*$/{$d;N;ba' -e '}'
echo '}'
