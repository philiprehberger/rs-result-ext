# Changelog

## 0.1.4 (2026-03-16)

- Add README badges
- Synchronize version across Cargo.toml, README, and CHANGELOG

## 0.1.0 (2026-03-15)

- Initial release
- `ResultExt` trait: `tap_ok`, `tap_err`, `map_both`, `or_try`
- `OptionExt` trait: `tap_some`, `tap_none`, `ok_or_else_try`
- `collect_results()` for accumulating all errors from an iterator
- `ResultGroup` for building up results and reporting all errors at once
- `no_std` compatible (with `alloc`)
