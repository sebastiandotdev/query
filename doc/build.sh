#!/bin/bash
# Script to build the documentation
# Usage: ./build.sh

# Build the documentation

cargo doc --no-deps --all-features --all-targets
