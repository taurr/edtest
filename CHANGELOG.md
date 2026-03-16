# Changelog

All notable changes to this project will be documented in this file.

## Unreleased


## 0.8.0 - 2026-03-16

- chore(ci): update GitHub Actions versions (checkout@v6, rust-cache, git-auto-commit@v7) (fea0f8b)
- Add set_snapshot_suffix! macro and example test; closes #3 (1b64b0c)
- Update tests and README for issue #4 (c6b510e)

## 0.7.0 - 2026-03-02

### Changed

- Renamed exported attribute macro from `test` to `rstest` to avoid clashes with the builtin `test` attribute. Updated re-exports, docs, examples, and internal tests to use `edtest::rstest` (closes #1).

<!--
Guidelines:
- Use "Unreleased" for changes that haven't been released yet.
- When creating a release, move entries under a new heading with the version and date,
  e.g. `## 0.7.0 - 2026-03-02`.
-->
