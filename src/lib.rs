//! 競技プログラミング向けの重複ありの順序付き集合 [`SortedMultiset`]。
//!
//! 本体は `src/sorted_multiset.rs` の1ファイルで完結しており、提出コードに貼り付けて使う。

pub mod sorted_multiset;

pub use sorted_multiset::SortedMultiset;

#[cfg(test)]
mod tests;
