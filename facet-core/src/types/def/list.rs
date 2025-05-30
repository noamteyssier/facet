use crate::ptr::{PtrConst, PtrMut, PtrUninit};

use super::Shape;

/// Fields for list types
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
#[non_exhaustive]
pub struct ListDef {
    /// vtable for interacting with the list
    pub vtable: &'static ListVTable,
    /// shape of the items in the list
    pub t: fn() -> &'static Shape,
}

impl ListDef {
    /// Returns a builder for ListDef
    pub const fn builder() -> ListDefBuilder {
        ListDefBuilder::new()
    }

    /// Returns the shape of the items in the list
    pub fn t(&self) -> &'static Shape {
        (self.t)()
    }
}

/// Builder for ListDef
pub struct ListDefBuilder {
    vtable: Option<&'static ListVTable>,
    t: Option<fn() -> &'static Shape>,
}

impl ListDefBuilder {
    /// Creates a new ListDefBuilder
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            vtable: None,
            t: None,
        }
    }

    /// Sets the vtable for the ListDef
    pub const fn vtable(mut self, vtable: &'static ListVTable) -> Self {
        self.vtable = Some(vtable);
        self
    }

    /// Sets the item shape for the ListDef
    pub const fn t(mut self, t: fn() -> &'static Shape) -> Self {
        self.t = Some(t);
        self
    }

    /// Builds the ListDef
    pub const fn build(self) -> ListDef {
        ListDef {
            vtable: self.vtable.unwrap(),
            t: self.t.unwrap(),
        }
    }
}

/// Initialize a list in place with a given capacity
///
/// # Safety
///
/// The `list` parameter must point to uninitialized memory of sufficient size.
/// The function must properly initialize the memory.
pub type ListInitInPlaceWithCapacityFn =
    for<'mem> unsafe fn(list: PtrUninit<'mem>, capacity: usize) -> PtrMut<'mem>;

/// Push an item to the list
///
/// # Safety
///
/// The `list` parameter must point to aligned, initialized memory of the correct type.
/// `item` is moved out of (with [`core::ptr::read`]) — it should be deallocated afterwards (e.g.
/// with [`core::mem::forget`]) but NOT dropped.
pub type ListPushFn = unsafe fn(list: PtrMut, item: PtrMut);
// FIXME: this forces allocating item separately, copying it, and then dropping it — it's not great.

/// Get the number of items in the list
///
/// # Safety
///
/// The `list` parameter must point to aligned, initialized memory of the correct type.
pub type ListLenFn = unsafe fn(list: PtrConst) -> usize;

/// Get pointer to the data buffer of the list.
///
/// # Safety
///
/// The `list` parameter must point to aligned, initialized memory of the correct type.
pub type ListAsPtrFn = unsafe fn(list: PtrConst) -> PtrConst;

/// Virtual table for a list-like type (like `Vec<T>`,
/// but also `HashSet<T>`, etc.)
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(C)]
#[non_exhaustive]
pub struct ListVTable {
    /// cf. [`ListInitInPlaceWithCapacityFn`].
    /// Unbuildable lists exist, like arrays.
    pub init_in_place_with_capacity: Option<ListInitInPlaceWithCapacityFn>,

    /// cf. [`ListPushFn`]
    pub push: ListPushFn,

    /// cf. [`ListLenFn`]
    pub len: ListLenFn,

    /// cf. [`ListAsPtrFn`]
    pub as_ptr: ListAsPtrFn,
}

impl ListVTable {
    /// Returns a builder for ListVTable
    pub const fn builder() -> ListVTableBuilder {
        ListVTableBuilder::new()
    }
}

/// Builds a [`ListVTable`]
pub struct ListVTableBuilder {
    init_in_place_with_capacity: Option<ListInitInPlaceWithCapacityFn>,
    push: Option<ListPushFn>,
    len: Option<ListLenFn>,
    as_ptr: Option<ListAsPtrFn>,
}

impl ListVTableBuilder {
    /// Creates a new [`ListVTableBuilder`] with all fields set to `None`.
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            init_in_place_with_capacity: None,
            push: None,
            len: None,
            as_ptr: None,
        }
    }

    /// Sets the init_in_place_with_capacity field
    pub const fn init_in_place_with_capacity(mut self, f: ListInitInPlaceWithCapacityFn) -> Self {
        self.init_in_place_with_capacity = Some(f);
        self
    }

    /// Sets the push field
    pub const fn push(mut self, f: ListPushFn) -> Self {
        self.push = Some(f);
        self
    }

    /// Sets the len field
    pub const fn len(mut self, f: ListLenFn) -> Self {
        self.len = Some(f);
        self
    }

    /// Sets the as_ptr field
    pub const fn as_ptr(mut self, f: ListAsPtrFn) -> Self {
        self.as_ptr = Some(f);
        self
    }

    /// Builds the [`ListVTable`] from the current state of the builder.
    ///
    /// # Panics
    ///
    /// This method will panic if any of the required fields are `None`.
    pub const fn build(self) -> ListVTable {
        ListVTable {
            init_in_place_with_capacity: self.init_in_place_with_capacity,
            push: self.push.unwrap(),
            len: self.len.unwrap(),
            as_ptr: self.as_ptr.unwrap(),
        }
    }
}
