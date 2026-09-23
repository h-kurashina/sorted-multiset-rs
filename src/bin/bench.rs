//! 操作列を Rust 版 SortedMultiset で実行する。形式は bench/ops.md を参照。
//!
//! 使い方: cargo run --release --bin bench < ops.txt > out.txt
//! 処理時間（入力の読み込みと出力の書き込みを除く）をミリ秒で標準エラーに出す。
//! Python 版と条件を揃えるため、行の解析は計測に含める。

use std::fmt::Write as _;
use std::io::{self, Read, Write};
use std::time::Instant;

use sorted_multiset_for_rust::SortedMultiset;

fn opt(v: Option<&i64>) -> i64 {
    v.copied().unwrap_or(-1)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let start = Instant::now();
    let mut tokens = input.split_ascii_whitespace();
    let mut next = || -> i64 { tokens.next().unwrap().parse().unwrap() };
    let q = next() as usize;
    let mut s = SortedMultiset::new();
    let mut out = String::with_capacity(q * 8);
    for _ in 0..q {
        let op = next();
        let r = match op {
            0 => {
                s.insert(next());
                continue;
            }
            1 => s.remove_one(&next()) as i64,
            2 => opt(s.nth(next() as usize)),
            3 => opt(s.ge(&next())),
            4 => opt(s.gt(&next())),
            5 => opt(s.le(&next())),
            6 => opt(s.lt(&next())),
            7 => s.index(&next()) as i64,
            8 => s.index_right(&next()) as i64,
            9 => s.count(&next()) as i64,
            10 => s.contains(&next()) as i64,
            11 => s.pop_nth(next() as usize).unwrap_or(-1),
            12 => s.pop_first().unwrap_or(-1),
            13 => s.pop_last().unwrap_or(-1),
            _ => panic!("unknown op {op}"),
        };
        writeln!(out, "{r}").unwrap();
    }
    let elapsed = start.elapsed();

    io::stdout().write_all(out.as_bytes()).unwrap();
    eprintln!("{:.1}", elapsed.as_secs_f64() * 1000.0);
}
