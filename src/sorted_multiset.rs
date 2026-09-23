//! 重複を許す順序付き集合 SortedMultiset（平方分割）。
//!
//! 標準ライブラリのみで完結する。提出時は `mod sorted_multiset { ... }` で囲んで貼り付ける
//! （`scripts/paste.sh` がその形で出力する）。
//! 内部構造は tatyam 氏の Python 版 SortedMultiset に合わせている。
//! CC0-1.0（パブリックドメイン相当）。貼り付けるときに著作権表示やライセンス文は要らない。

use std::fmt;

/// 構築時のバケット数: ceil(sqrt(n / BUCKET_RATIO))
const BUCKET_RATIO: usize = 4;
/// バケットの長さが (バケット数 * SPLIT_RATIO) を超えたら半分に分割する。
/// Python 版は (16, 24)。Rust ではバケット内の memmove が効くため小さくした（README のベンチマーク参照）。
/// 構築直後のバケット長に対する分割閾値の比 SPLIT_RATIO / BUCKET_RATIO = 1.5 は Python 版と同じ。
const SPLIT_RATIO: usize = 6;

/// 重複を許す順序付き集合。インデックスは0始まりで、該当なしは `None` を返す。
#[derive(Clone)]
pub struct SortedMultiset<T> {
    /// 各バケットは空でなく昇順、バケット間も昇順
    buckets: Vec<Vec<T>>,
    len: usize,
}

/// 昇順のイテレータ。`.rev()` で降順になる。
pub type Iter<'a, T> = std::iter::Flatten<std::slice::Iter<'a, Vec<T>>>;

impl<T: Ord> SortedMultiset<T> {
    /// 空の集合を作る。O(1)
    pub fn new() -> Self {
        Self {
            buckets: Vec::new(),
            len: 0,
        }
    }

    fn from_sorted_vec(a: Vec<T>) -> Self {
        let n = a.len();
        let mut k = 0;
        while k * k * BUCKET_RATIO < n {
            k += 1;
        }
        let mut it = a.into_iter();
        let buckets = (0..k)
            .map(|i| it.by_ref().take(n * (i + 1) / k - n * i / k).collect())
            .collect();
        Self { buckets, len: n }
    }

    /// 要素数。O(1)
    pub fn len(&self) -> usize {
        self.len
    }

    /// 空かどうか。O(1)
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// x 以上の最初の要素の位置 (バケット, バケット内の位置)。無ければ (バケット数, 0)。
    fn lower(&self, x: &T) -> (usize, usize) {
        let b = self.buckets.partition_point(|a| a[a.len() - 1] < *x);
        match self.buckets.get(b) {
            Some(a) => (b, a.partition_point(|y| y < x)),
            None => (b, 0),
        }
    }

    /// x より大きい最初の要素の位置。無ければ (バケット数, 0)。
    fn upper(&self, x: &T) -> (usize, usize) {
        let b = self.buckets.partition_point(|a| a[a.len() - 1] <= *x);
        match self.buckets.get(b) {
            Some(a) => (b, a.partition_point(|y| y <= x)),
            None => (b, 0),
        }
    }

    fn at(&self, (b, i): (usize, usize)) -> Option<&T> {
        self.buckets.get(b).and_then(|a| a.get(i))
    }

    /// 位置 (b, i) の直前の要素
    fn before(&self, (b, i): (usize, usize)) -> Option<&T> {
        if i > 0 {
            Some(&self.buckets[b][i - 1])
        } else if b > 0 {
            self.buckets[b - 1].last()
        } else {
            None
        }
    }

    /// 位置 (b, i) より前にある要素数。近い側の端から数える。
    fn rank(&self, (b, i): (usize, usize)) -> usize {
        if b <= self.buckets.len() / 2 {
            self.buckets[..b].iter().map(Vec::len).sum::<usize>() + i
        } else {
            self.len - self.buckets[b..].iter().map(Vec::len).sum::<usize>() + i
        }
    }

    /// k 番目の要素の位置。近い側の端から数える。
    fn locate(&self, k: usize) -> Option<(usize, usize)> {
        if k >= self.len {
            return None;
        }
        if k < self.len / 2 {
            let mut k = k;
            for (b, a) in self.buckets.iter().enumerate() {
                if k < a.len() {
                    return Some((b, k));
                }
                k -= a.len();
            }
        } else {
            // r: k 番目から末尾までの要素数（1以上）
            let mut r = self.len - k;
            for (b, a) in self.buckets.iter().enumerate().rev() {
                if r <= a.len() {
                    return Some((b, a.len() - r));
                }
                r -= a.len();
            }
        }
        unreachable!("bucket lengths do not add up to len")
    }

    fn remove_at(&mut self, (b, i): (usize, usize)) -> T {
        let x = self.buckets[b].remove(i);
        if self.buckets[b].is_empty() {
            self.buckets.remove(b);
        }
        self.len -= 1;
        x
    }

