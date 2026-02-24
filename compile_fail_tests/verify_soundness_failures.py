#!/usr/bin/env python3
"""
Verify that all Rust files in the compile_fail_tests directory
fail to compile with the expected error messages.
"""

import argparse
import os
import re
import subprocess


import sys


def main():
    """
    Find all rs, run cargo check on them, and verify that they fail with the expected error messages
    """
    args = parse_args()
    os.chdir(args.dir)

    rs_files = find_rs_files()
    unexpected_success = []
    expectation_failures = {}

    if not rs_files:
        print("No Rust files found in compile_fail_tests directory")
        return 1

    for file in rs_files:
        bin_name = file[:-3]  # strip .rs extension
        print(f"Testing {bin_name}...")

        expectations = extract_expectations(file)
        if not expectations:
            print(f"ERROR: No EXPECT: lines found in {file}")
            return 1

        success, output = run_cargo_check(bin_name)

        show_output = args.show_output
        if success:
            print(f"FAIL: Compilation succeeded unexpectedly: {bin_name}")
            unexpected_success.append(bin_name)
            show_output = True
        else:
            # Check if all expected errors are in the output
            unmet = verify_expectations(file, expectations, output)

            if unmet:
                print(f"FAIL: Expected errors not found for {bin_name}:")
                for exp in unmet:
                    print(f"  - {exp}")
                expectation_failures[bin_name] = unmet
                show_output = True

        if show_output:
            print(f"=== output for {bin_name}")
            print(output)
            print(f"=== end output for {bin_name}")

    # Final report
    if not unexpected_success and not expectation_failures:
        print("All tests failed to compile with expected errors!")
        return 0
    else:
        if unexpected_success:
            print("\nFAIL: Successfully compiled binaries that should have failed:")
            for target in unexpected_success:
                print(f"  {target}")

        if expectation_failures:
            print("\nFAIL: Missing expected error messages:")
            for bin_name, unmet in expectation_failures.items():
                print(f"  {bin_name}:")
                for exp in unmet:
                    print(f"    - {exp}")

        return 1


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Verify that all Rust files in the compile_fail_tests directory fail to compile with the expected error messages."
    )
    parser.add_argument(
        "dir",
        nargs="?",
        help="Directory containing Rust binaries",
        default=os.path.abspath(os.path.dirname(__file__)),
    )
    parser.add_argument("--show-output", action="store_true", help="Show output of all compilations, whether or not they succeed", default=False)
    return parser.parse_args()


def find_rs_files() -> list[str]:
    """Find all Rust files in the given directory."""
    return [file for file in os.listdir() if file.endswith(".rs")]


def extract_expectations(file_path: str) -> list[str]:
    """Extract the EXPECT: lines from a Rust file."""
    expectations = []
    with open(file_path, "r") as f:
        for line in f:
            match = re.search(r"EXPECT:\s*(.*)", line)
            if match:
                expectations.append(match.group(1).strip())
    return expectations


def run_cargo_check(bin_name: str) -> tuple[bool, str]:
    """
    Run cargo check for a specific binary target.
    Returns (compilation_succeeded, stderr)
    """
    env = os.environ.copy()
    # the simple pattern recognition fails with interpolated ANSI colors
    env["CARGO_TERM_COLOR"] = "never"
    try:
        process = subprocess.run(
            ["cargo", "check", "--bin", bin_name],
            capture_output=True,
            text=True,
            check=False,
            env=env,
        )
        return process.returncode == 0, process.stderr
    except subprocess.SubprocessError as e:
        print(f"Error running cargo check: {e}")
        return False, str(e)


def verify_expectations(file: str, expectations: list[str], output: str) -> list[str]:
    """
    Verify that all expectations appear in the compiler output.
    Returns a list of unmet expectations.
    """
    unmet_expectations = []
    for expectation in expectations:
        if expectation not in output:
            unmet_expectations.append(expectation)
    return unmet_expectations


if __name__ == "__main__":
    sys.exit(main())
