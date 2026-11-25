"""Validate conformance catalogs and expand them to one TSV record per test condition."""
import argparse
import csv
import itertools
import json
import re
import sys
import tomllib
from collections import Counter, defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent
TESTS = HERE.parents[1] / "tests"


def require(condition, message):
    if not condition:
        raise ValueError(message)


def values(selector, sets, parents=()):
    items = [selector] if isinstance(selector, str) else selector
    require(isinstance(items, list) and items and all(isinstance(s, str) and s for s in items),
            f"expected nonempty dimension values, got {items!r}")
    expanded = []
    for item in items:
        if item.startswith("$"):
            require(item not in parents, f"cyclic dimension set {item}")
            expanded.extend(values(sets[item[1:]], sets, parents + (item,)))
        else:
            expanded.append(item)
    require(len(expanded) == len(set(expanded)), f"duplicate dimension values in {selector}")
    return expanded


def evidence(fixture, location):
    filename, symbol = location.split("#")
    path = (fixture / filename).resolve()
    require(path.is_relative_to(fixture), f"evidence outside fixture: {location}")
    # Anchor on a method declaration, rather than a call or an unstable line number.
    pattern = rf"(?m)^\s*(?:[\w<>\[\].?,]+\s+)+{re.escape(symbol)}\s*\([^;{{}}]*\)\s*(?:throws [\w.,\s]+)?\{{"
    require(re.search(pattern, path.read_text()), f"missing method {path}#{symbol}")
    return f"{path.relative_to(TESTS).as_posix()}#{symbol}"


def catalog(path, data, ids):
    require(data["version"] == 1, f"unknown format in {path}")
    sets = data.get("sets", {})
    for name in sets:
        values("$" + name, sets)
    scope = (TESTS / data["suite"]).resolve()
    require(scope.is_relative_to(TESTS) and scope.is_dir(), f"invalid suite {scope}")
    for enum in data.get("enumeration", []):
        source = (scope / enum["source"]).resolve()
        require(source.is_relative_to(scope), f"enumeration outside suite: {source}")
        pattern = re.compile(enum["pattern"])
        require(pattern.groups == 1, f"{source}: enumeration needs one capture group")
        actual = set(pattern.findall(source.read_text()))
        declared = set(values("$" + enum["set"], sets))
        require(actual == declared, f"{source}: enumeration differs by {sorted(actual ^ declared)}")
    indexed = defaultdict(set)
    for case in data["case"]:
        key = case["id"]
        require(key not in ids, f"duplicate requirement ID {key}")
        ids.add(key)
        refs = [data["spec"][s] for s in case["spec"]]
        require(refs and all(s.startswith("https://") for s in refs), f"{key}: missing specification URL")
        status = case.get("status", "implemented")
        require(status in ("implemented", "planned"), f"{key}: unknown coverage status")
        versions = values(case["java"], sets)
        sources = []
        fixture_name = ""
        if status == "implemented":
            fixture = (scope / case["fixture"]).resolve()
            require(fixture.is_relative_to(scope) and (fixture / "Test.java").is_file(), f"{key}: invalid fixture")
            fixture_name = fixture.relative_to(TESTS).as_posix()
            sources = [evidence(fixture, entry) for entry in case["evidence"]]
            require(sources, f"{key}: missing executable evidence")
            indexed[fixture].update(versions)
        else:
            require("fixture" not in case and "evidence" not in case, f"{key}: planned case has executable evidence")
        axes = {k: values(v, sets) for k, v in case.get("axes", {}).items()}
        reserved = {"id", "status", "fixture", "java", "expected", "evidence", "spec", "fixture_int", "fixture_jit"}
        require(not reserved.intersection(axes), f"{key}: axis shadows a catalog column")
        axes = {"java": versions, **axes}
        rules = []
        shared = data["rules"][case["rules"]] if "rules" in case else []
        for rule in case.get("override", []) + shared:
            when = {k: values(v, sets) for k, v in rule["when"].items()}
            require(when and all(k in axes and set(vals) <= set(axes[k]) for k, vals in when.items()),
                    f"{key}: invalid condition {when}")
            rules.append((when, rule["expect"]))
        used = set()
        for combination in itertools.product(*axes.values()):
            point = dict(zip(axes, combination))
            expected = case["expect"]
            for i, (when, outcome) in enumerate(rules):
                if all(point[k] in vals for k, vals in when.items()):
                    expected = outcome
                    used.add(i)
                    break
            require(isinstance(expected, str) and expected.strip(), f"{key}: missing expected outcome")
            yield {"id": key, "status": status, "fixture": fixture_name, **point, "expected": expected,
                   "evidence": " ".join(sources), "spec": " ".join(refs)}
        require(len(used) == len(rules), f"{key}: unreachable expectation override")
    if data.get("inventory", False):
        for main in scope.rglob("Test.java"):
            require(main.parent in indexed, f"unindexed fixture {main.parent}")
    for fixture, releases in indexed.items():
        properties = fixture / "runtime.properties"
        if properties.exists():
            match = re.search(r"^java\.versions\s*=\s*(.+)$", properties.read_text(), re.M)
            require(match and releases <= {v.strip() for v in match[1].split(",")},
                    f"release coverage differs from {properties}")
            if data.get("inventory", False):
                require(releases == {v.strip() for v in match[1].split(",")}, f"incomplete inventory for {fixture}")
        else:
            require(releases == {"default"}, f"{fixture}: use default for the harness default JDK")


