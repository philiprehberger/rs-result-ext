//! # philiprehberger-result-ext
//!
//! Extension traits for `Result` and `Option` with tap, map, and error accumulation.
//!
//! This crate is `no_std` compatible (requires `alloc` for `Vec`-based APIs).
//!
//! # Examples
//!
//! ```
//! use philiprehberger_result_ext::{ResultExt, OptionExt, collect_results, ResultGroup};
//!
//! // Tap for side effects
//! let value = Ok::<_, &str>(42)
//!     .tap_ok(|v| assert_eq!(*v, 42));
//!
//! // Map both variants at once
//! let result: Result<String, usize> = Ok("hello")
//!     .map_both(|s| s.to_uppercase(), |e: &str| e.len());
//! assert_eq!(result, Ok("HELLO".to_string()));
//!
//! // Collect all errors from an iterator
//! let results = vec![Ok(1), Err("a"), Ok(3), Err("b")];
//! let outcome = collect_results(results);
//! assert_eq!(outcome, Err(vec!["a", "b"]));
//! ```

#![no_std]

extern crate alloc;

use alloc::vec::Vec;

/// Extension trait for `Result<T, E>` providing tap, map, and recovery operations.
pub trait ResultExt<T, E> {
    /// Calls `f` with a reference to the `Ok` value if present, then returns `self` unchanged.
    ///
    /// Useful for side effects like logging without consuming the result.
    ///
    /// # Examples
    ///
    /// ```
    /// use philiprehberger_result_ext::ResultExt;
    ///
    /// let result = Ok::<i32, &str>(10)
    ///     .tap_ok(|v| assert_eq!(*v, 10));
    /// assert_eq!(result, Ok(10));
    /// ```
    fn tap_ok(self, f: impl FnOnce(&T)) -> Self;

    /// Calls `f` with a reference to the `Err` value if present, then returns `self` unchanged.
    ///
    /// Useful for side effects like logging without consuming the result.
    ///
    /// # Examples
    ///
    /// ```
    /// use philiprehberger_result_ext::ResultExt;
    ///
    /// let result = Err::<i32, &str>("oops")
    ///     .tap_err(|e| assert_eq!(*e, "oops"));
    /// assert_eq!(result, Err("oops"));
    /// ```
    fn tap_err(self, f: impl FnOnce(&E)) -> Self;

    /// Maps both the `Ok` and `Err` variants using the provided functions.
    ///
    /// # Examples
    ///
    /// ```
    /// use philiprehberger_result_ext::ResultExt;
    ///
    /// let ok_result: Result<i32, &str> = Ok(5);
    /// let mapped = ok_result.map_both(|v| v * 2, |e| e.len());
    /// assert_eq!(mapped, Ok(10));
    /// ```
    fn map_both<U, F>(self, ok_fn: impl FnOnce(T) -> U, err_fn: impl FnOnce(E) -> F) -> Result<U, F>;

    /// If `self` is `Err`, calls `f` with the error to attempt recovery.
    /// If `self` is `Ok`, returns it unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use philiprehberger_result_ext::ResultExt;
    ///
    /// let result: Result<i32, &str> = Err("failed");
    /// let recovered = result.or_try(|_| Ok(42));
    /// assert_eq!(recovered, Ok(42));
    /// ```
    fn or_try<F: FnOnce(E) -> Result<T, E>>(self, f: F) -> Result<T, E>;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn tap_ok(self, f: impl FnOnce(&T)) -> Self {
        if let Ok(ref v) = self {
            f(v);
        }
        self
    }

    fn tap_err(self, f: impl FnOnce(&E)) -> Self {
        if let Err(ref e) = self {
            f(e);
        }
        self
    }

    fn map_both<U, F>(self, ok_fn: impl FnOnce(T) -> U, err_fn: impl FnOnce(E) -> F) -> Result<U, F> {
        match self {
            Ok(v) => Ok(ok_fn(v)),
            Err(e) => Err(err_fn(e)),
        }
    }

    fn or_try<F: FnOnce(E) -> Result<T, E>>(self, f: F) -> Result<T, E> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => f(e),
        }
    }
}

/// Extension trait for `Option<T>` providing tap and fallible conversion operations.
pub trait OptionExt<T> {
    /// Calls `f` with a reference to the `Some` value if present, then returns `self` unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use philiprehberger_result_ext::OptionExt;
    ///
    /// let option = Some(42)
    ///     .tap_some(|v| assert_eq!(*v, 42));
    /// assert_eq!(option, Some(42));
    /// ```
    fn tap_some(self, f: impl FnOnce(&T)) -> Self;

    /// Calls `f` if `self` is `None`, then returns `self` unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use philiprehberger_result_ext::OptionExt;
    ///
    /// let option: Option<i32> = None;
    /// let option = option.tap_none(|| {});
    /// assert_eq!(option, None);
    /// ```
    fn tap_none(self, f: impl FnOnce()) -> Self;

