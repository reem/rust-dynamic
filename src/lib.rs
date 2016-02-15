#![cfg_attr(test, deny(warnings))]
#![deny(missing_docs)]

//! # dynamic
//!
//! A dyanmically typed value with fast downcasting.
//!

extern crate unsafe_any as uany;

use uany::UnsafeAnyExt;

use std::any::{TypeId, Any};
use std::{fmt, mem};

/// A dynamically typed value.
///
/// Differs from `Any` in that it pre-computes type information at
/// creation-time, so that downcasting and other queries to the type
/// information can be implemented without virtual calls.
///
/// Not Sized, since the size of the type is determined at runtime, so must be
/// used behind a pointer (e.g. `&Dynamic`, `Box<Dynamic`, etc.)
pub struct Dynamic {
    desc: Descriptor,
    data: Dyn
}

impl Dynamic {
    /// Create a new, heap-allocated Dynamic value containing the given value.
    ///
    /// The resulting `Dynamic` can be downcasted back to a `T`.
    #[inline]
    pub fn new<T: Any>(val: T) -> Box<Dynamic> {
        let un_sized = Box::new(Described {
            desc: Descriptor::new::<T>(),
            data: val
        }) as Box<Described<Dyn>>;

        unsafe { mem::transmute(un_sized) }
    }

    /// Create a new, immutable Dynamic value from the given described reference.
    ///
    /// The resulting `Dynamic` can be downcasted back to a `T`.
    #[inline]
    pub fn from_ref<T: Any>(val: &Described<T>) -> &Dynamic {
        let un_sized = val as &Described<Dyn>;
        unsafe { mem::transmute(un_sized) }
    }

    /// Create a new, mutable Dynamic value from the given described reference.
    ///
    /// The resulting `Dynamic` can be downcasted back to a `T`.
    #[inline]
    pub fn from_mut<T: Any>(val: &mut Described<T>) -> &mut Dynamic {
        let un_sized = val as &mut Described<Dyn>;
        unsafe { mem::transmute(un_sized) }
    }

    /// Read the type Descriptor for the contained value.
    #[inline]
    pub fn descriptor(&self) -> Descriptor {
        self.desc
    }

    /// Check if the contained type is a `T`.
    #[inline(always)]
    pub fn is<T: Any>(&self) -> bool {
        self.desc.id == TypeId::of::<T>()
    }

    /// If the contained value is a `T`, downcast back to it.
    ///
    /// If the value is not a `T`, returns `Err(self)`.
    #[inline]
    pub fn downcast<T: Any>(self: Box<Self>) -> Result<Box<Described<T>>, Box<Self>> {
        if self.is::<T>() {
            Ok(unsafe { Box::from_raw(Box::into_raw(self) as *mut Described<T>) })
        } else {
            Err(self)
        }
    }

    /// If the contained value is a `T`, get an immutable reference to it.
    #[inline]
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        if self.is::<T>() {
            Some(unsafe { self.data.downcast_ref_unchecked() })
        } else {
            None
        }
    }

    /// If the contained value is a `T`, get a mutable reference to it.
    #[inline]
    pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            Some(unsafe { self.data.downcast_mut_unchecked() })
        } else {
            None
        }
    }
}

impl fmt::Debug for Dynamic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Dynamic")
            .field("descriptor", &self.desc)
            .field("data", &"{{ dynamically typed value }}")
            .finish()
    }
}

/// A value T paired with its type descriptor.
///
/// Can be converted to a `Dynamic` value.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Described<T: ?Sized> {
    // The Descriptor is private to prevent mutation, as a user could then
    // invalidate it.
    desc: Descriptor,

    /// The described data.
    pub data: T
}

impl<T: Any> Described<T> {
    /// Create a new Described instance that can be converted to a Dynamic.
    #[inline]
    pub fn new(val: T) -> Described<T> {
        Described {
            desc: Descriptor::new::<T>(),
            data: val
        }
    }

    /// Read the type Descriptor for this value.
    #[inline]
    pub fn descriptor(&self) -> Descriptor { self.desc }
}

/// A type descriptor, containing metadata about a type.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Descriptor {
    /// The compiler-generated unique id of the type.
    ///
    /// As given by `TypeId::of::<T>()`
    pub id: TypeId,

    /// The size of the type.
    ///
    /// As given by `mem::size_of::<T>()`
    pub size: usize,

    /// The alignment of the type.
    ///
    /// As given by `mem::align_of::<T>()`
    pub alignment: usize
}

impl Descriptor {
    /// Create a
    #[inline(always)]
    pub fn new<T: Any>() -> Self {
        Descriptor {
            id: TypeId::of::<T>(),
            size: mem::size_of::<T>(),
            alignment: mem::align_of::<T>()
        }
    }
}

// Empty trait for small vtables.
trait Dyn {}
impl<T> Dyn for T {}

// Add raw downcasting methods to Dyn trait objects.
unsafe impl UnsafeAnyExt for Dyn {}

#[cfg(test)]
mod test {
    use {Dynamic, Described, Descriptor};

    struct X(usize);
    struct Y(usize);
    struct Z(usize);

    #[test]
    fn test_downcasting() {
        let mut x = Dynamic::new(X(1));

        assert!(x.is::<X>());
        assert!(!x.is::<Y>());
        assert!(!x.is::<Z>());

        *x.downcast_mut::<X>().unwrap() = X(100);
        assert_eq!(x.downcast_ref::<X>().unwrap().0, 100);

        let described_x = x.downcast::<X>().unwrap();
        assert_eq!(described_x.descriptor(), Descriptor::new::<X>());
        assert_eq!(described_x.data.0, 100);
    }

    #[test]
    fn test_dynamic_refs() {
        let described_z = Described::new(Z(1000));

        let z_ref = Dynamic::from_ref(&described_z);
        assert_eq!(z_ref.downcast_ref::<Z>().unwrap().0, 1000);
    }
}

