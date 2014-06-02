#!/bin/bash

show_help() {
cat << EOF
Usage: ${0##*/} [-tb] FILE [FILES]...
Compile rust FILEs and summarize compilation status, test status and
errors. When FILE is -, read standard input.  This file is meant to be
called from a Makefile.

    -b          only compile FILE regularly, don't test
    -h          print this help
    -t          only compile FILEs with --test
EOF
}

test_only=false
build_only=false

OPTIND=1
while getopts ":tbh" opt; do
    case "$opt" in
        b)
            build_only=true
            shift 1
            ;;
        h)
            show_help
            exit
            ;;
        t)
            test_only=true
            shift 1
            ;;
        \?)
            echo "Invalid option: -$OPTARG" >&2
            exit 1
            ;;
    esac
done

if "$test_only" && "$build_only"; then
    echo "Cannot specify both -b (only build) and -t (only test) flags.  \
 To both build and test files, don't use either -t or -b."
    exit 1
fi

RESTORE='\033[0m'
RED='\033[00;31m'
GREEN='\033[00;32m'
YELLOW='\033[00;33m'
BLUE='\033[00;34m'

OK="${GREEN}ok${RESTORE}"
WARN="${YELLOW}warn${RESTORE}"
WARNING="${YELLOW}warning${RESTORE}"
ERROR="${RED}error${RESTORE}"

if [[ -z "$MAKELEVEL" ]]; then
    echo -e "$WARNING: $0 not run from Makefile, \
assuming sane defaults"
fi

# assume reasonable defaults
RUSTC=${RUSTC-rustc}
SRC_DIR=${SRC_DIR-src}
BUILD_DIR=${BUILD_DIR-build}
TEST_DIR=${TEST_DIR-build-tests}

# 0 - Ok, 1 - Warning, 2 - Error
compilation_status=''
compilation_output=''
compile_file() {
    compilation_output=''       # reset

    # TODO: avoid eval.  HOW?!
    local raw_output=$(eval "$@")
    # eval breaks getting the exit code with $?, so we grep for it
    # instead
    # local compile_exit_code=$?
    if echo "$raw_output" | grep -Fq 'error'; then
        # Error
        compilation_status=2
    else
        if [[ $raw_output == '' ]]; then
            # Ok
            compilation_status=0
        else
            # Warning
            compilation_status=1
        fi
    fi

    # Prettify output
    if [[ "$compilation_status" -gt 0 ]]; then
        compilation_output="\n"
        # Indent and replace backslashes with forward slashes.
        # rustc uses backslashes for Windows paths, which interacts
        # poorly with MSYS bash.
        compilation_output+=$(echo "$raw_output" | sed -e "s/^/  /" \
            | tr '\\' '/')
        compilation_output="${compilation_output//warning:/$WARNING:}"
        compilation_output="${compilation_output//error:/$ERROR:}"

    fi
    return "$compilation_status"
}

compilation_status_string=''
compilation_status_to_string() {
    local s="$1"
    case "$s" in
        0)
            compilation_status_string="$OK"
            ;;
        1)
            compilation_status_string="$WARNING"
            ;;
        2)
            compilation_status_string="$ERROR"
            ;;
    esac
}

lint_results=''

max_column_lint() {
    local source_file="$1"
    if grep  -q '.\{101,\}' "$source_file"; then
        lint_results+="\n  $source_file: $WARNING: file contains lines over \
100 columns"
    fi
}

trailing_whitespace_lint() {
    local source_file="$1"
    if grep -Eq ".* +$" "$source_file"; then
        lint_results+="\n  $source_file: $WARNING: file contains trailing \
whitespace"
    fi
}

run_all_lints() {
    local source_file="$1"
    max_column_lint "$source_file"
    trailing_whitespace_lint "$source_file"
}

# Used to delimit compilation output, test output, etc.
section_bullet="\n ${BLUE}*${RESTORE}"

