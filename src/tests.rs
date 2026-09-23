//! 単体テストと、ソート済み Vec による参照実装との差分テスト。
//!
//! 差分テストの操作回数は環境変数 SMS_OPS で変えられる（既定 20,000）。要件どおりに回すには:
//!   SMS_OPS=1000000 cargo test --profile release-checked

use crate::SortedMultiset;

// ---------------------------------------------------------------- 単体テスト

#[test]
fn empty_set() {
    let mut s = SortedMultiset::<i64>::new();
    assert!(s.is_empty());
    assert_eq!(s.len(), 0);
    assert!(!s.contains(&0));
    assert_eq!(s.count(&0), 0);
    assert_eq!(s.nth(0), None);
    assert_eq!(s.first(), None);
    assert_eq!(s.last(), None);
    assert_eq!(s.lt(&0), None);
    assert_eq!(s.le(&0), None);
    assert_eq!(s.gt(&0), None);
    assert_eq!(s.ge(&0), None);
    assert_eq!(s.index(&0), 0);
    assert_eq!(s.index_right(&0), 0);
    assert_eq!(s.iter().next(), None);
    assert!(!s.remove_one(&0));
    assert_eq!(s.pop_first(), None);
    assert_eq!(s.pop_last(), None);
    assert_eq!(s.pop_nth(0), None);
    s.check_invariants();
}

#[test]
fn single_element() {
    let mut s = SortedMultiset::new();
    s.insert(5);
    assert_eq!(s.len(), 1);
    assert_eq!(s.first(), Some(&5));
    assert_eq!(s.last(), Some(&5));
    assert_eq!(s.nth(0), Some(&5));
    assert_eq!(s.nth(1), None);
    assert_eq!(s.lt(&5), None);
    assert_eq!(s.le(&5), Some(&5));
    assert_eq!(s.gt(&5), None);
    assert_eq!(s.ge(&5), Some(&5));
    assert_eq!(s.lt(&6), Some(&5));
    assert_eq!(s.gt(&4), Some(&5));
    assert_eq!(s.index(&5), 0);
    assert_eq!(s.index_right(&5), 1);
    assert!(!s.remove_one(&4));
    assert!(s.remove_one(&5));
    assert!(s.is_empty());
    s.check_invariants();
}

#[test]
fn all_same_value() {
    let mut s: SortedMultiset<i32> = std::iter::repeat_n(7, 1000).collect();
    s.check_invariants();
    assert!(s.bucket_count() > 1);
    assert_eq!(s.count(&7), 1000);
    assert_eq!(s.index(&7), 0);
    assert_eq!(s.index_right(&7), 1000);
    assert_eq!(s.lt(&7), None);
    assert_eq!(s.gt(&7), None);
    assert_eq!(s.nth(999), Some(&7));
    assert_eq!(s.nth(1000), None);
    for _ in 0..500 {
        s.insert(7);
    }
    s.check_invariants();
    for left in (0..1500).rev() {
        assert!(s.remove_one(&7));
        assert_eq!(s.count(&7), left);
    }
    assert!(!s.remove_one(&7));
    s.check_invariants();
}

#[test]
fn out_of_range_index() {
    let mut s: SortedMultiset<u32> = (0..100).collect();
    assert_eq!(s.nth(99), Some(&99));
    assert_eq!(s.nth(100), None);
    assert_eq!(s.nth(usize::MAX), None);
    assert_eq!(s.pop_nth(100), None);
    assert_eq!(s.pop_nth(usize::MAX), None);
    assert_eq!(s.len(), 100);
}

#[test]
fn extreme_values_do_not_overflow() {
    let mut s: SortedMultiset<i64> = [i64::MIN, i64::MAX, 0, i64::MIN, i64::MAX]
        .into_iter()
        .collect();
    assert_eq!(s.lt(&i64::MIN), None);
    assert_eq!(s.gt(&i64::MAX), None);
    assert_eq!(s.le(&i64::MIN), Some(&i64::MIN));
    assert_eq!(s.ge(&i64::MAX), Some(&i64::MAX));
    assert_eq!(s.count(&i64::MIN), 2);
    assert_eq!(s.index_right(&i64::MAX), 5);
    assert_eq!(s.pop_last(), Some(i64::MAX));
    assert_eq!(s.pop_first(), Some(i64::MIN));
}

