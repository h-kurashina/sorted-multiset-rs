# SortedMultiset for Rust

競技プログラミング向けの、重複を許す順序付き集合 `SortedMultiset<T: Ord>`。
[tatyam 氏の Python 版](https://github.com/tatyam-prime/SortedSet)と同じ平方分割で、同じ感覚で使える API を持つ。

- 標準ライブラリのみ、`unsafe` なし
- CC0-1.0。提出コードに貼り付けるときに著作権表示やライセンス文は要らない
- 本体は [`src/sorted_multiset.rs`](src/sorted_multiset.rs) の1ファイル。貼り付け形式で288行
- AtCoder のジャッジ（rustc 1.89.0）でコンパイルできることを確認済み（edition 2021 / 2024 とも警告なし）

## 提出コードに貼り付ける

```bash
scripts/paste.sh | pbcopy                        # mod sorted_multiset { ... } をコピー
scripts/submit.sh examples/abc241_d.rs | pbcopy  # 解答とライブラリを1ファイルにまとめる
```

```rust
use sorted_multiset::SortedMultiset;

let mut s: SortedMultiset<i64> = [3, 1, 4, 1, 5].into_iter().collect();
s.insert(2);
assert_eq!(s.nth(1), Some(&1));   // 小さい方から1番目（0始まり）
assert_eq!(s.ge(&3), Some(&3));   // 3 以上の最小
assert_eq!(s.index(&4), 4);       // 4 未満の個数
assert!(s.remove_one(&1));        // 1 を1個だけ削除
```

## API

インデックスは0始まり、該当なしは `None`（panic しない）。計算量は要素数 n に対する目安。

| API | 動作 | 計算量 |
|---|---|---|
| `new()` / `from_iter(iter)` (`collect`) | 空の集合 / 要素列から構築 | O(1) / O(n log n) |
| `insert(x)` | x を1個追加 | O(√n) |
| `remove_one(&x) -> bool` | x を1個だけ削除、無ければ false | O(√n) |
| `len()` / `is_empty()` | 要素数 | O(1) |
| `contains(&x)` / `count(&x)` | x があるか / x の個数 | O(√n) |
| `nth(k)` / `pop_nth(k)` | k 番目の要素 / 取り出す | O(√n) |
| `first()` / `last()` | 最小値 / 最大値 | O(1) |
| `pop_first()` / `pop_last()` | 最小値 / 最大値を取り出す | O(√n) |
| `lt(&x)` / `le(&x)` | x 未満 / x 以下の最大要素 | O(√n) |
| `gt(&x)` / `ge(&x)` | x より大 / x 以上の最小要素 | O(√n) |
| `index(&x)` / `index_right(&x)` | x 未満 / x 以下の要素数 | O(√n) |
| `iter()` | 昇順イテレータ（`.rev()` で降順） | 1要素あたり償却 O(1) |

`Default`・`Clone`・`Debug`・`Extend`・`&SortedMultiset` の `IntoIterator` も実装している。

## 開発

```bash
cargo test                                          # 単体テスト + 差分テスト（各2万操作）
SMS_OPS=1000000 cargo test --profile release-checked  # 要件どおり: 10^6 操作 × 3シード、最適化 + overflow-checks
cargo clippy --all-targets -- -D warnings
bench/run.sh [シード]                               # Python 版とのベンチマーク
```

### テスト

- **単体テスト**: 空集合、要素1個、全要素が同じ値、範囲外の k、存在しない値の削除、`i64::MIN`/`MAX`、Copy でない型（`String`）など
- **差分テスト**: ソート済み `Vec` による参照実装と並べ、乱数シード固定のランダム操作列で全 API の戻り値を比較する。値域（8〜10^9）と目標要素数（60〜5000）を変えた4パターン × 3シード。範囲外の値や k も問い合わせる
- **不変条件**: 毎操作後に「バケットが空でない」「合計長が len と一致」「バケット間が昇順」を O(バケット数) で検査する。O(n) かかる「バケット内が昇順」と全要素の一致は1024操作ごとと最後に検査する
- 差分テストがバグを検出できることは、わざとバグを入れて確認した

### ベンチマーク

`bench/gen_ops.py` が作った操作列を、Rust 版（`src/bin/bench.rs`）と Python 版（`bench/run_python.py`）の両方で実行し、出力の一致と処理時間を比べる。処理時間は入出力を除き、行の解析は含む。操作列は2種類ある。

- **mixed**: insert 35%・削除15%・残りを各種の問い合わせに振り分けたもの。バケットの長さを足し合わせる操作（nth・index・index_right・count・pop_nth）は計23%
- **rank**: その足し合わせる操作を計60%にしたもの。バケット数を増やしたときの劣化を見るために使う

Apple Silicon / rustc 1.98.1 / Python 3.14.6、シード1での結果:

| 操作列 | 操作数 | 値域 | Rust (ms) | Python (ms) | 倍率 | 結果 |
|---|---:|---:|---:|---:|---:|:---:|
| mixed | 200000 | 10^9 | 23.7 | 250.8 | 10.6x | 一致 |
| mixed | 200000 | 1000 | 21.6 | 227.1 | 10.5x | 一致 |
| mixed | 1000000 | 10^9 | 165.4 | 2121.0 | 12.8x | 一致 |
| mixed | 1000000 | 1000 | 152.5 | 1744.8 | 11.4x | 一致 |
| rank | 200000 | 10^9 | 24.4 | 267.7 | 11.0x | 一致 |
| rank | 1000000 | 10^9 | 171.9 | 2377.4 | 13.8x | 一致 |

性能目標（10^6 操作で1秒以内、2×10^5 操作で100ミリ秒以内、Python 版の10倍以上）はすべて満たす。

### 定数の調整とプロファイル

Python 版の定数（`BUCKET_RATIO = 16`, `SPLIT_RATIO = 24`）のままだと、倍率は 9.9〜11.4倍で目標ぎりぎりだった。操作の種類ごとに時間を計ると（10^6 操作、最終要素数 約30万）:

- 行の解析と出力の整形は全体の約15%（1操作あたり約25ns）
- 検索系（`ge`・`lt`・`index` など）はどれも約145ns。バケットの末尾での二分探索と、バケット内での二分探索のたびに、ヒープ上の別の場所を読むキャッシュミスが支配的
- `insert` は約300ns。上の検索に加えて、バケット内の memmove（バケット長 約1900）がかかる

memmove を減らすためにバケットを小さくした。`SPLIT_RATIO` を 24→1 まで振ると6付近で改善が頭打ちになったので `SPLIT_RATIO = 6` とした。`BUCKET_RATIO = 4` は、構築直後のバケット長に対する分割閾値の比（1.5倍）を Python 版と揃えるための値。

バケットを小さくするとバケット数が増える（約30万要素で約2倍）ので、バケットの長さを足し合わせる nth・index 系は遅くなる。rank 操作列（10^6 操作）で1操作あたりの時間を比べると:

| 定数 (BUCKET, SPLIT) | insert | pop_nth | nth | index | count | 合計 |
|---|---:|---:|---:|---:|---:|---:|
| (16, 24) Python 版と同じ | 307ns | 198ns | 43ns | 150ns | 176ns | 191.5ms |
| (4, 6) 採用 | 244ns | 134ns | 51ns | 159ns | 197ns | 170.1ms |

nth・index・count は予想どおり 8〜21ns 遅くなるが、insert と pop_nth の memmove が 60ns 以上減るほうが大きい。足し合わせは連続した Vec ヘッダを順に読むだけでキャッシュに乗りやすく、近い側の端から数えるので走査するのはバケット数の半分以下で済むため、劣化は小さい。ただし、ほぼ挿入・削除のない「一度構築して問い合わせるだけ」の使い方では、上の1操作あたりの差（nth で約19%、index で約6%、count で約12%）がそのまま効き、(16, 24) より遅くなる。とはいえ挿入・削除がないならソート済みの `Vec` と `partition_point` で足りて、そのほうが速い。つまりこの弱点が出るのは SortedMultiset を使う理由がない場面だけなので、(4, 6) を採用した。

検索がキャッシュミスで律速している点は Python 版も同じ構造である。Python 版もバケット内の二分探索（`bisect`）と挿入・削除（`list.insert`・`pop`）は C で実装された組み込み関数で、インタプリタが遅いのはバケットを選ぶ部分くらいしかない。そのため倍率は10倍強に留まる。これ以上速くするには、バケットの末尾の値を別の配列に持つ（`T: Clone` が必要になる）か、別方式に差し替える必要がある。

## 要件との対応

| マイルストーン | 状況 |
|---|---|
| M1 骨格 | 完了 |
| M2 全API | 完了。差分テスト（10^6 操作 × 3シード × 4パターン）と不変条件検査で違反ゼロ |
| M3 計測 | 完了。上のベンチマークとプロファイル |
| M4 実戦 | 完了。ABC241 D「Sequence Query」（[`examples/abc241_d.rs`](examples/abc241_d.rs)）を Rust (rustc 1.89.0) で提出し、[AC](https://atcoder.jp/contests/abc241/submissions/79447594)（全29ケース、実行時間 35ms、メモリ 10412KiB） |

要件定義書の完成の定義（3章の全 API が差分テストを通り、4章の性能目標を満たし、AtCoder の実問題で1問以上 AC）をすべて満たした。

未決事項:

- [x] AtCoder の rustc バージョン: [公式の言語一覧（2025/10 ジャッジ更新）](https://img.atcoder.jp/file/language-update/2025-10/language-list.html)で「Rust (rustc 1.89.0)」を確認した
- [x] 検索 API の命名: `lt`/`le`/`gt`/`ge` のみで確定。`lower_bound`/`upper_bound` は取り違えやすく、別名は API と貼り付けの行数を増やすだけなので用意しない
- [x] Python 版の定数: [tatyam-prime/SortedSet](https://github.com/tatyam-prime/SortedSet) の最新（コミット `9a205c6`）で `BUCKET_RATIO = 16`, `SPLIT_RATIO = 24`。Rust 版では上のとおり調整した
- [x] 実戦で提出する過去問: ABC241 D「Sequence Query」で AC

## ライセンス

[CC0-1.0](LICENSE)。競プロのライブラリは提出コードに貼り付けて使うので、MIT などのように貼り付け先に著作権表示とライセンス文を含める必要が出ないようにした。AtCoder Library や ac-library-rs（CC0-1.0）、tatyam 氏の Python 版（Unlicense）と同じ扱い。

`bench/tatyam_sorted_multiset.py` は tatyam 氏の実装（Unlicense）をそのまま取り込んだもの。
