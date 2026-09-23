# 操作列の形式

1行目に操作数 Q、続く Q 行に `op 引数` を書く。各実行系は、insert 以外の操作ごとに結果を1行出力する。
値は0以上なので、該当なし（`None`・範囲外）は `-1`、真偽値は `1`/`0` で出す。

| op | 操作 | 出力 |
|---|---|---|
| 0 x | insert(x) | なし |
| 1 x | remove_one(x) | 1/0 |
| 2 k | nth(k) | 値 or -1 |
| 3 x | ge(x) | 値 or -1 |
| 4 x | gt(x) | 値 or -1 |
| 5 x | le(x) | 値 or -1 |
| 6 x | lt(x) | 値 or -1 |
| 7 x | index(x) | 個数 |
| 8 x | index_right(x) | 個数 |
| 9 x | count(x) | 個数 |
| 10 x | contains(x) | 1/0 |
| 11 k | pop_nth(k) | 値 or -1 |
| 12 | pop_first() | 値 or -1 |
| 13 | pop_last() | 値 or -1 |