    /// If `self` is `None`, calls `f` to try to produce a value or an error.
    ///
    /// If `self` is `Some(v)`, returns `Ok(v)` without calling `f`.
    ///
    /// # Examples
    ///
    /// ```
    /// use philiprehberger_result_ext::OptionExt;
    ///
    /// let option: Option<i32> = None;
    /// let result: Result<i32, &str> = option.ok_or_else_try(|| Ok(99));
    /// assert_eq!(result, Ok(99));
    ///
    /// let option: Option<i32> = None;
    /// let result: Result<i32, &str> = option.ok_or_else_try(|| Err("no value"));
    /// assert_eq!(result, Err("no value"));
    /// ```
    fn ok_or_else_try<E, F: FnOnce() -> Result<T, E>>(self, f: F) -> Result<T, E>;
}

impl<T> OptionExt<T> for Option<T> {
    fn tap_some(self, f: impl FnOnce(&T)) -> Self {
        if let Some(ref v) = self {
            f(v);
        }
        self
    }

    fn tap_none(self, f: impl FnOnce()) -> Self {
        if self.is_none() {
            f();
        }
        self
    }

    fn ok_or_else_try<E, F: FnOnce() -> Result<T, E>>(self, f: F) -> Result<T, E> {
        match self {
            Some(v) => Ok(v),
            None => f(),
        }
    }
}

/// Collects all results from an iterator, returning either all `Ok` values or all `Err` values.
///
/// Unlike the standard `collect::<Result<Vec<T>, E>>()` which stops at the first error,
/// this function processes every item and accumulates all errors.
///
/// # Examples
///
/// ```
/// use philiprehberger_result_ext::collect_results;
///
/// let all_ok: Vec<Result<i32, &str>> = vec![Ok(1), Ok(2), Ok(3)];
/// assert_eq!(collect_results(all_ok), Ok(vec![1, 2, 3]));
///
/// let mixed = vec![Ok(1), Err("a"), Ok(3), Err("b")];
/// assert_eq!(collect_results(mixed), Err(vec!["a", "b"]));
/// ```
pub fn collect_results<T, E>(iter: impl IntoIterator<Item = Result<T, E>>) -> Result<Vec<T>, Vec<E>> {
    let mut oks = Vec::new();
    let mut errs = Vec::new();

    for item in iter {
        match item {
            Ok(v) => oks.push(v),
            Err(e) => errs.push(e),
        }
    }

    if errs.is_empty() {
        Ok(oks)
    } else {
        Err(errs)
    }
}

/// An accumulator for `Result` values that collects all successes and errors.
///
/// Unlike short-circuiting error handling, `ResultGroup` processes every result
/// and reports all errors at once.
///
/// # Examples
///
/// ```
/// use philiprehberger_result_ext::ResultGroup;
///
/// let mut group = ResultGroup::new();
/// group.push(Ok(1));
/// group.push(Err("first error"));
/// group.push(Ok(3));
/// group.push(Err("second error"));
///
/// assert!(group.has_errors());
/// assert_eq!(group.error_count(), 2);
/// assert_eq!(group.finish(), Err(vec!["first error", "second error"]));
/// ```
pub struct ResultGroup<T, E> {
    oks: Vec<T>,
    errs: Vec<E>,
}

impl<T, E> ResultGroup<T, E> {
    /// Creates a new empty `ResultGroup`.
    pub fn new() -> Self {
        Self {
            oks: Vec::new(),
            errs: Vec::new(),
        }
    }

    /// Pushes a `Result` into the group, accumulating its value or error.
    pub fn push(&mut self, result: Result<T, E>) {
        match result {
            Ok(v) => self.oks.push(v),
            Err(e) => self.errs.push(e),
        }
    }

    /// Consumes the group and returns all `Ok` values if there were no errors,
    /// or all `Err` values if any errors were accumulated.
    pub fn finish(self) -> Result<Vec<T>, Vec<E>> {
        if self.errs.is_empty() {
            Ok(self.oks)
        } else {
            Err(self.errs)
        }
    }

    /// Returns `true` if any errors have been accumulated.
    pub fn has_errors(&self) -> bool {
        !self.errs.is_empty()
    }

    /// Returns the number of errors accumulated so far.
    pub fn error_count(&self) -> usize {
        self.errs.len()
    }
}

