#!/bin/sh
# Copyright castrogarciajs author. All rights reserved. MIT license.

set -e

if ! command -v unzip >/dev/null; then
	echo "Error: unzip is required to install query (see: https://github.com/castrogarciajs/query#unzip-is-required )." 1>&2
	exit 1
fi