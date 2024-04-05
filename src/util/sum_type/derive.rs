use core::{fmt, hash::Hasher, mem::ManuallyDrop};

use super::repr::{Cons, Nil, SumList};

pub(super) trait SumClone: SumList {
    unsafe fn clone(this: &Self::Repr, tag: u8) -> ManuallyDrop<Self::Repr>;
}

impl SumClone for () {
    unsafe fn clone(_: &Self::Repr, _: u8) -> ManuallyDrop<Self::Repr>
    where
        Self: Clone,
    {
        ManuallyDrop::new(Nil)
    }
}

impl<Head, Tail> SumClone for (Head, Tail)
where
    Head: Clone,
    Tail: SumClone,
{
    unsafe fn clone(this: &Self::Repr, tag: u8) -> ManuallyDrop<Self::Repr> {
        ManuallyDrop::new(if tag == 0 {
            let data = unsafe { this.data.clone() };
            Cons { data }
        } else {
            let next = unsafe { Tail::clone(&this.next, tag - 1) };
            Cons { next }
        })
    }
}

pub(super) trait SumDebug: SumList {
    unsafe fn debug(this: &Self::Repr, tag: u8) -> &dyn fmt::Debug;
}

impl SumDebug for () {
    unsafe fn debug(_: &Self::Repr, _: u8) -> &dyn fmt::Debug {
        &()
    }
}

impl<Head, Tail> SumDebug for (Head, Tail)
where
    Head: fmt::Debug,
    Tail: SumDebug,
{
    unsafe fn debug(this: &Self::Repr, tag: u8) -> &dyn fmt::Debug {
        if tag == 0 {
            unsafe { &this.data }
        } else {
            unsafe { Tail::debug(&this.next, tag - 1) }
        }
    }
}

pub(super) trait SumPartialEq: SumList {
    unsafe fn eq(this: &Self::Repr, other: &Self::Repr, tag: u8) -> bool;
}

impl SumPartialEq for () {
    unsafe fn eq(_: &Self::Repr, _: &Self::Repr, _: u8) -> bool {
        true
    }
}

impl<Head, Tail> SumPartialEq for (Head, Tail)
where
    Head: PartialEq,
    Tail: SumPartialEq,
{
    unsafe fn eq(this: &Self::Repr, other: &Self::Repr, tag: u8) -> bool {
        if tag == 0 {
            unsafe { this.data == other.data }
        } else {
            unsafe { Tail::eq(&this.next, &other.next, tag - 1) }
        }
    }
}

pub(super) trait SumPartialOrd: SumList + SumPartialEq {
    unsafe fn partial_cmp(
        this: &Self::Repr,
        other: &Self::Repr,
        tag: u8,
    ) -> Option<core::cmp::Ordering>;
}

impl SumPartialOrd for () {
    unsafe fn partial_cmp(_: &Self::Repr, _: &Self::Repr, _: u8) -> Option<core::cmp::Ordering> {
        Some(core::cmp::Ordering::Equal)
    }
}

impl<Head, Tail> SumPartialOrd for (Head, Tail)
where
    Head: PartialOrd,
    Tail: SumPartialOrd,
{
    unsafe fn partial_cmp(
        this: &Self::Repr,
        other: &Self::Repr,
        tag: u8,
    ) -> Option<core::cmp::Ordering> {
        if tag == 0 {
            unsafe { this.data.partial_cmp(&other.data) }
        } else {
            unsafe { Tail::partial_cmp(&this.next, &other.next, tag - 1) }
        }
    }
}

pub(super) trait SumOrd: SumList + SumPartialOrd {
    unsafe fn cmp(this: &Self::Repr, other: &Self::Repr, tag: u8) -> core::cmp::Ordering;
}

impl SumOrd for () {
    unsafe fn cmp(_: &Self::Repr, _: &Self::Repr, _: u8) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
    }
}

impl<Head, Tail> SumOrd for (Head, Tail)
where
    Head: Ord,
    Tail: SumOrd,
{
    unsafe fn cmp(this: &Self::Repr, other: &Self::Repr, tag: u8) -> core::cmp::Ordering {
        if tag == 0 {
            unsafe { this.data.cmp(&other.data) }
        } else {
            unsafe { Tail::cmp(&this.next, &other.next, tag - 1) }
        }
    }
}

pub(super) trait SumHash: SumList {
    unsafe fn hash<H: Hasher>(this: &Self::Repr, tag: u8, state: &mut H);
}

impl SumHash for () {
    unsafe fn hash<H: Hasher>(_: &Self::Repr, _: u8, _: &mut H) {}
}

impl<Head, Tail> SumHash for (Head, Tail)
where
    Head: core::hash::Hash,
    Tail: SumHash,
{
    unsafe fn hash<H: Hasher>(this: &Self::Repr, tag: u8, state: &mut H) {
        if tag == 0 {
            unsafe { this.data.hash(state) }
        } else {
            unsafe { Tail::hash(&this.next, tag - 1, state) }
        }
    }
}