"""Independent scalar checksum/fault model for the finite Java reference fixture."""
from pathlib import Path
import sys

def signed(value):
    value &= (1 << 32) - 1
    return value if value < (1 << 31) else value - (1 << 32)

def expected(method, n):
    if method == "nodes":
        total = 0
        for index in range(n):
            total = signed(total * 31 + index + 3)
        return str(signed(total + n + 2) if n else total)
    if method == "mixed":
        return str(n + ((n + 1) // 2) * 9 + (n // 2) * 17)
    if method == "returned":
        return str(n)
    if method == "failNegative":
        return "java.lang.NegativeArraySizeException"
    if method in {"failBounds", "failStoreBounds"}:
        return "java.lang.ArrayIndexOutOfBoundsException"
    if method in {"failNull", "failNullStore", "failNullLength"}:
        return "java.lang.NullPointerException"
    if method == "failType":
        return "java.lang.ArrayStoreException"
    raise ValueError(method)

if __name__ == "__main__":
    rows = [line.split("\t") for line in Path(sys.argv[1]).read_text().splitlines()]
    assert len(rows) == 34 and len({(m, n) for m, n, _ in rows}) == 34
    for method, argument, value in rows:
        assert expected(method, int(argument)) == value, (method, argument, value)
    print("34 HotSpot rows match independent scalar/fault oracle")
