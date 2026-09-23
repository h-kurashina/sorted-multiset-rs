"""ベンチマーク用の操作列を生成する。

使い方: python3 bench/gen_ops.py <操作数> <シード> [値域] [mixed|rank] > ops.txt

mixed（既定）は各種の操作を混ぜた操作列。rank はバケットの長さを先頭から足し合わせる
nth・index 系の操作を6割にした操作列で、バケット数を増やしたときの劣化を見るのに使う。

1行目に操作数 Q、続く Q 行に「op 引数」を書く。op の意味は bench/ops.md を参照。
"""
import random
import sys

# (op, 重み): insert と削除を多めにし、残りを各種の問い合わせに振り分ける
WEIGHTS = [
    (0, 35),  # insert x
    (1, 15),  # remove_one x
    (2, 10),  # nth k
    (3, 10),  # ge x
    (4, 3),   # gt x
    (5, 3),   # le x
    (6, 5),   # lt x
    (7, 5),   # index x
    (8, 3),   # index_right x
    (9, 3),   # count x
    (10, 3),  # contains x
    (11, 2),  # pop_nth k
    (12, 2),  # pop_first
    (13, 1),  # pop_last
]

RANK_WEIGHTS = [
    (0, 35),  # insert x
    (1, 5),   # remove_one x
    (2, 25),  # nth k
    (7, 15),  # index x
    (8, 10),  # index_right x
    (9, 5),   # count x
    (11, 5),  # pop_nth k
]


def main() -> None:
    q = int(sys.argv[1])
    rng = random.Random(int(sys.argv[2]))
    value_range = int(sys.argv[3]) if len(sys.argv) > 3 else 10**9
    mix = sys.argv[4] if len(sys.argv) > 4 else "mixed"
    table = {"mixed": WEIGHTS, "rank": RANK_WEIGHTS}[mix]
    ops = [op for op, _ in table]
    weights = [w for _, w in table]
    size = 0  # 要素数の概算（k の範囲を決めるためだけに使う）
    out = [str(q)]
    for op in rng.choices(ops, weights, k=q):
        if op in (2, 11):
            out.append(f"{op} {rng.randrange(size + 2)}")
        elif op in (12, 13):
            out.append(str(op))
            size = max(size - 1, 0)
        else:
            out.append(f"{op} {rng.randrange(value_range)}")
        if op == 0:
            size += 1
        elif op in (1, 11):
            size = max(size - 1, 0)
    sys.stdout.write("\n".join(out) + "\n")


if __name__ == "__main__":
    main()
