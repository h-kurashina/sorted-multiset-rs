"""操作列を tatyam 氏の Python 版 SortedMultiset で実行する。

使い方: python3 bench/run_python.py < ops.txt > out.txt
処理時間（入力の読み込みと出力の書き込みを除く）をミリ秒で標準エラーに出す。
"""
import os
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from tatyam_sorted_multiset import SortedMultiset  # noqa: E402


def main() -> None:
    data = sys.stdin.buffer.read().split(b"\n")
    q = int(data[0])
    lines = data[1 : q + 1]
    start = time.perf_counter()
    s = SortedMultiset()
    out = []
    push = out.append
    for line in lines:
        parts = line.split()
        op = int(parts[0])
        if op == 0:
            s.add(int(parts[1]))
        elif op == 1:
            push(1 if s.discard(int(parts[1])) else 0)
        elif op == 2:
            k = int(parts[1])
            push(s[k] if k < len(s) else -1)
        elif op <= 6:
            x = int(parts[1])
            r = (s.ge, s.gt, s.le, s.lt)[op - 3](x)
            push(-1 if r is None else r)
        elif op == 7:
            push(s.index(int(parts[1])))
        elif op == 8:
            push(s.index_right(int(parts[1])))
        elif op == 9:
            push(s.count(int(parts[1])))
        elif op == 10:
            push(1 if int(parts[1]) in s else 0)
        elif op == 11:
            k = int(parts[1])
            push(s.pop(k) if k < len(s) else -1)
        elif op == 12:
            push(s.pop(0) if len(s) else -1)
        else:
            push(s.pop(-1) if len(s) else -1)
    text = "\n".join(map(str, out))
    elapsed = (time.perf_counter() - start) * 1000
    sys.stdout.write(text + "\n")
    print(f"{elapsed:.1f}", file=sys.stderr)


if __name__ == "__main__":
    main()
