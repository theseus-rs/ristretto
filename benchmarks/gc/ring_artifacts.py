"""Offline artifact and result checks for the finite ring comparison harness."""
import json
from pathlib import Path
from canonical_service import digest, fields, integer, unique_object, workload_digest
from canonical_service_artifacts import read_json, verify_build
from ring_comparison import validate_admission, validate_plan, validate_canonical, validate_peer, validate_warmup_check
from process_resources import validate_resources as validate_process_resources


def process_resources(root, run):
    """Accept legacy runs without retrospective metrics; bind new receipts exactly."""
    names = ("schema_version run_id mode host_process_id exit_code command launcher_sha256 "
             "admission_sha256 workload_sha256 host_executable_sha256")
    version = run.get("schema_version")
    if type(version) is not int or version not in (1, 2):
        raise ValueError("unsupported run schema")
    if version == 2:
        names += " controller_sha256 resource_observer_sha256 process_resources_sha256 launch_plan_sha256"
    fields(run, names, "run receipt")
    if version == 1:
        if "benchmarks/gc/process_resources.py" in read_json(root / "source-manifest.json"):
            raise ValueError("resource-aware source requires a schema-2 run")
        if (root / "process-resources.json").exists():
            raise ValueError("legacy run cannot acquire unbound resource metrics")
        return None
    manifest = read_json(root / "source-manifest.json")
    for field, saved, source in (("controller_sha256", "controller.py", "run_ring_sample.py"),
                                 ("resource_observer_sha256", "resource-observer.py", "process_resources.py")):
        if digest(root / saved) != run[field] or run[field] != manifest.get("benchmarks/gc/" + source):
            raise ValueError("resource controller/source binding differs")
    if digest(root / "process-resources.json") != run["process_resources_sha256"]:
        raise ValueError("resource observation changed")
    if digest(root / "launch-plan.json") != run["launch_plan_sha256"]:
        raise ValueError("frozen launch plan changed")
    launch_plan = read_json(root / "launch-plan.json")
    if launch_plan["run_id"] != run["run_id"] or launch_plan["mode"] != run["mode"]:
        raise ValueError("launch plan run identity differs")
    usage = validate_process_resources(read_json(root / "process-resources.json"),
                                       run["host_process_id"], run["exit_code"],
                                       run["run_id"], launch_plan["watchdog_seconds"])
    if usage["timed_out"]:
        raise ValueError("child watchdog expired")
    return usage


def packet(path, prefix):
    rows = [line[len(prefix):] for line in Path(path).read_text().splitlines() if line.startswith(prefix)]
    if len(rows) != 1:
        raise ValueError("expected one result packet")
    return json.loads(rows[0], object_pairs_hook=unique_object)


def with_process_resources(root, result):
    usage = process_resources(root, read_json(root / "run.json"))
    if usage is not None:
        result["process_resources"] = usage
    return result


def resources(root, mode):
    actual = read_json(root / "resources.json")
    fields(actual, "enforced cpu_affinity cgroup cpu_max memory_max pids_max cgroup_mount cgroup_root cgroup_membership", "actual resources")
    request = read_json(root / "launch-plan.json")["resources"]
    if request is None:
        if mode not in {"check", "scheduled-check"} or actual["enforced"] is not False:
            raise ValueError("measurements require enforced resources")
    else:
        validate_resources(request)
        if (actual["enforced"] is not True or actual["cpu_affinity"] != request["cpu_affinity"]
                or actual["cpu_max"] != f'{request["cpu_quota_us"]} {request["cpu_period_us"]}'
                or actual["memory_max"] != str(request["memory_max_bytes"])
                or actual["pids_max"] != str(request["pids_max"])):
            raise ValueError("actual resources differ from frozen envelope")
        import os
        point, group = Path(actual["cgroup_mount"]), Path(actual["cgroup"])
        if (not group.is_absolute() or not point.is_absolute() or not group.is_relative_to(point)
                or actual["cgroup_membership"] != os.path.normpath(str(Path(actual["cgroup_root"]) / group.relative_to(point)))):
            raise ValueError("resource receipt has no matching kernel cgroup membership")
    return actual


