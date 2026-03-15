# rs-result-ext

Extension traits for Result and Option with tap, map, and error accumulation.

## Installation

```toml
[dependencies]
philiprehberger-result-ext = "0.1"
```

## Usage

```rust
use philiprehberger_result_ext::{ResultExt, OptionExt, collect_results, ResultGroup};

// Tap for side effects (logging, metrics)
let value = Ok::<_, &str>(42)
    .tap_ok(|v| println!("Got value: {}", v))
    .tap_err(|e| println!("Error: {}", e));

// Map both variants
let result: Result<String, i32> = Ok("hello")
    .map_both(|s| s.to_uppercase(), |e: &str| e.len() as i32);

// Try recovery on error
let result = Err("primary failed")
    .or_try(|_| Ok("recovered"));

// Collect all errors, not just the first
let results = vec![Ok(1), Err("a"), Ok(3), Err("b")];
let outcome = collect_results(results);
assert_eq!(outcome, Err(vec!["a", "b"]));

// Accumulate results
let mut group = ResultGroup::new();
group.push(Ok(1));
group.push(Err("oops"));
group.push(Ok(3));
assert!(group.has_errors());
```

## API

| Function / Type | Description |
|-----------------|-------------|
| `ResultExt::tap_ok(f)` | Inspect Ok value without consuming |
| `ResultExt::tap_err(f)` | Inspect Err value without consuming |
| `ResultExt::map_both(ok_fn, err_fn)` | Map both Ok and Err variants |
| `ResultExt::or_try(f)` | Try to recover from an error |
| `OptionExt::tap_some(f)` | Inspect Some value without consuming |
| `OptionExt::tap_none(f)` | Execute function on None |
| `collect_results(iter)` | Collect all Ok values or all Err values |
| `ResultGroup::new()` | Create an error accumulator |
| `ResultGroup::push(result)` | Add a result to the group |
| `ResultGroup::finish()` | Get accumulated Ok values or all errors |

## License

MIT
