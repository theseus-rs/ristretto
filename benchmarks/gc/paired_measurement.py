"""File identity and paired process timing statistics for GC screens."""
import hashlib
import random
import statistics


def sha256(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def paired_interval(pairs, seed=42, draws=10000):
    """Median of paired ratios; resample independent process pairs, not operations."""
    if len(pairs) < 2 or any(a <= 0 or b <= 0 for a, b in pairs):
        raise ValueError("need at least two positive independent pairs")
    ratios = [b / a for a, b in pairs]
    rng = random.Random(seed)
    samples = sorted(statistics.median(rng.choices(ratios, k=len(ratios))) for _ in range(draws))
    return {"median": statistics.median(ratios), "low": samples[int(draws * .025)],
            "high": samples[min(draws - 1, int(draws * .975))], "pairs": len(pairs)}
