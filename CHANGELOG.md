# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.2]

- Fixed pre-allocated memmap buffer size
- Fixed failing tests

## [0.6.0] - 2019-06-15

### Improvements

- `sd` now uses memory-mapped files, allowing replacement on files of any size
- `-p`/`--preview` flag is now added to preview changes
  - as of right now, results are simply emitted to stdout
  - in a future version, the output will be changed to contain only relevant information
- a `w` regex flag is added to match full words only, e.g. `sd -f w foo bar file.txt`

### Deprecations

- `--in-place` is now deprecated and assumed whenever a list of files is given

## [0.5.0] - 2019-02-22

### Improvements

- Windows support (thanks to @ErichDonGubler)

## [0.4.2] - 2019-01-02

### Improvements

- Support for unicode and special characters (like `\n`) in replacement expressions
  - Only in regex mode
- Fixed edge-cases when replacing content containing unescaped characters

## [0.4.1] - 2019-01-01

### Improvements

- Significant performance improvements (see benchmarks in README)

## [0.4.0] - 2018-12-30

### Added

- Option to set regex flags via `-f` or `--flags`:
  - `m` (multi-line)
  - `c` (case-sensitive)
  - `i` (case-insensitive)
- Smart case-sensitivity is used by default with regular expressions

### Improvements

- You may now pass multiple files to `sd`
  - this is now valid: `sd -i "\n" "," *.txt`

## [0.3.0] - 2018-12-29

**Breaking Change**: the `-i`/`--input` is revamped per [#1](https://github.com/chmln/sd/issues/1). The file path now comes at the end, instead of `-i`. 

Transforming the file in-place:
- Before: `sd -s 'str' '' -i file.txt'`
- Now: `sd -i -s 'str' '' file.txt`
- Future: `sd -i -s 'str' '' *.txt`

To reflect this change, `--input` is also renamed to `--in-place`. This is the first and most likely the last breaking change in this project.

### Improvements

- Files are now written to [atomically](https://github.com/chmln/sd/issues/3)


