'''
Script to execute cargo tests
and build a release

This script is intended to be executed
from INSIDE the ../scripts directory!

@author: mnowzari
'''
import os
import glob
from pathlib import Path
import subprocess as sbpc

def err_msg(e: Exception, cmd: str):
    print (f"\nException {e} has occurred during the execution of {cmd}\n")

def teardown() -> bool:
    print("----\nExecuting teardown\n----")

    parent_path = Path(os.getcwd()).parent.absolute()
    search_pattern = f"{parent_path}\\temp"

    print (f"Searching {parent_path} for {search_pattern}\n")

    try:

        for f in glob.glob(search_pattern):
            print (f"Found and removing {f}")
            os.removedirs(f)

    except Exception as e:
        err_msg(e, "teardown")
        return False
    return True

def execute_tests() -> bool:
    print("----\nExecuting cargo tests\n----")

    try:
        cmd = "cargo test --no-fail-fast"
        print (f"{cmd}")
        output = sbpc.check_output(cmd, shell=True).decode("utf-8")
        print (f"{output}")

        teardown()
    except Exception as e:
        err_msg(e, cmd)
        return False
    return True

def rustfmt() -> bool:
    print("----\nExecuting rustfmt\n----")
    try:
        parent_path = Path(os.getcwd()).parent.absolute()

        cmd = f"rustfmt --verbose {parent_path}\\src\\main.rs"
        print (f"{cmd}")
        output = sbpc.check_output(cmd, shell=True).decode("utf-8")
        print (f"{output}")
    except Exception as e:
        err_msg(e, cmd)


if __name__ == "__main__":
    rustfmt()
    execute_tests()