impl<T, E> Default for ResultGroup<T, E> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::cell::Cell;

    #[test]
    fn tap_ok_calls_fn_on_ok() {
        let called = Cell::new(false);
        let result = Ok::<i32, &str>(42).tap_ok(|v| {
            assert_eq!(*v, 42);
            called.set(true);
        });
        assert!(called.get());
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn tap_ok_does_not_call_fn_on_err() {
        let called = Cell::new(false);
        let result = Err::<i32, &str>("oops").tap_ok(|_| {
            called.set(true);
        });
        assert!(!called.get());
        assert_eq!(result, Err("oops"));
    }

    #[test]
    fn tap_err_calls_fn_on_err() {
        let called = Cell::new(false);
        let result = Err::<i32, &str>("oops").tap_err(|e| {
            assert_eq!(*e, "oops");
            called.set(true);
        });
        assert!(called.get());
        assert_eq!(result, Err("oops"));
    }

    #[test]
    fn tap_err_does_not_call_fn_on_ok() {
        let called = Cell::new(false);
        let result = Ok::<i32, &str>(42).tap_err(|_| {
            called.set(true);
        });
        assert!(!called.get());
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn map_both_maps_ok() {
        let result: Result<i32, &str> = Ok(5);
        let mapped = result.map_both(|v| v * 2, |e| e.len());
        assert_eq!(mapped, Ok(10));
    }

    #[test]
    fn map_both_maps_err() {
        let result: Result<i32, &str> = Err("hello");
        let mapped = result.map_both(|v| v * 2, |e| e.len());
        assert_eq!(mapped, Err(5));
    }

    #[test]
    fn or_try_recovers_from_err() {
        let result: Result<i32, &str> = Err("failed");
        let recovered = result.or_try(|_| Ok(42));
        assert_eq!(recovered, Ok(42));
    }

    #[test]
    fn or_try_recovery_fails() {
        let result: Result<i32, &str> = Err("first");
        let still_err = result.or_try(|_| Err("second"));
        assert_eq!(still_err, Err("second"));
    }

    #[test]
    fn or_try_passes_through_ok() {
        let result: Result<i32, &str> = Ok(10);
        let unchanged = result.or_try(|_| Ok(99));
        assert_eq!(unchanged, Ok(10));
    }

    #[test]
    fn tap_some_calls_fn_on_some() {
        let called = Cell::new(false);
        let option = Some(42).tap_some(|v| {
            assert_eq!(*v, 42);
            called.set(true);
        });
        assert!(called.get());
        assert_eq!(option, Some(42));
    }

    #[test]
    fn tap_some_does_not_call_fn_on_none() {
        let called = Cell::new(false);
        let option: Option<i32> = None;
        let option = option.tap_some(|_| {
            called.set(true);
        });
        assert!(!called.get());
        assert_eq!(option, None);
    }

    #[test]
    fn tap_none_calls_fn_on_none() {
        let called = Cell::new(false);
        let option: Option<i32> = None;
        let option = option.tap_none(|| {
            called.set(true);
        });
        assert!(called.get());
        assert_eq!(option, None);
    }

    #[test]
    fn tap_none_does_not_call_fn_on_some() {
        let called = Cell::new(false);
        let option = Some(42).tap_none(|| {
            called.set(true);
        });
        assert!(!called.get());
        assert_eq!(option, Some(42));
    }

    #[test]
    fn ok_or_else_try_returns_some_as_ok() {
        let option = Some(42);
        let result: Result<i32, &str> = option.ok_or_else_try(|| Err("unreachable"));
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn ok_or_else_try_calls_fn_on_none_ok() {
        let option: Option<i32> = None;
        let result: Result<i32, &str> = option.ok_or_else_try(|| Ok(99));
        assert_eq!(result, Ok(99));
    }

    #[test]
    fn ok_or_else_try_calls_fn_on_none_err() {
        let option: Option<i32> = None;
        let result: Result<i32, &str> = option.ok_or_else_try(|| Err("no value"));
        assert_eq!(result, Err("no value"));
    }

    #[test]
    fn collect_results_all_ok() {
        let results: Vec<Result<i32, &str>> = alloc::vec![Ok(1), Ok(2), Ok(3)];
        assert_eq!(collect_results(results), Ok(alloc::vec![1, 2, 3]));
    }

    #[test]
    fn collect_results_all_err() {
        let results: Vec<Result<i32, &str>> = alloc::vec![Err("a"), Err("b")];
        assert_eq!(collect_results(results), Err(alloc::vec!["a", "b"]));
    }

    #[test]
    fn collect_results_mixed() {
        let results = alloc::vec![Ok(1), Err("a"), Ok(3), Err("b")];
        assert_eq!(collect_results(results), Err(alloc::vec!["a", "b"]));
    }

    #[test]
    fn collect_results_empty() {
        let results: Vec<Result<i32, &str>> = alloc::vec![];
        assert_eq!(collect_results(results), Ok(alloc::vec![]));
    }

    #[test]
    fn result_group_all_ok() {
        let mut group = ResultGroup::new();
        group.push(Ok::<i32, &str>(1));
        group.push(Ok(2));
        group.push(Ok(3));
        assert!(!group.has_errors());
        assert_eq!(group.error_count(), 0);
        assert_eq!(group.finish(), Ok(alloc::vec![1, 2, 3]));
    }

    #[test]
    fn result_group_mixed() {
        let mut group = ResultGroup::new();
        group.push(Ok(1));
        group.push(Err("a"));
        group.push(Ok(3));
        group.push(Err("b"));
        assert!(group.has_errors());
        assert_eq!(group.error_count(), 2);
        assert_eq!(group.finish(), Err(alloc::vec!["a", "b"]));
    }

    #[test]
    fn result_group_empty() {
        let group: ResultGroup<i32, &str> = ResultGroup::new();
        assert!(!group.has_errors());
        assert_eq!(group.error_count(), 0);
        assert_eq!(group.finish(), Ok(alloc::vec![]));
    }

    #[test]
    fn result_group_default() {
        let group: ResultGroup<i32, &str> = ResultGroup::default();
        assert_eq!(group.finish(), Ok(alloc::vec![]));
    }
}
