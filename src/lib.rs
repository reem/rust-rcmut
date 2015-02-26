#![cfg_attr(test, deny(warnings))]
#![deny(missing_docs)]

//! # RcMut
//!
//! An unchecked shared mutability primitive.
//!
//! RcMut provides unchecked shared mutability and ownership.
//! It is extremely unsafe to use directly.
//!

use std::cell::UnsafeCell;
use std::sync::Arc;
use std::rc::Rc;
use std::mem;

/// A reference counted smart pointer with unrestricted mutability.
pub struct RcMut<T> {
    inner: Rc<UnsafeCell<T>>
}

impl<T> Clone for RcMut<T> {
    fn clone(&self) -> RcMut<T> {
        RcMut { inner: self.inner.clone() }
    }
}

impl<T> RcMut<T> {
    /// Create a new RcMut for a value.
    pub fn new(val: T) -> RcMut<T> {
        RcMut {
            inner: Rc::new(UnsafeCell::new(val))
        }
    }

    /// Retrieve the inner Rc as a reference.
    pub unsafe fn as_rc(&self) -> &Rc<T> {
        mem::transmute(&self.inner)
    }

    /// Retrieve the inner Rc as a mutable reference.
    pub unsafe fn as_rc_mut(&mut self) -> &mut Rc<T> {
        mem::transmute(&mut self.inner)
    }

    /// Get a reference to the value.
    pub unsafe fn borrow(&self) -> &T {
        mem::transmute(self.inner.get())
    }

    /// Get a mutable reference to the value.
    pub unsafe fn borrow_mut(&mut self) -> &mut T {
        mem::transmute(self.inner.get())
    }
}

/// A reference counted smart pointer with unrestricted mutability.
pub struct ArcMut<T> {
    inner: Arc<UnsafeCell<T>>
}

impl<T> Clone for ArcMut<T> {
    fn clone(&self) -> ArcMut<T> {
        ArcMut { inner: self.inner.clone() }
    }
}

impl<T> ArcMut<T> {
    /// Create a new ArcMut for a value.
    pub fn new(val: T) -> ArcMut<T> {
        ArcMut {
            inner: Arc::new(UnsafeCell::new(val))
        }
    }

    /// Retrieve the inner Rc as a reference.
    pub unsafe fn as_arc(&self) -> &Arc<T> {
        mem::transmute(&self.inner)
    }

    /// Retrieve the inner Rc as a mutable reference.
    pub unsafe fn as_arc_mut(&mut self) -> &mut Arc<T> {
        mem::transmute(&mut self.inner)
    }

    /// Get a reference to the value.
    pub unsafe fn borrow(&self) -> &T {
        mem::transmute(self.inner.get())
    }

    /// Get a mutable reference to the value.
    pub unsafe fn borrow_mut(&mut self) -> &mut T {
        mem::transmute(self.inner.get())
    }
}

