# JVM conformance matrix

The chapter catalogs follow the [Java SE 25 JVMS table of contents][jvms]. Each file includes the
published sections for that chapter, expected behavior, and links to fixtures where available.

- [1. Introduction and preview features](jvms-1-intro.toml)
- [2. Structure of the virtual machine](jvms-2-structure.toml)
- [3. Compiling for the virtual machine](jvms-3-compiling.toml)
- [4. Class-file format and verification](jvms-4-class-file.toml)
- [5. Loading, linking, and initialization](jvms-5-linking.toml)
- [6. Instruction execution and exceptions](jvms-6-instructions.toml)
- [7. Opcode encodings](jvms-7-opcodes.toml)

For example, `JVMS.4.VERIFY.IADD` describes the verifier's operand types, `JVMS.6.IADD` describes
the arithmetic result, and `JVMS.7.IADD` identifies the encoded byte. The three cases can have
different coverage. A `planned` case records an expected result awaiting executable evidence.
Chapter 3 uses example program behavior as its oracle. Compilers may emit different bytecode for
the same expression.

In these files, `[context]` identifies overviews and informative material. The `[anchors]` lists
index the specification's named passages under each section. Append a suffix to the section's URL
fragment to locate a passage. For a chapter URL, start with `jvms-N`, where `N` is the chapter
number. Chapter 4 also lists the full fragments for named grammar productions in `[productions]`.
An anchor identifies source material. Executable evidence belongs to individual `[[case]]` entries.

[jvms]: https://docs.oracle.com/javase/specs/jvms/se25/html/index.html

[The VarHandle catalog](varhandle.toml) maps Java SE API contracts, Java Memory Model guarantees,
and JVMS linkage and instruction rules to executable compatibility fixtures. Each requirement has a
stable ID, specification links, applicable JDKs, an expected observation, and source method anchors.
A condition is identified by the requirement ID, JDK, and named axis values. Counts describe the
catalog's test conditions. They do not measure the fraction of the JVMS implemented.

## Read and reproduce the matrix

Run these commands from the repository root with Python 3.11 or newer. The first validates every
catalog. The other two show the conditions and expected results for a selected requirement.

```sh
python3 docs/conformance/matrix.py
python3 docs/conformance/matrix.py --id JVMS.6.IADD --tsv
python3 docs/conformance/matrix.py --id VH.FINAL.MODES --tsv
```

To capture VarHandle execution evidence, run the compatibility harness and retain its exit status
and log. For all compatibility fixtures, omit the `TEST_FILTER` assignment in the command below. The
harness compiles each fixture for its configured releases, obtains output from the matching
reference JDK, and compares Ristretto interpreter and JIT output independently. The catalog includes
Java 11, 17, 21, and 25 where the tested API is available.

```sh
mkdir -p target/conformance
TEST_FILTER='^method/varhandle/' RUST_LOG=info cargo test --profile release-tests \
  -p ristretto_vm --test compatibility -- --exact compatibility_tests --nocapture \
  >target/conformance/varhandle.log 2>&1
```

The stack-exhaustion case in chapter 2 uses the `resource_errors` integration harness, which starts
bounded JVM subprocesses. Its results are recorded in `target/resource-errors/results.json`.

The export can also be generated after a failing run. It includes the expected outcome alongside the
observed result of the enclosing fixture for each engine.

```sh
python3 docs/conformance/matrix.py --tsv --log target/conformance/varhandle.log \
  >target/conformance/varhandle.tsv
python3 docs/conformance/matrix.py --failures --log target/conformance/varhandle.log \
  >target/conformance/failures.tsv
```

`fixture_int` and `fixture_jit` are **fixture-level** observations. A pass records agreement with
the reference run for the cases exercised by that fixture. A failure may stop execution at its first
failing assertion. The field and array matrix resets writable storage for each call and reports each
failed storage, carrier, mode, and route on standard error. The failure export preserves those
diagnostics. Use the fixture/JDK name to find its diagnostic in the log. An engine without a
matching result is marked `unobserved`, including when reference execution or compilation failed.
Preserve the catalog, source revision or working-tree diff, platform, JDK builds, and log together
when comparing runs. Multiple `--log` arguments apply in command-line order for reruns of the same
sources and environment.

## Extend coverage

Add a TOML catalog in this directory or extend an existing one. A catalog declares its format
version, a suite path relative to `tests/`, and a `[spec]` table of section URLs. JVMS requirements
can use a chapter, section, or instruction anchor. API and memory-order rules can cite the Java SE
API or JLS directly. Use `suite = "."` for requirements that span fixture directories. The optional
`inventory = true` checks that every fixture in the subtree and every configured JDK is indexed. Use
it for a catalog that owns that complete inventory.

Each `[[case]]` supplies the following fields.

<table>
  <thead>
    <tr>
      <th>Field</th>
      <th>Meaning</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><code>id</code></td>
      <td>Unique, persistent requirement name across catalogs.</td>
    </tr>
    <tr>
      <td><code>spec</code></td>
      <td>One or more keys from the catalog's specification table.</td>
    </tr>
    <tr>
      <td><code>java</code></td>
      <td>
        JDK versions as strings, or <code>"default"</code> for fixtures that use the harness
        default.
      </td>
    </tr>
    <tr>
      <td><code>fixture</code>, <code>evidence</code></td>
      <td>
        Fixture directory relative to the suite and executable <code>File.java#method</code> anchors
        relative to that directory.
      </td>
    </tr>
    <tr>
      <td><code>axes</code></td>
      <td>
        Optional independent dimensions. Each combination runs through the cited fixture. Choose
        dimensions appropriate to the rule, such as opcode, operand type, class-file version, or
        resolution state.
      </td>
    </tr>
    <tr>
      <td><code>expect</code></td>
      <td>Default observable result, exception, or allowed set of outcomes.</td>
    </tr>
    <tr>
      <td><code>rules</code></td>
      <td>
        Optional shared list from <code>[[rules.name]]</code>, applied after case-specific
        overrides.
      </td>
    </tr>
    <tr>
      <td><code>override</code></td>
      <td>
        Ordered rules with <code>when</code> and <code>expect</code>. The first matching rule
        supplies the outcome. The validator checks that every override matches at least one
        combination.
      </td>
    </tr>
    <tr>
      <td><code>status</code></td>
      <td>
        Defaults to <code>"implemented"</code>. Use <code>"planned"</code> with a specification,
        applicability, and expected outcome to record a gap. Omit fixture and evidence until an
        executable test exists.
      </td>
    </tr>
  </tbody>
</table>

Axes and Java versions accept a single string, a list, or a `"$name"` reference to a list in
`[sets]`. Sets may include other named sets. An optional `[[enumeration]]` cross-checks a set
against one regular-expression capture group in a suite source file. The VarHandle catalog uses this
to detect drift between the 31 cataloged modes and the Java calls that exercise them. Split cases
when an axis applies only to some inputs. Keep unsupported operations as executable exception cases.
Record release-specific reference behavior explicitly, especially where it differs from the current
API description.

For example, the version-55 dynamic-constant fixture indexes JVMS §5.4.3.6 and the `ldc` instruction
under `JVMS.LDC.VARHANDLE`. For concurrency requirements, describe permitted and forbidden
observations. Passing a finite run provides evidence from the schedules exercised.
