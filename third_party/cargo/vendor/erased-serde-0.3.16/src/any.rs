use crate::alloc::Box;
#[cfg(no_maybe_uninit)]
use core::marker::PhantomData;
use core::mem;
#[cfg(not(no_maybe_uninit))]
use core::mem::MaybeUninit;
use core::ptr;

#[cfg(feature = "unstable-debug")]
use core::any;

pub struct Any {
    value: Value,
    drop: fn(&mut Value),
    fingerprint: Fingerprint,

    /// For panic messages only. Not used for comparison.
    #[cfg(feature = "unstable-debug")]
    type_name: &'static str,
}

union Value {
    ptr: *mut (),
    inline: [MaybeUninit<usize>; 2],
}

fn is_small<T>() -> bool {
    cfg!(not(no_maybe_uninit))
        && mem::size_of::<T>() <= mem::size_of::<Value>()
        && mem::align_of::<T>() <= mem::align_of::<Value>()
}

// These functions are all unsafe. They are not exposed to the user. Declaring
// them as `unsafe fn` would not make the rest of erased-serde any safer or more
// readable.
impl Any {
    // This is unsafe -- caller must not hold on to the Any beyond the lifetime
    // of T.
    //
    // Example of bad code:
    //
    //    let s = "bad".to_owned();
    //    let a = Any::new(&s);
    //    drop(s);
    //
    // Now `a.view()` and `a.take()` return references to a dead String.
    pub(crate) fn new<T>(t: T) -> Self {
        let value: Value;
        let drop: fn(&mut Value);
        let fingerprint = Fingerprint::of::<T>();

        if is_small::<T>() {
            let mut inline = [MaybeUninit::uninit(); 2];
            unsafe { ptr::write(inline.as_mut_ptr() as *mut T, t) };
            value = Value { inline };
            drop = |value| unsafe { ptr::drop_in_place(value.inline.as_mut_ptr() as *mut T) };
        } else {
            let ptr = Box::into_raw(Box::new(t)) as *mut ();
            value = Value { ptr };
            drop = |value| mem::drop(unsafe { Box::from_raw(value.ptr as *mut T) });
        };

        // Once attributes on struct literal fields are stable, do that instead.
        // https://github.com/rust-lang/rust/issues/41681
        #[cfg(not(feature = "unstable-debug"))]
        {
            Any {
                value,
                drop,
                fingerprint,
            }
        }

        #[cfg(feature = "unstable-debug")]
        {
            let type_name = any::type_name::<T>();
            Any {
                value,
                drop,
                fingerprint,
                type_name,
            }
        }
    }

    // This is unsafe -- caller is responsible that T is the correct type.
    pub(crate) fn view<T>(&mut self) -> &mut T {
        if cfg!(not(miri)) && self.fingerprint != Fingerprint::of::<T>() {
            self.invalid_cast_to::<T>();
        }

        let ptr = if is_small::<T>() {
            unsafe { self.value.inline.as_mut_ptr() as *mut T }
        } else {
            unsafe { self.value.ptr as *mut T }
        };

        unsafe { &mut *ptr }
    }

    // This is unsafe -- caller is responsible that T is the correct type.
    pub(crate) fn take<T>(mut self) -> T {
        if cfg!(not(miri)) && self.fingerprint != Fingerprint::of::<T>() {
            self.invalid_cast_to::<T>();
        }

        if is_small::<T>() {
            let ptr = unsafe { self.value.inline.as_mut_ptr() as *mut T };
            let value = unsafe { ptr::read(ptr) };
            mem::forget(self);
            value
        } else {
            let ptr = unsafe { self.value.ptr as *mut T };
            let box_t = unsafe { Box::from_raw(ptr) };
            mem::forget(self);
            *box_t
        }
    }

    #[cfg(not(feature = "unstable-debug"))]
    fn invalid_cast_to<T>(&self) -> ! {
        panic!("invalid cast; enable `unstable-debug` feature to debug");
    }

    #[cfg(feature = "unstable-debug")]
    fn invalid_cast_to<T>(&self) -> ! {
        let from = self.type_name;
        let to = any::type_name::<T>();
        panic!("invalid cast: {} to {}", from, to);
    }
}

impl Drop for Any {
    fn drop(&mut self) {
        (self.drop)(&mut self.value);
    }
}

#[cfg(no_maybe_uninit)]
#[derive(Copy, Clone)]
struct MaybeUninit<T>(PhantomData<T>);

#[cfg(no_maybe_uninit)]
impl<T> MaybeUninit<T> {
    fn uninit() -> Self {
        MaybeUninit(PhantomData)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Fingerprint {
    size: usize,
    align: usize,
    id: usize,
}

impl Fingerprint {
    fn of<T>() -> Fingerprint {
        Fingerprint {
            size: mem::size_of::<T>(),
            align: mem::align_of::<T>(),
            // This is not foolproof -- theoretically Rust or LLVM could
            // deduplicate some or all of these methods. But in practice it's
            // great and I am comfortable relying on this in debug mode to catch
            // bugs early.
            id: Fingerprint::of::<T> as usize,
        }
    }
}

#[cfg(not(miri))]
#[test]
fn test_fingerprint() {
    assert_eq!(Fingerprint::of::<usize>(), Fingerprint::of::<usize>());
    assert_eq!(Fingerprint::of::<&str>(), Fingerprint::of::<&'static str>());

    assert_ne!(Fingerprint::of::<usize>(), Fingerprint::of::<isize>());
    assert_ne!(Fingerprint::of::<usize>(), Fingerprint::of::<&usize>());
    assert_ne!(Fingerprint::of::<&usize>(), Fingerprint::of::<&&usize>());
    assert_ne!(Fingerprint::of::<&usize>(), Fingerprint::of::<&mut usize>());

    struct A;
    struct B;
    assert_ne!(Fingerprint::of::<A>(), Fingerprint::of::<B>());
}
