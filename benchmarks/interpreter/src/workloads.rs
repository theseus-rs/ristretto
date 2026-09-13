//! IDs must match InterpreterBench.run's Java switch.

#[derive(Clone, Copy, Debug)]
pub struct Workload {
    pub id: usize,
    pub name: &'static str,
    pub category: &'static str,
    pub work: &'static str,
}

macro_rules! workloads {
    ($(($id:literal, $name:literal, $category:literal, $work:literal)),* $(,)?) => {
        pub const WORKLOADS: &[Workload] = &[
            $(Workload { id: $id, name: $name, category: $category, work: $work }),*
        ];
    };
}

workloads![
    (0, "control", "control", "one empty kernel"),
    (1, "int_arithmetic", "arithmetic", "256 integer recurrences"),
    (2, "long_arithmetic", "arithmetic", "256 long recurrences"),
    (3, "float_arithmetic", "arithmetic", "256 float recurrences"),
    (
        4,
        "double_arithmetic",
        "arithmetic",
        "256 double recurrences"
    ),
    (
        5,
        "conversions",
        "arithmetic",
        "256 mixed primitive conversions"
    ),
    (
        6,
        "branch_predictable",
        "control flow",
        "1024 mostly taken branches and array reads"
    ),
    (
        7,
        "branch_random",
        "control flow",
        "1024 data-dependent branches and array reads"
    ),
    (8, "tableswitch", "control flow", "256 dense switches"),
    (9, "lookupswitch", "control flow", "256 sparse switches"),
    (10, "static_call", "method dispatch", "256 static calls"),
    (
        11,
        "virtual_call",
        "method dispatch",
        "256 bimorphic virtual calls"
    ),
    (
        12,
        "interface_call",
        "method dispatch",
        "256 bimorphic interface calls"
    ),
    (
        13,
        "recursion",
        "method dispatch",
        "one recursive Fibonacci(12 + (seed & 1))"
    ),
    (
        14,
        "array_sequential",
        "memory",
        "1024 sequential int reads"
    ),
    (
        15,
        "array_random",
        "memory",
        "1024 indexed int reads; 4 KiB data + 4 KiB indices"
    ),
    (
        16,
        "object_fields",
        "memory",
        "256 field read/modify/write iterations"
    ),
    (
        17,
        "allocate_objects",
        "allocation",
        "64 linked nodes allocated and traversed"
    ),
    (
        18,
        "allocate_arrays",
        "allocation",
        "one int[256] allocated, filled and summed"
    ),
    (
        19,
        "exception_reused",
        "exceptions",
        "16 throws/catches of an existing exception"
    ),
    (
        20,
        "exception_fresh",
        "exceptions",
        "16 new exceptions, stack captures and throws/catches"
    ),
    (
        21,
        "monitor",
        "synchronization",
        "256 uncontended synchronized blocks"
    ),
    (
        22,
        "string_scan",
        "library",
        "one 55-character String scan and rolling hash"
    ),
    (
        23,
        "string_builder",
        "library",
        "16 String/int appends and toString"
    ),
    (
        24,
        "arraycopy",
        "library",
        "one 4 KiB System.arraycopy and one read"
    ),
    (
        25,
        "array_sort",
        "algorithm/library",
        "clone, sort and checksum 1024 ints"
    ),
    (
        26,
        "hashmap",
        "library",
        "64 boxed puts + 64 boxed gets in a fresh HashMap"
    ),
    (
        27,
        "arraylist",
        "library",
        "256 boxed adds + 256 gets in a fresh ArrayList"
    ),
    (
        28,
        "sieve",
        "algorithm",
        "one sieve below 4096 and prime sum"
    ),
    (
        29,
        "matrix_multiply",
        "algorithm",
        "one 16x16 integer matrix product (4096 multiply-adds)"
    ),
];