def diagnostics(message):
    # Rust prints the captured Java stderr inside a quoted InternalError payload.
    if message.startswith('InternalError("') and message.endswith('")'):
        try:
            payload = json.loads(message[len("InternalError("):-1])
            stderr = payload.partition("\nstderr: ")[2]
            if stderr.strip():
                return stderr.rstrip().splitlines()
        except ValueError:
            pass
    return [message]


def outcomes(logs):
    results = {}
    failures = {}
    fixture_name = r"([\w./-]+)(?:@java([\w.]+))?"
    started = re.compile(r"\bRunning test " + fixture_name + r"$")
    finished = re.compile(r"\b(Passed|Failed) \((int|jit)\) " + fixture_name + r"(?=[ :])")
    ansi = re.compile(r"\x1b\[[0-9;]*m")
    for path in logs:
        with path.open() as log:
            for raw in log:
                line = ansi.sub("", raw.rstrip())
                match = started.search(line)
                if match:
                    fixture, java = match.groups()
                    for engine in ("int", "jit"):
                        key = (fixture, java or "default", engine)
                        results.pop(key, None)
                        failures.pop(key, None)
                match = finished.search(line)
                if match:
                    state, engine, fixture, java = match.groups()
                    key = (fixture, java or "default", engine)
                    results[key] = state.lower()
                    if state == "Failed":
                        failures[key] = diagnostics(line[match.end():].removeprefix(": "))
    return results, failures


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    output = parser.add_mutually_exclusive_group()
    output.add_argument("--failures", action="store_true", help="export the selected fixtures' failure diagnostics as TSV")
    output.add_argument("--tsv", action="store_true", help="write expanded rows to standard output")
    parser.add_argument("--id", help="select a single requirement after validating all catalogs")
    parser.add_argument("--log", type=Path, action="append", default=[], help="compatibility harness log in run order")
    args = parser.parse_args()
    catalogs = [(path, tomllib.loads(path.read_text())) for path in sorted(HERE.glob("*.toml"))]
    ids = set()
    fixtures = set()
    dimensions = set()
    count = 0
    for path, data in catalogs:
        for row in catalog(path, data, ids):
            if args.id and row["id"] != args.id:
                continue
            count += 1
            dimensions.update(row)
            if row["fixture"]:
                fixtures.add((row["fixture"], row["java"]))
    if args.id:
        require(args.id in ids, f"unknown requirement {args.id}")
    require(count, "no coverage records")
    results, failures = outcomes(args.log)
    if args.failures:
        require(args.log, "--failures needs a compatibility harness --log")
        writer = csv.writer(sys.stdout, delimiter="\t", lineterminator="\n")
        writer.writerow(["fixture", "java", "engine", "diagnostic"])
        for (fixture, java, engine), messages in sorted(failures.items()):
            if (fixture, java) in fixtures:
                writer.writerows((fixture, java, engine, message) for message in messages)
    elif args.tsv:
        first = ["id", "status", "fixture", "java"]
        last = ["expected", "evidence", "spec", "fixture_int", "fixture_jit"]
        fields = first + sorted(dimensions.difference(first, last)) + last
        writer = csv.DictWriter(sys.stdout, fields, delimiter="\t", lineterminator="\n")
        writer.writeheader()
        # Validate before emitting any rows, then stream the Cartesian products.
        ids.clear()
        for path, data in catalogs:
            for row in catalog(path, data, ids):
                if args.id and row["id"] != args.id:
                    continue
                for engine in ("int", "jit"):
                    row["fixture_" + engine] = results.get((row["fixture"], row["java"], engine), "unobserved")
                writer.writerow(row)
    else:
        print(f"{1 if args.id else len(ids)} requirements, {count} conditions, {len(fixtures)} fixture/JDK pairs")
        if args.log:
            print(dict(Counter(results.get((f, v, e), "unobserved") for f, v in fixtures for e in ("int", "jit"))))


if __name__ == "__main__":
    try:
        main()
    except (KeyError, OSError, ValueError) as error:
        sys.exit(str(error))
