#![feature(core)]
#![cfg_attr(test, deny(warnings))]
#![deny(missing_docs)]

//! # Void
//!
//! The uninhabited void type for use in statically impossible cases.
//!
//! In its own crate so all the users in the ecosystem can share the same type.
//! This crate also comes ready with several extension traits for Result that add
//! extra functionality to `Result<T, Void>` and `Result<Void, E>`.
//!

use std::{fmt, ops, cmp};

/// The empty type for cases which can't occur.
#[derive(Copy)]
pub enum Void { }

impl Clone for Void {
    fn clone(&self) -> Void {
        unreachable(*self)
    }
}

impl fmt::Debug for Void {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unreachable(*self)
    }
}

impl fmt::Display for Void {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unreachable(*self)
    }
}

impl<T> cmp::PartialEq<T> for Void {
    fn eq(&self, _: &T) -> bool {
        unreachable(*self)
    }
}

impl<T> cmp::PartialOrd<T> for Void {
    fn partial_cmp(&self, _: &T) -> Option<cmp::Ordering> {
        unreachable(*self)
    }
}

/// A safe version of `intrinsincs::unreachable`. If this typechecks, anything
/// that causes this to run is unreachable code.
#[inline(always)]
pub fn unreachable<T = ()>(_: Void) -> T {
    use std::intrinsics;
    unsafe { intrinsics::unreachable() }
}

/// Extensions to `Result<T, Void>`
pub trait VoidExtensions<T>: Sized {
    /// Get the value out of a wrapper.
    fn void_unwrap(self) -> T;
}

impl<T> VoidExtensions<T> for Result<T, Void> {
    /// Get the value out of an always-ok Result.
    ///
    /// Never panics, since it is statically known to be Ok.
    #[inline]
    fn void_unwrap(self) -> T {
        match self {
            Ok(val) => val,
            Err(e) => unreachable(e)
        }
    }
}

/// Extensions to `Result<Void, E>`
pub trait ErrVoidExtensions<E>: Sized {
    /// Get the error out of a wrapper.
    fn void_unwrap_err(self) -> E;
}

impl<E> ErrVoidExtensions<E> for Result<Void, E> {
    /// Get the error out of an always-err Result.
    ///
    /// Never panics, since it is statically known to be Err.
    #[inline]
    fn void_unwrap_err(self) -> E {
        match self {
            Ok(v) => unreachable(v),
            Err(e) => e
        }
    }
}