def validate_resources(request):
    fields(request, "cpu_affinity cpu_quota_us cpu_period_us memory_max_bytes pids_max", "resource envelope")
    cpus = request["cpu_affinity"]
    if not isinstance(cpus, list) or not cpus or cpus != sorted(set(cpus)):
        raise ValueError("CPU affinity must be a sorted unique nonempty list")
    for cpu in cpus:
        integer(cpu, "CPU index")
    for key in ("cpu_quota_us", "cpu_period_us", "memory_max_bytes", "pids_max"):
        integer(request[key], key, 1)
    if request["cpu_quota_us"] > request["cpu_period_us"] * len(cpus):
        raise ValueError("CPU quota exceeds admitted affinity capacity")
    return request


def verify_shared_classes(root, manifest):
    names = ["CanonicalRingWorkload.class", "CanonicalRingWorkload$Node.class"]
    classes = root / "classes"
    expected_names = set(names)
    if (classes / "CanonicalRingPeer.class").exists():
        expected_names.add("CanonicalRingPeer.class")
    if {path.relative_to(classes).as_posix() for path in classes.rglob("*.class")} != expected_names:
        raise ValueError("unexpected classfiles in ring classpath")
    for name in names:
        if digest(classes / name) != manifest["ristretto_vm/tests/fixtures/canonical/" + name]:
            raise ValueError("consumed workload bytecode differs from frozen ring fixture")


def check_canonical(directory):
    root = Path(directory)
    identity, admission = expected_identity(root)
    plan = validate_plan(read_json(root / "sample-plan.json"))
    if plan["run_id"] != identity["run_id"]:
        raise ValueError("sample/launch run identity differs")
    if plan["mode"] in {"measure", "scheduled-measure"} and "--release" not in read_json(root / "build.json")["command"]:
        raise ValueError("timing comparison requires a release build")
    verify_shared_classes(root, read_json(root / "source-manifest.json"))
    resources(root, plan["mode"])
    if admission["schema_version"] == 3:
        from scheduled_ring_comparison import validate_canonical as scheduled_canonical
        result = scheduled_canonical(packet(root / "stdout.txt", "RISTRETTO_RING_SCHEDULED "), identity, admission, plan)
    elif admission["schema_version"] == 2:
        from shared_ring_comparison import validate_shared
        result = validate_shared(packet(root / "stdout.txt", "RISTRETTO_CANONICAL_RING "), identity, admission, plan)
    elif plan["mode"] == "warmup-check":
        result = validate_warmup_check(packet(root / "stdout.txt", "RISTRETTO_RING_WARMUP_CHECK "), identity, admission["expected_result"], plan, True)
    else:
        result = validate_canonical(packet(root / "stdout.txt", "RISTRETTO_CANONICAL_RING "), identity, admission, plan)
    return with_process_resources(root, result)


def verify_peer_build(directory):
    root = Path(directory)
    receipt = read_json(root / "peer-build.json")
    fields(receipt, "schema_version jdk jdk_sha256 command peer_source_sha256 peer_class_sha256 shared_workload_sha256 "
           "source_archive_sha256 source_manifest_sha256 qualification", "peer build")
    if type(receipt["schema_version"]) is not int or receipt["schema_version"] != 1 or receipt["qualification"] is not False:
        raise ValueError("unsupported peer build receipt")
    for key, filename in (("source_archive_sha256", "sources.tar.gz"), ("source_manifest_sha256", "source-manifest.json"),
                          ("peer_class_sha256", "classes/CanonicalRingPeer.class")):
        if digest(root / filename) != receipt[key]:
            raise ValueError("peer build artifact differs")
    import tarfile
    import hashlib
    manifest = read_json(root / "source-manifest.json")
    seen = set()
    with tarfile.open(root / "sources.tar.gz") as archive:
        for member in archive:
            if (not member.isfile() or member.name not in manifest or member.name in seen
                    or Path(member.name).is_absolute() or ".." in Path(member.name).parts):
                raise ValueError("invalid peer source archive")
            seen.add(member.name)
            with archive.extractfile(member) as stream:
                if hashlib.file_digest(stream, "sha256").hexdigest() != manifest[member.name]:
                    raise ValueError("peer source member changed")
    if seen != set(manifest) or receipt["peer_source_sha256"] != manifest["benchmarks/gc/ring_compare/CanonicalRingPeer.java"]:
        raise ValueError("peer source manifest differs")
    verify_shared_classes(root, manifest)
    if workload_digest(root / "shared-classes") != receipt["shared_workload_sha256"]:
        raise ValueError("peer shared workload digest differs")
    for name in ("CanonicalRingWorkload.class", "CanonicalRingWorkload$Node.class"):
        if digest(root / "shared-classes" / name) != digest(root / "classes" / name):
            raise ValueError("peer classpath differs from bound workload")
    if set(receipt["jdk_sha256"]) != {"bin/java", "bin/javac", "lib/server/libjvm.so", "lib/modules", "release"}:
        raise ValueError("incomplete JDK artifact inventory")
    for name, value in receipt["jdk_sha256"].items():
        if digest(root / "jdk-artifacts" / name) != value:
            raise ValueError("preserved JDK artifact changed")
    return receipt


