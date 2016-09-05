#!/bin/sh
set -eu

command_exists() {
    command -v $1 &> /dev/null
}

CARGO_HOME=${CARGO_HOME:-${HOME}/.cargo}
KCOV_DEFAULT_VERSION="v36"
GITHUB_KCOV="https://api.github.com/repos/SimonKagstrom/kcov/releases/latest"

# Usage: download and install the latest kcov version by default.
# Fall back to ${KCOV_DEFAULT_VERSION} from the kcov archive if the latest is unavailable.
KCOV_VERSION=$(curl -s ${GITHUB_KCOV} | jq -Mr .tag_name || echo)
KCOV_VERSION=${KCOV_VERSION:-$KCOV_DEFAULT_VERSION}

KCOV_TGZ="https://github.com/SimonKagstrom/kcov/archive/${KCOV_VERSION}.tar.gz"

rm -rf kcov-${KCOV_VERSION}/
mkdir kcov-${KCOV_VERSION}
curl -L --retry 3 "${KCOV_TGZ}" | tar xzvf - -C kcov-${KCOV_VERSION} --strip-components 1

num_proc=1
# If PARALLEL_BUILD environment variable is set then parallel build is enabled
if [ "${PARALLEL_BUILD:-}" != "" ]; then
    # If PARALLEL_BUILD content is a number then use it as number of parallel jobs
    if [ ! -z "${PARALLEL_BUILD##*[!0-9]*}" ]; then
        num_proc=${PARALLEL_BUILD}
    else
        # Try to determine the number of available CPUs
        if command_exists nproc; then
            num_proc=$(nproc)
        elif command_exists sysctl; then
            num_proc=$(sysctl -n hw.ncpu)
        fi
    fi
fi

cd kcov-${KCOV_VERSION}
mkdir build
cd build
if [ "$(uname)" = Darwin ]; then
    cmake -DCMAKE_BUILD_TYPE=RelWithDebInfo -GXcode ..
    xcodebuild -configuration Release
    cp src/Release/kcov src/Release/libkcov_system_lib.so "${CARGO_HOME}/bin"
else
    cmake -DCMAKE_BUILD_TYPE=RelWithDebInfo ..
    make -j ${num_proc}
    cp src/kcov src/libkcov_sowrapper.so "${CARGO_HOME}/bin"
  fi
