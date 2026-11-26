"""One-owner child reaping with Linux lifetime rusage, never sampled RSS.

wait4 includes the launched process and accounting for descendants it reaped.
RSS is a maximum, not a sum or a measurement-window statistic. See
PROCESS_RESOURCES.md for the exec/launcher and Wasmtime scope.
"""
import math
import os
import selectors
import signal
import subprocess
import sys

from canonical_service import fields, integer

SCOPE = "linux_wait4_child_lifetime_including_reaped_descendants"
FIELDS = ("schema_version run_id host_process_id exit_code timed_out timeout_seconds "
          "available unavailable_reason scope raw_wait_status peak_rss_kib peak_rss_bytes "
          "user_cpu_seconds system_cpu_seconds minor_faults major_faults "
          "voluntary_context_switches involuntary_context_switches "
          "measurement_window_only process_tree_total guest_only")


def supported():
    return (sys.platform == "linux" and hasattr(os, "wait4")
            and hasattr(os, "pidfd_open") and hasattr(signal, "pidfd_send_signal"))


def wait_resources(process, timeout, *, run_id):
    """Own all reaping of a fresh Popen with file-backed output and no other waiter.

    The pidfd blocks until exit or a single watchdog deadline. No periodic parent
    polling, sampling thread or child instrumentation is introduced. Setting the
    Popen returncode after wait4 prevents a second reap by its destructor.
    """
    if type(timeout) not in (int, float) or not math.isfinite(timeout) or timeout <= 0:
        raise ValueError("watchdog timeout must be finite and positive")
    if not isinstance(run_id, str) or not run_id:
        raise ValueError("run identity must be a nonempty string")
    if process.returncode is not None:
        raise ValueError("process was already reaped")
    value = dict.fromkeys(FIELDS.split())
    value.update(schema_version=1, run_id=run_id, host_process_id=process.pid, timeout_seconds=timeout,
                 timed_out=False, available=False, measurement_window_only=False,
                 process_tree_total=False, guest_only=False)
    if not supported():
        try:
            process.wait(timeout=timeout)
        except subprocess.TimeoutExpired:
            value["timed_out"] = True
            process.kill()
            process.wait()
        value.update(exit_code=process.returncode,
                     unavailable_reason="Linux wait4/pidfd APIs are unavailable")
        return value
    # Hold the fd until after reaping; timeout signaling cannot hit a reused PID.
    descriptor = None
    try:
        descriptor = os.pidfd_open(process.pid)
        with selectors.DefaultSelector() as selector:
            selector.register(descriptor, selectors.EVENT_READ)
            if not selector.select(timeout):
                value["timed_out"] = True
                try:
                    signal.pidfd_send_signal(descriptor, signal.SIGKILL)
                except ProcessLookupError:
                    pass  # Exit raced the deadline; the watchdog still expired.
        pid, status, usage = os.wait4(process.pid, 0)
        if pid != process.pid:
            raise RuntimeError("wait4 returned another process")
        process.returncode = os.waitstatus_to_exitcode(status)
    except BaseException:
        # No valid receipt after a reaping/control failure. Clean up our child;
        # a successful wait4 above already set returncode, so do not reap twice.
        if process.returncode is None:
            if descriptor is not None:
                try:
                    signal.pidfd_send_signal(descriptor, signal.SIGKILL)
                except ProcessLookupError:
                    pass
            else:
                process.kill()
            process.wait()
        raise
    finally:
        if descriptor is not None:
            os.close(descriptor)
    value.update(available=True, exit_code=process.returncode, scope=SCOPE,
                 raw_wait_status=status, peak_rss_kib=usage.ru_maxrss,
                 peak_rss_bytes=usage.ru_maxrss * 1024,
                 user_cpu_seconds=usage.ru_utime, system_cpu_seconds=usage.ru_stime,
                 minor_faults=usage.ru_minflt, major_faults=usage.ru_majflt,
                 voluntary_context_switches=usage.ru_nvcsw,
                 involuntary_context_switches=usage.ru_nivcsw)
    return validate_resources(value, process.pid, process.returncode, run_id, timeout)


def validate_resources(value, pid, exit_code, run_id, expected_timeout):
    fields(value, FIELDS, "child lifetime resources")
    if not isinstance(run_id, str) or not run_id or value["run_id"] != run_id:
        raise ValueError("resource receipt run identity differs")
    integer(pid, "expected child PID", 1)
    if type(exit_code) is not int:
        raise ValueError("expected child exit status must be an integer")
    integer(value["host_process_id"], "resource child PID", 1)
    if (type(value["schema_version"]) is not int or value["schema_version"] != 1
            or value["host_process_id"] != pid or type(value["exit_code"]) is not int
            or value["exit_code"] != exit_code):
        raise ValueError("resource receipt PID/exit/schema differs")
    if (type(expected_timeout) not in (float, int) or not math.isfinite(expected_timeout)
            or expected_timeout <= 0 or value["timeout_seconds"] != expected_timeout):
        raise ValueError("resource watchdog differs from frozen request")
    timeout = value["timeout_seconds"]
    if type(timeout) not in (float, int) or not math.isfinite(timeout) or timeout <= 0:
        raise ValueError("invalid resource watchdog")
    if type(value["timed_out"]) is not bool or type(value["available"]) is not bool:
        raise ValueError("invalid resource availability/watchdog flag")
    if any(value[key] is not False for key in ("measurement_window_only", "process_tree_total", "guest_only")):
        raise ValueError("invalid resource scope claim")
    measured = ("scope raw_wait_status peak_rss_kib peak_rss_bytes user_cpu_seconds system_cpu_seconds "
                "minor_faults major_faults voluntary_context_switches involuntary_context_switches").split()
    if not value["available"]:
        if (not isinstance(value["unavailable_reason"], str) or not value["unavailable_reason"]
                or any(value[key] is not None for key in measured)):
            raise ValueError("unavailable resource observation must have null metrics")
        return value
    if value["scope"] != SCOPE or value["unavailable_reason"] is not None:
        raise ValueError("invalid Linux resource scope")
    for key in ("raw_wait_status", "peak_rss_kib", "peak_rss_bytes", "minor_faults", "major_faults",
                "voluntary_context_switches", "involuntary_context_switches"):
        integer(value[key], key)
    if (os.waitstatus_to_exitcode(value["raw_wait_status"]) != exit_code
            or value["peak_rss_bytes"] != value["peak_rss_kib"] * 1024
            or value["peak_rss_bytes"] > (1 << 64) - 1):
        raise ValueError("resource status or RSS units differ")
    for key in ("user_cpu_seconds", "system_cpu_seconds"):
        if type(value[key]) not in (int, float) or not math.isfinite(value[key]) or value[key] < 0:
            raise ValueError("invalid CPU time")
    return value
