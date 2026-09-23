//! ABC241 D - Sequence Query
//! https://atcoder.jp/contests/abc241/tasks/abc241_d
//!
//! 手元で実行: cargo run --example abc241_d < 入力
//! 提出用コード: scripts/submit.sh examples/abc241_d.rs

use std::io::{self, Read, Write};

use sorted_multiset::SortedMultiset;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input
        .split_ascii_whitespace()
        .map(|t| t.parse::<u64>().unwrap());
    let q = it.next().unwrap();
    let mut s = SortedMultiset::new();
    let mut out = String::new();
    for _ in 0..q {
        let t = it.next().unwrap();
        let x = it.next().unwrap();
        if t == 1 {
            s.insert(x);
            continue;
        }
        let k = it.next().unwrap() as usize;
        let ans = if t == 2 {
            // x 以下で大きい方から k 番目
            let i = s.index_right(&x);
            i.checked_sub(k).and_then(|j| s.nth(j))
        } else {
            // x 以上で小さい方から k 番目
            s.nth(s.index(&x) + k - 1)
        };
        match ans {
            Some(v) => out.push_str(&format!("{v}\n")),
            None => out.push_str("-1\n"),
        }
    }
    io::stdout().write_all(out.as_bytes()).unwrap();
}

#[path = "../src/sorted_multiset.rs"]
mod sorted_multiset;
