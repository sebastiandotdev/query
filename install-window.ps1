#!/usr/bin/env pwsh
# Copyright the castrogarciajs author. All rights reserved. MIT license.

$ErrorActionPreference = 'Stop'

if ($v) {
  $Version = "v${v}"
}

if ($Args.Length -eq 1) {
  $Version = $Args.Get(0)
}

$QueryInstall = $env:QUERY_INSTALL

$BinDir = if ($QueryInstall) {
  "${QueryInstall}\bin"
} else {
  "${Home}\.query\bin"
}