test_rust_file() {
    local source_file="$1"

    test_status=''
    test_output=''
    test_status_string=''
    test_summary=''
    test_failures=''

    local compile_tests=true
    # Has the file been annotated as not_tested?  This attribute is by
    # convention, rustc doesn't care one bit about it.  If this
    # attribute is used, we won't throw warnings about the lack of
    # tests.
    if grep -Fq '// not_tested' "$source_file"; then
        test_status_string="${GREEN}no tests${RESTORE}"
        test_summary=" - ${GREEN}0/0${RESTORE}"
        compile_tests=false

    # Does the file have tests?  We don't want to compile the file for
    # tests if there are no tests because it will crash Windows.
    # https://github.com/mozilla/rust/issues/13793
    elif ! grep -Fq '#[test]' "$source_file"; then
        test_status_string="${YELLOW}not run${RESTORE}"
        test_summary=" - ${YELLOW}0/0${RESTORE}"
        # Special case lint, since it changes colors
        lint_results="\n  $source_file: $WARNING: no tests; add tests or \
annotate no tests with the magic comment: // not_tested"
        # don't break build for the lack of tests
        # touch "$TEST_DIR/ERRORS_HAPPENED"
        compile_tests=false
    fi

    if "$test_only"; then
        run_all_lints "$source_file"

        if [[ "$lint_results" ]]; then
            lint_results=" $section_bullet Lint Results$lint_results"
        fi

        if ! "$compile_tests"; then
            echo -e "$test_status_string$test_summary: $source_file\
$test_output$test_failures$lint_results"
        fi
    fi

    # Exit here because everything below requires test compilation
    if ! "$compile_tests"; then
        return 0
    fi

    # remove extension and paths
    file_name=$(basename "${source_file%.rs}")

    # TODO: do we need to worry about exit codes for testing?  I think
    # if the file compiles cleanly during regular compilation, it will
    # compile cleanly during the test compilation.
    local cmd="$RUSTC -o $TEST_DIR/$file_name --test $source_file 2>&1 >/dev/null"
    compile_file "$cmd"
    test_status="$compilation_status"
    test_output="$compilation_output"
    compilation_status_to_string "$test_status"
    test_status_string="$compilation_status_string"

    if [[ "$test_output" ]]; then
        test_output="$section_bullet Test Compilation Output$test_output"
    fi

    # Run the tests
    if [[ "$test_status" -lt 2 ]]; then
        # Ignore stderr for test_output. Useful test ouput is on stdout.
        # Only the overall rust task failure is printed on stderr.
        test_output_raw=$(./"$TEST_DIR"/"$file_name" 2>/dev/null)

        # Match specific test failure cases.  For example, we want to
        # match the following line:
        #
        # 'test_one' failed at 'assertion failed: is_prime(8)', src\prime.rs:24
        test_failures=$(echo "$test_output_raw" | grep -o "task '.*" \
            | sed -e "s/^/  /" \
            | tr '\\' '/')

        if [[ "$test_failures" ]]; then
            test_failures="$section_bullet Test Results\n$test_failures"
        fi

        # Extract the four numbers that summarize the test results.  For
        # example, the following string would yield the array (2 1 0 0):
        #
        # test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured
        test_results_raw=( $(echo "$test_output_raw" | grep "^test result" \
            | grep -o "[0-9]") )
        tests_passed="${test_results_raw[0]-0}"
        tests_failed="${test_results_raw[1]-0}"
        # TODO: ignore these for now
        # tests_ignored="${test_results_raw[2]}"
        # tests_measured="${test_results_raw[3]}"
        tests_total=$((tests_passed + tests_failed))

        if [[ "$tests_failed" -gt 0 ]]; then
            test_summary=" - ${RED}${tests_passed}/${tests_total}${RESTORE}"
        else
            test_summary=" - ${GREEN}${tests_passed}/${tests_total}${RESTORE}"
        fi
    fi

    if [[ "$test_status" -gt 0 || "$tests_failed" -gt 0 ]]; then
        touch "$TEST_DIR/ERRORS_HAPPENED"
    fi
    if "$test_only"; then
        echo -e "$test_status_string$test_summary: $source_file\
$test_output$test_failures$lint_results"
    fi

}

build_rust_file() {
    local source_file=$1

    # remove extension and paths
    file_name=$(basename "${source_file%.rs}")

    # Capture stderr and ignore stdout.  rustc doesn't print
    # anything on success.  For warnings and errors, rustc uses
    # stderr
    local cmd="$RUSTC -o $BUILD_DIR/$file_name $source_file 2>&1 >/dev/null"
    compile_file "$cmd"
    local build_status="$compilation_status"
    local build_output="$compilation_output"

    if [[ "$build_output" ]]; then
        build_output="$section_bullet Build Compilation Output$build_output"
    fi

    # Build the test version of the source file.
    if [[ "$build_status" -ne 2 ]] && ! "$build_only"; then
        test_rust_file "$source_file"

        # Pick the worse build status to display.
        if [[ "$test_status" -gt "$build_status" ]]; then
            build_status="$test_status"
        fi
    fi

    run_all_lints "$source_file"
    if [[ "$lint_results" ]]; then
        lint_results=" $section_bullet Lint Results$lint_results"
    fi

    if [[ "$build_status" -gt 0 ]]; then
        touch "$BUILD_DIR/ERRORS_HAPPENED"
    fi

    # Put after running the test file because the build_status takes
    # the worst of the build_status and test_status
    compilation_status_to_string "$build_status"
    local build_status_string="$compilation_status_string"

    echo -e "$build_status_string$test_summary: $source_file$build_output\
$test_output$test_failures$lint_results"
}

for f in "$@"; do
    if "$test_only"; then
        test_rust_file "$f"
    else
        build_rust_file "$f"
    fi
done
