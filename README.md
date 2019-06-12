# sr

[![Build status](https://ci.appveyor.com/api/projects/status/je4jlk0bvygwnoi3/branch/master?svg=true)](https://ci.appveyor.com/project/DoumanAsh/sr/branch/master)
[![Build Status](https://travis-ci.org/DoumanAsh/sr.svg?branch=master)](https://travis-ci.org/DoumanAsh/sr)
[![Crates.io](https://img.shields.io/crates/v/sr.svg)](https://crates.io/crates/sr)
[![Dependency status](https://deps.rs/crate/sr/0.1.0/status.svg)](https://deps.rs/crate/sr)

Simple utility to search and replace within files

## Download links

* Windows [64bit](https://github.com/DoumanAsh/sr/releases/download/0.1.0/sr-0.1.0-x86_64-pc-windows-msvc.zip)
* Linux [64bit](https://github.com/DoumanAsh/sr/releases/download/0.1.0/sr-0.1.0-x86_64-unknown-linux-gnu.zip)
* OSX [64bit](https://github.com/DoumanAsh/sr/releases/download/0.1.0/sr-0.1.0-x86_64-apple-darwin.zip)

## Usage

```
USAGE:
    sr [options] <pattern> <replace> [file]...

OPTIONS:
    -i, --in-place [SUFFIX] - Modifies files in place. If SUFFIX is specified creates creates backup with it.
    -q, --quiet             - Specifies silent mode. Default false.
    -h, --help              - Prints this help message.

ARGS:
    <pattern> - Specifies regex to look for.
    <replace> - Specifies expression to replace with. captured values like $1 are allowed
    [file]... - Optionally specifies list of files. If omitted reads from STDIN.
```
