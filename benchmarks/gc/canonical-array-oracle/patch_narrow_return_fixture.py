#!/usr/bin/env python3
"""Build a verified ireturn fixture from javac output without changing descriptors.

Each helper becomes iload_0; ireturn. Java permits these int operand-stack values
for B/C/S/Z return descriptors; ireturn itself must perform the narrowing. All
other class bytes remain unchanged. The caller methods are ordinary javac code.
"""
import pathlib
import struct
import sys


class Reader:
    def __init__(self, data):
        self.data = data
        self.offset = 0

    def take(self, size):
        end = self.offset + size
        if end > len(self.data):
            raise ValueError("truncated classfile")
        result = self.data[self.offset:end]
        self.offset = end
        return result

    def u1(self):
        return self.take(1)[0]

    def u2(self):
        return struct.unpack(">H", self.take(2))[0]

    def u4(self):
        return struct.unpack(">I", self.take(4))[0]


def patch(data):
    reader = Reader(data)
    if reader.take(4) != b"\xca\xfe\xba\xbe":
        raise ValueError("not a classfile")
    reader.take(4)
    cp_count = reader.u2()
    utf8 = {}
    index = 1
    while index < cp_count:
        tag = reader.u1()
        if tag == 1:
            utf8[index] = reader.take(reader.u2()).decode("utf-8")
        elif tag in (3, 4):
            reader.take(4)
        elif tag in (5, 6):
            reader.take(8)
            index += 1
        elif tag in (7, 8, 16, 19, 20):
            reader.take(2)
        elif tag in (9, 10, 11, 12, 17, 18):
            reader.take(4)
        elif tag == 15:
            reader.take(3)
        else:
            raise ValueError(f"unsupported constant tag {tag}")
        index += 1
    reader.take(6)
    reader.take(2 * reader.u2())
    if reader.u2() != 0:
        raise ValueError("fixture must have no fields")
    method_count = reader.u2()
    prefix = data[:reader.offset]
    methods = []
    expected = {"byteValue": "(I)B", "charValue": "(I)C", "shortValue": "(I)S", "booleanValue": "(I)Z"}
    patched = set()
    for _ in range(method_count):
        start = reader.offset
        flags = reader.u2()
        name = utf8[reader.u2()]
        descriptor = utf8[reader.u2()]
        attribute_count = reader.u2()
        header = data[start:reader.offset]
        attributes = []
        for _ in range(attribute_count):
            attribute_index = reader.u2()
            payload = reader.take(reader.u4())
            if name in expected and utf8[attribute_index] == "Code":
                if descriptor != expected[name] or not flags & 0x0008 or name in patched:
                    raise ValueError("unexpected helper definition")
                # max_stack=1, max_locals=1, code_length=2, no handlers or nested attributes.
                payload = struct.pack(">HHI", 1, 1, 2) + b"\x1a\xac" + struct.pack(">HH", 0, 0)
                patched.add(name)
            attributes.append(struct.pack(">HI", attribute_index, len(payload)) + payload)
        methods.append(header + b"".join(attributes))
    if patched != set(expected):
        raise ValueError("missing helper methods")
    return prefix + b"".join(methods) + data[reader.offset:]


if __name__ == "__main__":
    if len(sys.argv) != 3:
        raise SystemExit("usage: patch_narrow_return_fixture.py JAVAC_CLASS PATCHED_CLASS")
    source, target = map(pathlib.Path, sys.argv[1:])
    if source.resolve() == target.resolve():
        raise SystemExit("keep the original javac class separate")
    target.write_bytes(patch(source.read_bytes()))
