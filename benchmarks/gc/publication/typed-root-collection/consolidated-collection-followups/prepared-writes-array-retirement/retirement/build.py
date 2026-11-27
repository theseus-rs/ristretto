"""Build the frozen reference-array probe against a caller-selected Ristretto checkout."""
import argparse
import json
from pathlib import Path
import shutil
import subprocess


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("checkout", type=Path)
    parser.add_argument("output", type=Path, help="new build directory")
    parser.add_argument("--target", choices=["native", "wasm32-wasip2"], default="native")
    args = parser.parse_args()

    source = args.checkout.resolve(strict=True)
    fixture = Path(__file__).resolve().parent
    output = args.output.resolve()
    manifest = ['[workspace]', '[package]', 'name = "reference_card_scan"',
                'version = "0.0.0"', 'edition = "2024"', '[dependencies]']
    for name in ("ristretto_gc", "ristretto_classfile", "ristretto_classloader"):
        crate = (source / name / "Cargo.toml").resolve(strict=True).parent
        manifest.append(f"{name} = {{ path = {json.dumps(str(crate), ensure_ascii=False)} }}")

    output.mkdir(parents=True, exist_ok=False)
    (output / "src").mkdir()
    (output / "Cargo.toml").write_text("\n".join(manifest) + "\n", encoding="utf-8")
    shutil.copyfile(fixture / "Cargo.lock", output / "Cargo.lock")
    shutil.copyfile(fixture / "reference_card_scan.rs", output / "src/main.rs")
    shutil.copyfile(source / "rust-toolchain.toml", output / "rust-toolchain.toml")

    command = ["cargo", "build", "--locked", "--release", "--message-format=json-render-diagnostics",
               "--target-dir", str(output.parent / "build-cache")]
    if args.target != "native":
        command.extend(["--target", args.target])
    messages = subprocess.check_output(command, cwd=output, text=True).splitlines()
    for message in map(json.loads, messages):
        if message["reason"] == "compiler-artifact" and message.get("executable"):
            if message["target"]["name"] == "reference_card_scan":
                executable = Path(message["executable"])
                shutil.copyfile(executable, output / executable.name)
                (output / executable.name).chmod(0o755)
                print(executable)
                return
    raise RuntimeError("Cargo did not report the probe executable")


if __name__ == "__main__":
    main()
