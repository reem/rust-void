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

/// The empty type for cases which can't occur.
pub enum Void { }

/// A safe version of `intrinsincs::unreachable`. If this typechecks, anything
/// that causes this to run is unreachable code.
#[inline(always)]
pub fn unreachable(_: Void) {
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
        self.map_err(unreachable).unwrap()
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
        self.map(unreachable).unwrap_err()
    }
}

