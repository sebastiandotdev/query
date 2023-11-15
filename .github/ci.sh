#!/bin/bash
# Script to build CI

gh workflow run ci.yml -f build=1 -f test=1 -f bench=1 -f do