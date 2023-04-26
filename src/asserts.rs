#[macro_export]
macro_rules! assert_ok {
    ($cond:expr,) => {
        $crate::assert_ok!($cond);
    };

    ($cond:expr) => {
        match $cond {
            Ok(t) => t,
            Err(e) => {
                panic!("assertion failed, expected Ok(..), got Err({:?})", e);
            }
        }
    };

    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            Ok(t) => t,
            Err(e) => {
                panic!("assertion failed, expected Ok(..), got Err({:?}): {}", e, format_args!($($arg)+));
            }
        }
    };
}

#[macro_export]
macro_rules! debug_assert_ok {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_ok!($($arg)*); })
}

#[macro_export]
macro_rules! assert_err {
    ($cond:expr,) => {
        $crate::assert_err!($cond);
    };
    ($cond:expr) => {
        match $cond {
            Ok(t) => {
                panic!("assertion failed, expected Err(..), got Ok({:?})", t);
            },
            Err(e) => e,
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            Ok(t) => {
                panic!("assertion failed, expected Err(..), got Ok({:?}): {}", t, format_args!($($arg)+));
            },
            Err(e) => e,
        }
    };
}

#[macro_export]
macro_rules! debug_assert_err {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_err!($($arg)*); })
}

#[macro_export]
macro_rules! assert_ok_eq {
    ($cond:expr, $expected:expr,) => {
        $crate::assert_ok_eq!($cond, $expected);
    };
    ($cond:expr, $expected:expr) => {
        match $cond {
            Ok(t) => {
                assert_eq!(t, $expected);
                t
            },
            e @ Err(..) => {
                panic!("assertion failed, expected Ok(..), got {:?}", e);
            }
        }
    };
    ($cond:expr, $expected:expr, $($arg:tt)+) => {
        match $cond {
            Ok(t) => {
                assert_eq!(t, $expected);
                t
            },
            e @ Err(..) => {
                panic!("assertion failed, expected Ok(..), got {:?}: {}", e, format_args!($($arg)+));
            }
        }
    };
}

#[macro_export]
macro_rules! debug_assert_ok_eq {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_ok_eq!($($arg)*); })
}

// Return Err of the expression: `return Err($expression);`.
///
/// Used as `fail!(expression)`.
#[macro_export]
macro_rules! fail {
    ( $y:expr ) => {{
        return Err($y.into());
    }};
}

/// Evaluate `$x:expr` and if not true return `Err($y:expr)`.
///
/// Used as `ensure!(expression_to_ensure, expression_to_return_on_false)`.
#[macro_export]
macro_rules! ensure {
    ( $x:expr, $y:expr $(,)? ) => {{
        if !$x {
            $crate::fail!($y);
        }
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_ok_success() {
        assert_ok!(Ok::<(), ()>(()));
        assert_ok_eq!(Ok::<_, ()>(10), 10);
    }

    #[test]
    fn assert_err_success() {
        assert_err!(Err::<(), _>("fail"));
        assert_eq!(assert_err!(Err::<(), _>("error message")), "error message");
    }

    #[test]
    #[should_panic]
    fn assert_err_fail_on_ok() {
        assert_err!(Ok::<(), ()>(()));
    }

    #[test]
    #[should_panic]
    fn assert_ok_fail_on_err() {
        assert_ok!(Err::<(), ()>(()));
    }

    #[test]
    #[should_panic]
    fn assert_ok_fail_on_mismatch() {
        assert_ok_eq!(Ok::<_, ()>(10), 20);
    }
}
