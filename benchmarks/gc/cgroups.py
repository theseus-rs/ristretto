"""Per-process cgroup v2 ceilings. Requires an explicitly delegated, empty parent.

Does not change the parent's controllers or move any existing process. A missing
delegation is an error, never a fallback to an unenforced memory budget.
"""
from pathlib import Path
import os
import time
import uuid


class Budget:
    def __init__(self, parent, memory_bytes, cpu_quota_us):
        parent = Path(parent).resolve()
        if memory_bytes <= 0 or cpu_quota_us <= 0:
            raise ValueError("CPU and memory limits must be positive")
        enabled = set((parent / "cgroup.subtree_control").read_text().split())
        if not {"cpu", "memory"} <= enabled:
            raise ValueError("delegated parent must already enable cpu and memory controllers")
        self.path = parent / f"ristretto-gc-{os.getpid()}-{uuid.uuid4().hex}"
        self.path.mkdir()
        try:
            (self.path / "memory.max").write_text(str(memory_bytes))
            (self.path / "memory.swap.max").write_text("0")
            (self.path / "cpu.max").write_text(f"{cpu_quota_us} 100000")
            self.limits = {name: (self.path / name).read_text().strip()
                           for name in ["memory.max", "memory.swap.max", "cpu.max"]}
            if self.limits != {"memory.max": str(memory_bytes), "memory.swap.max": "0",
                               "cpu.max": f"{cpu_quota_us} 100000"}:
                raise ValueError("cgroup ceilings were not applied exactly")
        except BaseException:
            self.path.rmdir()
            raise

    def join(self):
        # Called in the single-threaded launcher's child before exec. Descendants
        # inherit this cgroup, including HotSpot's compiler and GC workers.
        (self.path / "cgroup.procs").write_text("0")

    def snapshot(self):
        def counters(name):
            return {key: int(value) for key, value in
                    (line.split() for line in (self.path / name).read_text().splitlines())}
        return {"limits": self.limits, "memory_events": counters("memory.events"),
                "cpu_stat": counters("cpu.stat"),
                "memory_peak": int((self.path / "memory.peak").read_text())}

    def close(self):
        # Also clean descendants of a program that exited before its children.
        (self.path / "cgroup.kill").write_text("1")
        deadline = time.monotonic() + 5
        while "populated 1" in (self.path / "cgroup.events").read_text():
            if time.monotonic() >= deadline:
                raise TimeoutError(f"cgroup descendants did not exit: {self.path}")
            time.sleep(.01)
        self.path.rmdir()
