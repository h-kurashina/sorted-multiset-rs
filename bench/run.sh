#!/usr/bin/env bash
# Rust 版と Python 版（tatyam 氏実装）に同じ操作列を流し、結果の一致と処理時間を比べる。
# 使い方: bench/run.sh [シード]
set -euo pipefail
cd "$(dirname "$0")/.."

seed="${1:-1}"
data=bench/data
mkdir -p "$data"
cargo build --release --quiet --bin bench

printf '| 操作列 | 操作数 | 値域 | Rust (ms) | Python (ms) | 倍率 | 結果 |\n|---|---:|---:|---:|---:|---:|:---:|\n'
for spec in "mixed 200000 1000000000" "mixed 200000 1000" "mixed 1000000 1000000000" "mixed 1000000 1000" \
            "rank 200000 1000000000" "rank 1000000 1000000000"; do
  set -- $spec
  mix=$1 q=$2 range=$3
  ops="$data/${mix}_${q}_${range}_${seed}.txt"
  [ -f "$ops" ] || python3 bench/gen_ops.py "$q" "$seed" "$range" "$mix" > "$ops"
  rust_ms=$(target/release/bench < "$ops" 2>&1 > "$data/rust.out")
  py_ms=$(python3 bench/run_python.py < "$ops" 2>&1 > "$data/py.out")
  if cmp -s "$data/rust.out" "$data/py.out"; then same=一致; else same=不一致; fi
  ratio=$(python3 -c "print(f'{$py_ms / $rust_ms:.1f}x')")
  printf '| %s | %s | %s | %s | %s | %s | %s |\n' "$mix" "$q" "$range" "$rust_ms" "$py_ms" "$ratio" "$same"
done
