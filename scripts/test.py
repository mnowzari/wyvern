'''
Script to execute cargo tests
and perform teardown post-testing bc
Rust does not offer a straightforward way
to teardown if a test fails

This script is intended to be executed
from INSIDE the ../scripts directory!

@author: mnowzari
'''
import os
import glob
from pathlib import Path
import subprocess as sbpc

def execute_tests() -> bool:
    print("----\nExecuting cargo tests\n----")

    try:
        cmd = "cargo test --no-fail-fast"
        print (f"{cmd}")
        output = sbpc.check_output(cmd, shell=True).decode("utf-8")
        print (f"{output}")
    except Exception as e:
        print (f"Except {e} has occurred during the execution of {cmd}")
        return False
    return True

def teardown() -> bool:
    print("----\nPerforming teardown\n----")

    parent_path = Path(os.getcwd()).parent.absolute()
    search_pattern = f"{parent_path}\\temp"

    print (f"Searching {parent_path} for {search_pattern}")

    try:
        for f in glob.glob(search_pattern):
            print (f"Found and removing {f}")
            os.removedirs(f)
    except Exception as e:
        print (f"Exception {e} has occurred during teardown!")
        return False
    return True

if __name__ == "__main__":
    execute_tests()
    teardown()