#[test]
fn from_iter_various_sizes() {
    for n in [0usize, 1, 2, 15, 16, 17, 100, 1000, 12345] {
        let v: Vec<usize> = (0..n).map(|i| (i * 7919) % (n / 3 + 1)).collect();
        let s: SortedMultiset<usize> = v.iter().copied().collect();
        s.check_invariants();
        let mut sorted = v.clone();
        sorted.sort();
        assert_eq!(s.iter().copied().collect::<Vec<_>>(), sorted);
        assert_eq!(
            s.iter().rev().copied().collect::<Vec<_>>(),
            sorted.iter().rev().copied().collect::<Vec<_>>()
        );
        assert_eq!(s.len(), n);
    }
}

#[test]
fn splits_on_growth() {
    let mut s = SortedMultiset::new();
    for i in 0..100_000u32 {
        s.insert(i.wrapping_mul(2654435761) % 1000);
    }
    s.check_invariants();
    assert!(s.bucket_count() > 10);
    while s.pop_first().is_some() {}
    assert_eq!(s.bucket_count(), 0);
    s.check_invariants();
}

#[test]
fn non_copy_elements() {
    let mut s: SortedMultiset<String> = ["pear", "apple", "fig", "apple"]
        .iter()
        .map(|x| x.to_string())
        .collect();
    assert_eq!(s.count(&"apple".to_string()), 2);
    assert_eq!(s.ge(&"b".to_string()).map(String::as_str), Some("fig"));
    assert_eq!(s.pop_last().as_deref(), Some("pear"));
    assert_eq!(format!("{:?}", s), r#"{"apple", "apple", "fig"}"#);
}

#[test]
fn traits() {
    let mut s = SortedMultiset::default();
    s.extend([3, 1, 2, 1]);
    let collected: Vec<i32> = (&s).into_iter().copied().collect();
    assert_eq!(collected, [1, 1, 2, 3]);
    let t = s.clone();
    s.insert(0);
    assert_eq!(t.len(), 4);
    assert_eq!(format!("{:?}", t), "{1, 1, 2, 3}");
}

// ---------------------------------------------------------------- 差分テスト

/// 遅いが明らかに正しい参照実装
struct Naive(Vec<i64>);

impl Naive {
    fn lower(&self, x: i64) -> usize {
        self.0.partition_point(|&y| y < x)
    }
    fn upper(&self, x: i64) -> usize {
        self.0.partition_point(|&y| y <= x)
    }
    fn insert(&mut self, x: i64) {
        let i = self.lower(x);
        self.0.insert(i, x);
    }
    fn remove_one(&mut self, x: i64) -> bool {
        let i = self.lower(x);
        if self.0.get(i) == Some(&x) {
            self.0.remove(i);
            true
        } else {
            false
        }
    }
    fn pop_nth(&mut self, k: usize) -> Option<i64> {
        (k < self.0.len()).then(|| self.0.remove(k))
    }
}

/// 外部クレートを使わないための xorshift64
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

fn ops_from_env() -> usize {
    std::env::var("SMS_OPS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(20_000)
}

/// 値域 [0, range) の値でランダム操作列を流し、全APIの戻り値を参照実装と比較する。
/// 要素数は target 付近を行き来するように挿入と削除の比率を変える。
fn run_diff(seed: u64, range: i64, target: usize, ops: usize) {
    let mut rng = Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
    let mut s = SortedMultiset::new();
    let mut n = Naive(Vec::new());
    let ctx = |step: usize| format!("seed={seed} range={range} target={target} step={step}");

    for step in 0..ops {
        // 範囲外の値も問い合わせる
        let x = rng.below(range as u64 + 4) as i64 - 2;
        let k = rng.below(n.0.len() as u64 + 3) as usize;
        let grow = n.0.len() < target;
        match rng.below(20) {
            0..=5 => {
                if grow || rng.below(3) == 0 {
                    s.insert(x);
                    n.insert(x);
                } else {
                    assert_eq!(
                        s.remove_one(&x),
                        n.remove_one(x),
                        "remove_one {}",
                        ctx(step)
                    );
                }
            }
            6..=7 => {
                if !grow {
                    // 存在する値を狙って消す
                    let y = n.0.get(k).copied().unwrap_or(x);
                    assert_eq!(
                        s.remove_one(&y),
                        n.remove_one(y),
                        "remove_one {}",
                        ctx(step)
                    );
                }
            }
            8 => {
                if !grow {
                    assert_eq!(s.pop_nth(k), n.pop_nth(k), "pop_nth {}", ctx(step));
                }
            }
            9 => {
                if !grow {
                    let (a, b) = if rng.below(2) == 0 {
                        (s.pop_first(), (!n.0.is_empty()).then(|| n.0.remove(0)))
                    } else {
                        (s.pop_last(), n.0.pop())
                    };
                    assert_eq!(a, b, "pop_first/last {}", ctx(step));
                }
            }
            10 => assert_eq!(s.nth(k), n.0.get(k), "nth {}", ctx(step)),
            11 => assert_eq!(
                s.lt(&x),
                n.lower(x).checked_sub(1).map(|i| &n.0[i]),
                "lt {}",
                ctx(step)
            ),
            12 => assert_eq!(
                s.le(&x),
                n.upper(x).checked_sub(1).map(|i| &n.0[i]),
                "le {}",
                ctx(step)
            ),
            13 => assert_eq!(s.gt(&x), n.0.get(n.upper(x)), "gt {}", ctx(step)),
            14 => assert_eq!(s.ge(&x), n.0.get(n.lower(x)), "ge {}", ctx(step)),
            15 => assert_eq!(s.index(&x), n.lower(x), "index {}", ctx(step)),
            16 => assert_eq!(s.index_right(&x), n.upper(x), "index_right {}", ctx(step)),
            17 => {
                let c = s.count(&x);
                assert_eq!(c, n.upper(x) - n.lower(x), "count {}", ctx(step));
                assert_eq!(
                    c,
                    s.index_right(&x) - s.index(&x),
                    "count invariant {}",
                    ctx(step)
                );
                assert_eq!(s.contains(&x), c > 0, "contains {}", ctx(step));
            }
            18 => {
                assert_eq!(s.first(), n.0.first(), "first {}", ctx(step));
                assert_eq!(s.last(), n.0.last(), "last {}", ctx(step));
            }
            _ => {
                assert_eq!(s.len(), n.0.len(), "len {}", ctx(step));
                assert_eq!(s.is_empty(), n.0.is_empty(), "is_empty {}", ctx(step));
            }
        }
        assert_eq!(s.len(), n.0.len(), "len {}", ctx(step));
        s.check_structure();
        // バケット内の昇順と全要素の一致は O(n) なので間引いて検査する
        if step % 1024 == 0 || step + 1 == ops {
            s.check_invariants();
            assert!(s.iter().eq(n.0.iter()), "iter {}", ctx(step));
            assert!(
                s.iter().rev().eq(n.0.iter().rev()),
                "iter rev {}",
                ctx(step)
            );
        }
    }
}

const SEEDS: [u64; 3] = [1, 2, 3];

#[test]
fn diff_heavy_duplicates() {
    for seed in SEEDS {
        run_diff(seed, 8, 60, ops_from_env());
    }
}

#[test]
fn diff_medium_range() {
    for seed in SEEDS {
        run_diff(seed, 1000, 400, ops_from_env());
    }
}

#[test]
fn diff_wide_range_large_set() {
    for seed in SEEDS {
        run_diff(seed, 1_000_000_000, 5000, ops_from_env());
    }
}

#[test]
fn diff_narrow_range_large_set() {
    for seed in SEEDS {
        run_diff(seed, 50, 5000, ops_from_env());
    }
}
