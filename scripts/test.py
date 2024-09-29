'''
Script to execute cargo tests

This script is intended to be executed
from INSIDE the ../scripts directory!

@author: mnowzari
'''
import os
import glob
from pathlib import Path
import subprocess as sbpc

def err_msg(excp: Exception, cmd: str):
    '''
    helper func to print error messages
    '''
    print (f"\nException {excp} has occurred during the execution of {cmd}\n")

def teardown() -> bool:
    '''
    perform any necessary teardown of files/folders created
    during test runs
    '''
    print("----\nExecuting teardown\n----")

    parent_path = Path(os.getcwd()).parent.absolute()
    search_path = os.path.join(parent_path, "temp")
    search_pattern = f"{search_path}*"

    print (f"Searching {parent_path} for {search_pattern}\n")

    try:

        for folder in glob.glob(search_pattern):
            print (f"Found and removing {folder}")
            os.removedirs(folder)

    except OSError as excp:
        err_msg(excp, "teardown")
        return False
    return True

def execute_tests() -> bool:
    '''
    run cargo test
    '''
    print("----\nExecuting cargo tests\n----")

    try:
        cmd = "cargo test --no-fail-fast"
        print (f"{cmd}")
        output = sbpc.check_output(cmd, shell=True).decode("utf-8")
        print (f"{output}")
    except sbpc.CalledProcessError as excp:
        err_msg(excp, cmd)
        return False
    return True

def rustfmt() -> bool:
    '''
    run rustfmt
    '''
    print("----\nExecuting rustfmt\n----")
    try:
        parent_path = Path(os.getcwd()).parent.absolute()

        cmd_path = os.path.join(parent_path, "src", "main.rs")
        cmd = f"rustfmt --verbose {cmd_path}"
        print (f"{cmd}")
        output = sbpc.check_output(cmd, shell=True).decode("utf-8")
        print (f"{output}")
    except sbpc.CalledProcessError as excp:
        err_msg(excp, cmd)

if __name__ == "__main__":
    rustfmt()
    execute_tests()
    teardown()
