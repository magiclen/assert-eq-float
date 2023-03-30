/*!
# assert-eq-float

This crate provides the `assert_eq_float!` macro that supports floats.

## Examples

```rust
use assert_eq_float::assert_eq_float;

assert_eq_float!(1.1 + 0.1, 1.2); // error = 0.0000000000000021316282072803005
assert_eq_float!(1e100 + 2e100, 3e100); // error = 53290705182007510000000000000000000000000000000000000000000000000000000000000000000000
```

The default margin of error is dynamically computed by properties of IEEE 754 floating point numbers. You don't need to worry about it if you just want to check two float values are **equal**.
*/

#![no_std]

use num_traits::float::FloatCore;

#[doc(hidden)]
#[inline]
pub fn get_error<T: FloatCore>(a: T, b: T) -> T {
    // See: https://magiclen.org/float-precision/
    a.abs().min(b.abs()) * T::epsilon() * T::from(8).unwrap()
}

/// Asserts that two float expressions are (approximately) equal to each other.
///
/// On panic, this macro will print the values of the expressions with their
/// debug representations.
///
/// Like [`assert_eq!`], this macro has a second form, where a custom
/// panic message can be provided.
///
/// # Examples
///
/// ```rust
/// use assert_eq_float::assert_eq_float;
///
/// let a = 3.0;
/// let b = 1.0 + 2.0;
///
/// assert_eq_float!(a, b);
///
/// assert_eq_float!(a, b; "we are testing addition with {} and {}", a, b);
///
/// assert_eq_float!(a, b, 1e-7); // use a fixed error
/// ```
///
/// Note that when setting a custom panic message, you should use a semicolon `;` instead of a comma `,`.
#[macro_export]
macro_rules! assert_eq_float {
    (@standard $left:ident, $right:ident, $error:ident) => {
        if $left != $right {
            use num_traits::float::FloatCore;

            let d_abs = ($left - $right).abs();

            assert!(
                d_abs < $error,
                "thread 'main' panicked at 'assertion failed: `(left == right)`\n  left: `{:?}`,\n right: `{:?}`\n  diff: `{:?}`\n error: `{:?}`",
                $left,
                $right,
                d_abs,
                $error,
            );
        }
    };
    (@custom $left:ident, $right:ident, $error:ident; $($arg:tt)+) => {
        if $left != $right {
            use num_traits::float::FloatCore;

            let d_abs = ($left - $right).abs();

            assert!(
                d_abs < $error,
                $($arg)+
            );
        }
    };
    ($left:expr, $right:expr $(,)?) => {{
        let left = $left;
        let right = $right;
        let error = $crate::get_error(left, right);

        $crate::assert_eq_float!(@standard left, right, error);
    }};
    ($left:expr, $right:expr; $($arg:tt)+) => {{
        let left = $left;
        let right = $right;
        let error = $crate::get_error(left, right);

        $crate::assert_eq_float!(@custom left, right, error; $($arg)+);
    }};
    ($left:expr, $right:expr, $error:expr $(,)?) => {{
        let left = $left;
        let right = $right;
        let error = $error;

        $crate::assert_eq_float!(@standard left, right, error);
    }};
    ($left:expr, $right:expr, $error:expr; $($arg:tt)+) => {{
        let left = $left;
        let right = $right;
        let error = $error;

        $crate::assert_eq_float!(@custom left, right, error; $($arg)+);
    }};
}
