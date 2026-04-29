# Changelog

## 0.3.0 (2026-04-28)

- Add `ResultExt::tap(f)` — uniform inspection of both variants
- Add `OptionExt::ok_or_default()` — unwrap or `T::default()`
- Add `ResultGroup::with_capacity(n)` for pre-allocated accumulation
- Add `ResultGroup::extend(iter)` to feed an iterator into an existing group
- Add `ResultGroup::is_empty()` convenience

## 0.2.3 (2026-03-31)

- Standardize README to 3-badge format with emoji Support section
- Update CI checkout action to v5 for Node.js 24 compatibility

## 0.2.2 (2026-03-27)

- Add GitHub issue templates, PR template, and dependabot configuration
- Update README badges and add Support section

## 0.2.1 (2026-03-22)

- Fix CHANGELOG formatting

## 0.2.0 (2026-03-21)

- Add partition() function for splitting iterator of Results into (Vec<T>, Vec<E>)
- Add ResultGroup accessor methods: success_count(), values(), errors(), into_parts()
- Add #[must_use] attributes on key functions and methods

## 0.1.6 (2026-03-17)

- Add readme, rust-version, documentation to Cargo.toml
- Add Development section to README

## 0.1.5 (2026-03-16)

- Update install snippet to use full version

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
