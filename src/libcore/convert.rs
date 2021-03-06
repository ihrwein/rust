// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Traits for conversions between types.
//!
//! The traits in this module provide a general way to talk about
//! conversions from one type to another. They follow the standard
//! Rust conventions of `as`/`to`/`into`/`from`.

#![stable(feature = "rust1", since = "1.0.0")]

use marker::Sized;

/// A cheap, reference-to-reference conversion.
#[stable(feature = "rust1", since = "1.0.0")]
pub trait AsRef<T: ?Sized> {
    /// Performs the conversion.
    #[stable(feature = "rust1", since = "1.0.0")]
    fn as_ref(&self) -> &T;
}

/// A cheap, mutable reference-to-mutable reference conversion.
#[stable(feature = "rust1", since = "1.0.0")]
pub trait AsMut<T: ?Sized> {
    /// Performs the conversion.
    #[stable(feature = "rust1", since = "1.0.0")]
    fn as_mut(&mut self) -> &mut T;
}

/// A conversion that consumes `self`, which may or may not be
/// expensive.
#[stable(feature = "rust1", since = "1.0.0")]
pub trait Into<T>: Sized {
    /// Performs the conversion.
    #[stable(feature = "rust1", since = "1.0.0")]
    fn into(self) -> T;
}

/// Construct `Self` via a conversion.
#[stable(feature = "rust1", since = "1.0.0")]
pub trait From<T> {
    /// Performs the conversion.
    #[stable(feature = "rust1", since = "1.0.0")]
    fn from(T) -> Self;
}

////////////////////////////////////////////////////////////////////////////////
// GENERIC IMPLS
////////////////////////////////////////////////////////////////////////////////

// As lifts over &
#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T: ?Sized, U: ?Sized> AsRef<U> for &'a T where T: AsRef<U> {
    fn as_ref(&self) -> &U {
        <T as AsRef<U>>::as_ref(*self)
    }
}

// As lifts over &mut
#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T: ?Sized, U: ?Sized> AsRef<U> for &'a mut T where T: AsRef<U> {
    fn as_ref(&self) -> &U {
        <T as AsRef<U>>::as_ref(*self)
    }
}

// FIXME (#23442): replace the above impls for &/&mut with the following more general one:
// // As lifts over Deref
// impl<D: ?Sized + Deref, U: ?Sized> AsRef<U> for D where D::Target: AsRef<U> {
//     fn as_ref(&self) -> &U {
//         self.deref().as_ref()
//     }
// }

// AsMut lifts over &mut
#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T: ?Sized, U: ?Sized> AsMut<U> for &'a mut T where T: AsMut<U> {
    fn as_mut(&mut self) -> &mut U {
        (*self).as_mut()
    }
}

// FIXME (#23442): replace the above impl for &mut with the following more general one:
// // AsMut lifts over DerefMut
// impl<D: ?Sized + Deref, U: ?Sized> AsMut<U> for D where D::Target: AsMut<U> {
//     fn as_mut(&mut self) -> &mut U {
//         self.deref_mut().as_mut()
//     }
// }

// From implies Into
#[stable(feature = "rust1", since = "1.0.0")]
impl<T, U> Into<U> for T where U: From<T> {
    fn into(self) -> U {
        U::from(self)
    }
}

// From (and thus Into) is reflexive
#[stable(feature = "rust1", since = "1.0.0")]
impl<T> From<T> for T {
    fn from(t: T) -> T { t }
}

////////////////////////////////////////////////////////////////////////////////
// CONCRETE IMPLS
////////////////////////////////////////////////////////////////////////////////

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> AsRef<[T]> for [T] {
    fn as_ref(&self) -> &[T] {
        self
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> AsMut<[T]> for [T] {
    fn as_mut(&mut self) -> &mut [T] {
        self
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl AsRef<str> for str {
    fn as_ref(&self) -> &str {
        self
    }
}