def check_peer(directory):
    root = Path(directory)
    build = verify_peer_build(root)
    admission = validate_admission(read_json(root / "admission.json"))
    plan = validate_plan(read_json(root / "sample-plan.json"), peer=True)
    run, launch = read_json(root / "run.json"), read_json(root / "launch.json")
    process_resources(root, run)
    fields(launch, "schema_version run_id mode host_process_id command host_executable_sha256 binary_sha256 launcher_sha256", "peer launch")
    if (type(launch["schema_version"]) is not int or launch["schema_version"] != 1
            or type(run["exit_code"]) is not int or run["exit_code"] != 0 or run["mode"] != "peer"):
        raise ValueError("peer process failed or unsupported receipts")
    integer(run["host_process_id"], "actual peer PID", 1)
    for key in ("run_id", "mode", "host_process_id", "command", "launcher_sha256", "host_executable_sha256"):
        if type(run[key]) != type(launch[key]) or run[key] != launch[key]:
            raise ValueError("actual Popen/exec peer receipts differ")
    if (run["run_id"] != plan["run_id"] or run["admission_sha256"] != digest(root / "admission.json")
            or run["workload_sha256"] != build["shared_workload_sha256"]
            or run["host_executable_sha256"] != build["jdk_sha256"]["bin/java"]
            or digest(root / "host-executable") != run["host_executable_sha256"]
            or launch["binary_sha256"] != build["peer_class_sha256"]
            or digest(root / "launcher.py") != run["launcher_sha256"]
            or run["launcher_sha256"] != read_json(root / "source-manifest.json")["benchmarks/gc/ring_launch.py"]):
        raise ValueError("peer executable/source/admission/bytecode identity differs")
    configuration = read_json(root / "peer-configuration.json")
    fields(configuration, "collector heap_bytes active_processors", "peer configuration")
    if configuration["collector"] not in {"G1", "Z"}:
        raise ValueError("unsupported collector")
    integer(configuration["heap_bytes"], "heap bytes", 1)
    if admission["schema_version"] in (2, 3) and configuration["heap_bytes"] != admission["max_payload_bytes"]:
        raise ValueError("peer heap differs from shared canonical payload ceiling")
    integer(configuration["active_processors"], "active processors", 1)
    expected = peer_command(root, Path(build["jdk"]), admission, plan, configuration)
    if run["command"] != expected:
        raise ValueError("actual peer command differs from frozen workload/resource configuration")
    actual = resources(root, plan["mode"])
    if actual["enforced"] and configuration["active_processors"] != len(actual["cpu_affinity"]):
        raise ValueError("HotSpot processor count differs from CPU envelope")
    if admission["schema_version"] == 3:
        from scheduled_ring_comparison import validate_peer as scheduled_peer
        result = scheduled_peer(packet(root / "stdout.txt", "RISTRETTO_RING_SCHEDULED_PEER "), admission, plan)
    elif plan["mode"] == "warmup-check":
        result = validate_warmup_check(packet(root / "stdout.txt", "RISTRETTO_RING_WARMUP_CHECK "), {"run_id": plan["run_id"]}, admission["expected_result"], plan, False)
    else:
        result = validate_peer(packet(root / "stdout.txt", "RISTRETTO_RING_PEER "), admission["method"], admission["argument"], plan)
    return with_process_resources(root, result)


