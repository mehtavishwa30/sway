# sway-fmt-v2 E2E Tests

This directory contains end to end tests for `sway_fmt_v2::Formatter`.

## Snapshot Testing with `insta`

Tests in this directory make use of [`insta`](https://crates.io/crates/insta#introduction) which is a library for snapshot testing.

Inline snapshot tests, which are tests where `assert_snapshot` takes two arguments, do not require any knowledge on how to use `insta` and can be treated as fancy `assert_eq`s.