    /// x を1個追加する。O(√n)
    pub fn insert(&mut self, x: T) {
        self.len += 1;
        if self.buckets.is_empty() {
            self.buckets.push(vec![x]);
            return;
        }
        let (mut b, mut i) = self.lower(&x);
        if b == self.buckets.len() {
            b -= 1;
            i = self.buckets[b].len();
        }
        let limit = self.buckets.len() * SPLIT_RATIO;
        let a = &mut self.buckets[b];
        a.insert(i, x);
        if a.len() > limit {
            let right = a.split_off(a.len() / 2);
            self.buckets.insert(b + 1, right);
        }
    }

    /// x を1個だけ削除する。無ければ false。O(√n)
    pub fn remove_one(&mut self, x: &T) -> bool {
        let pos = self.lower(x);
        if self.at(pos) != Some(x) {
            return false;
        }
        self.remove_at(pos);
        true
    }

    /// x が1個以上あるか。O(√n)
    pub fn contains(&self, x: &T) -> bool {
        self.at(self.lower(x)) == Some(x)
    }

    /// x の個数。O(√n)
    pub fn count(&self, x: &T) -> usize {
        self.index_right(x) - self.index(x)
    }

    /// 小さい方から k 番目（0始まり）の要素。O(√n)
    pub fn nth(&self, k: usize) -> Option<&T> {
        self.locate(k).map(|(b, i)| &self.buckets[b][i])
    }

    /// 最小値。O(1)
    pub fn first(&self) -> Option<&T> {
        self.buckets.first().and_then(|a| a.first())
    }

    /// 最大値。O(1)
    pub fn last(&self) -> Option<&T> {
        self.buckets.last().and_then(|a| a.last())
    }

    /// 最小値を取り出す。O(√n)
    pub fn pop_first(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        Some(self.remove_at((0, 0)))
    }

    /// 最大値を取り出す。O(√n)
    pub fn pop_last(&mut self) -> Option<T> {
        let b = self.buckets.len().checked_sub(1)?;
        let i = self.buckets[b].len() - 1;
        Some(self.remove_at((b, i)))
    }

    /// k 番目の要素を取り出す。O(√n)
    pub fn pop_nth(&mut self, k: usize) -> Option<T> {
        let pos = self.locate(k)?;
        Some(self.remove_at(pos))
    }

    /// x 未満の最大要素。O(√n)
    pub fn lt(&self, x: &T) -> Option<&T> {
        self.before(self.lower(x))
    }

    /// x 以下の最大要素。O(√n)
    pub fn le(&self, x: &T) -> Option<&T> {
        self.before(self.upper(x))
    }

    /// x より大きい最小要素。O(√n)
    pub fn gt(&self, x: &T) -> Option<&T> {
        self.at(self.upper(x))
    }

    /// x 以上の最小要素。O(√n)
    pub fn ge(&self, x: &T) -> Option<&T> {
        self.at(self.lower(x))
    }

    /// x 未満の要素数（x を挿入する位置の左端）。O(√n)
    pub fn index(&self, x: &T) -> usize {
        self.rank(self.lower(x))
    }

    /// x 以下の要素数（x を挿入する位置の右端）。O(√n)
    pub fn index_right(&self, x: &T) -> usize {
        self.rank(self.upper(x))
    }

    /// 昇順イテレータ。`.rev()` で降順。
    pub fn iter(&self) -> Iter<'_, T> {
        self.buckets.iter().flatten()
    }
}

impl<T: Ord> Default for SortedMultiset<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> FromIterator<T> for SortedMultiset<T> {
    /// 任意の要素列から構築する。O(n log n)、ソート済みなら O(n)
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut a: Vec<T> = iter.into_iter().collect();
        a.sort();
        Self::from_sorted_vec(a)
    }
}

impl<T: Ord> Extend<T> for SortedMultiset<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for x in iter {
            self.insert(x);
        }
    }
}

impl<'a, T: Ord> IntoIterator for &'a SortedMultiset<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: Ord + fmt::Debug> fmt::Debug for SortedMultiset<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

#[cfg(test)]
impl<T: Ord> SortedMultiset<T> {
    pub(crate) fn bucket_count(&self) -> usize {
        self.buckets.len()
    }

    /// 毎操作後に呼べる軽い検査: バケットが空でない、合計長が len と一致、バケットの境界が昇順。O(バケット数)
    pub(crate) fn check_structure(&self) {
        assert!(self.buckets.iter().all(|a| !a.is_empty()), "empty bucket");
        assert_eq!(
            self.buckets.iter().map(Vec::len).sum::<usize>(),
            self.len,
            "len mismatch"
        );
        for w in self.buckets.windows(2) {
            assert!(w[0][w[0].len() - 1] <= w[1][0], "buckets out of order");
        }
    }

    /// 全体の検査: 上に加えて各バケット内が昇順。O(n)
    pub(crate) fn check_invariants(&self) {
        self.check_structure();
        for a in &self.buckets {
            assert!(a.windows(2).all(|w| w[0] <= w[1]), "bucket not sorted");
        }
    }
}