def peer_command(root, jdk, admission, plan, configuration):
    command = [str(jdk / "bin/java"), f'-Xms{configuration["heap_bytes"]}', f'-Xmx{configuration["heap_bytes"]}',
               f'-XX:+Use{configuration["collector"]}GC', f'-XX:ActiveProcessorCount={configuration["active_processors"]}']
    if plan["mode"] in {"check", "allocation"}:
        command += ["-XX:+UnlockDiagnosticVMOptions", "-XX:+LogCompilation", f'-XX:LogFile={root / "compilation.xml"}',
                    f'-Xlog:gc*,safepoint:file={root / "gc.log"}:uptimenanos,level,tags']
    if plan.get("schema_version", 1) == 2:
        return command + ["-cp", str(root / "classes"), "CanonicalRingPeer", plan["run_id"], plan["mode"], admission["method"],
                          str(admission["argument"]), str(plan["warmup_invocations"]), str(plan["duration_seconds"]),
                          str(plan["arrival_rate"]), str(plan["max_completions"]), str(admission["expected_result"])]
    return command + ["-cp", str(root / "classes"), "CanonicalRingPeer", plan["run_id"], plan["mode"], admission["method"],
                      str(admission["argument"]), str(plan["warmup_invocations"]), str(plan["samples"]), str(admission["expected_result"])]


def expected_identity(directory):
    root = Path(directory)
    build = verify_build(root)
    admission = validate_admission(read_json(root / "admission.json"))
    run = read_json(root / "run.json")
    process_resources(root, run)
    launch = read_json(root / "launch.json")
    fields(launch, "schema_version run_id mode host_process_id command host_executable_sha256 "
           "binary_sha256 launcher_sha256", "exec receipt")
    if (type(launch["schema_version"]) is not int or launch["schema_version"] != 1
            or type(run["exit_code"]) is not int or run["exit_code"] != 0):
        raise ValueError("unsupported/failed launch")
    integer(run["host_process_id"], "actual Popen PID", 1)
    for key in ["run_id", "mode", "host_process_id", "command", "host_executable_sha256", "launcher_sha256"]:
        if run[key] != launch[key] or type(run[key]) != type(launch[key]):
            raise ValueError(f"Popen/exec receipts differ: {key}")
    if (run["admission_sha256"] != digest(root / "admission.json")
            or run["workload_sha256"] != workload_digest(root / "classes")
            or launch["binary_sha256"] != build["binary_sha256"]):
        raise ValueError("launched admission/classes/executable changed")
    command = run["command"]
    if not isinstance(command, list) or not all(isinstance(arg, str) for arg in command):
        raise ValueError("invalid exec arguments")
    if run["mode"] == "native":
        expected = [str((root / "program").resolve()), str(root.resolve())]
        if (command != expected or run["host_executable_sha256"] != build["binary_sha256"]
                or build["binary_name"] != "program"):
            raise ValueError("native command differs from actual executable/inputs")
        guest_pid = run["host_process_id"]
    elif run["mode"] == "wasi":
        expected_tail = ["run", "--dir", f"{root.resolve()}::/capture",
                         str((root / "program.wasm").resolve()), "/capture"]
        if (command[1:] != expected_tail or build["binary_name"] != "program.wasm"
                or build["target"] != "wasm32-wasip2"):
            raise ValueError("WASI command differs from module/guest inputs")
        guest_pid = None
    else:
        raise ValueError("unsupported launch mode")
    # The host executable is preserved independently so later host replacement
    # cannot silently change the command's identity at offline validation time.
    if digest(root / "host-executable") != run["host_executable_sha256"]:
        raise ValueError("preserved host executable differs")
    source_manifest = read_json(root / "source-manifest.json")
    if (digest(root / "launcher.py") != run["launcher_sha256"]
            or run["launcher_sha256"] != source_manifest.get("benchmarks/gc/ring_launch.py")):
        raise ValueError("launcher source differs from frozen build source")
    return {"run_id": run["run_id"], "mode": run["mode"], "tier": admission["tier"],
            "host_process_id": run["host_process_id"], "guest_process_id": guest_pid,
            "host_executable_sha256": run["host_executable_sha256"],
            "binary_sha256": build["binary_sha256"],
            "source_archive_sha256": build["source_archive_sha256"],
            "source_manifest_sha256": build["source_manifest_sha256"],
            "workload_sha256": run["workload_sha256"], "admission_sha256": run["admission_sha256"],
            "build_receipt_sha256": digest(root / "build.json"),
            "launch_receipt_sha256": digest(root / "launch.json"),
            "sample_plan_sha256": digest(root / "sample-plan.json"),
            "resource_receipt_sha256": digest(root / "resources.json")}, admission

