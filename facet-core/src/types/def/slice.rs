use crate::ptr::PtrConst;

use super::Shape;

/// Fields for slice types
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
#[non_exhaustive]
pub struct SliceDef {
    /// vtable for interacting with the slice
    pub vtable: &'static SliceVTable,

    /// shape of the items in the slice
    pub t: &'static Shape,
}

impl SliceDef {
    /// Returns a builder for SliceDef
    pub const fn builder() -> SliceDefBuilder {
        SliceDefBuilder::new()
    }

    /// Returns the shape of the items in the slice
    pub const fn t(&self) -> &'static Shape {
        self.t
    }
}

/// Builder for SliceDef
pub struct SliceDefBuilder {
    vtable: Option<&'static SliceVTable>,
    t: Option<&'static Shape>,
}

impl SliceDefBuilder {
    /// Creates a new SliceDefBuilder
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            vtable: None,
            t: None,
        }
    }

    /// Sets the vtable for the SliceDef
    pub const fn vtable(mut self, vtable: &'static SliceVTable) -> Self {
        self.vtable = Some(vtable);
        self
    }

    /// Sets the item shape for the SliceDef
    pub const fn t(mut self, t: &'static Shape) -> Self {
        self.t = Some(t);
        self
    }

    /// Builds the SliceDef
    pub const fn build(self) -> SliceDef {
        SliceDef {
            vtable: self.vtable.unwrap(),
            t: self.t.unwrap(),
        }
    }
}

/// Get the number of items in the slice
///
/// # Safety
///
/// The `slice` parameter must point to aligned, initialized memory of the correct type.
pub type SliceLenFn = unsafe fn(slice: PtrConst) -> usize;

/// Get pointer to the data buffer of the slice
///
/// # Safety
///
/// The `slice` parameter must point to aligned, initialized memory of the correct type.
pub type SliceAsPtrFn = unsafe fn(slice: PtrConst) -> PtrConst;

/// Virtual table for a slice-like type (like `Vec<T>`,
/// but also `HashSet<T>`, etc.)
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(C)]
#[non_exhaustive]
pub struct SliceVTable {
    /// Number of items in the slice
    pub len: SliceLenFn,
    /// Get pointer to the data buffer of the slice.
    pub as_ptr: SliceAsPtrFn,
}

impl SliceVTable {
    /// Returns a builder for SliceVTable
    pub const fn builder() -> SliceVTableBuilder {
        SliceVTableBuilder::new()
    }
}

/// Builds a [`SliceVTable`]
pub struct SliceVTableBuilder {
    as_ptr: Option<SliceAsPtrFn>,
    len: Option<SliceLenFn>,
}

impl SliceVTableBuilder {
    /// Creates a new [`SliceVTableBuilder`] with all fields set to `None`.
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            len: None,
            as_ptr: None,
        }
    }

    /// Sets the `len` field
    pub const fn len(mut self, f: SliceLenFn) -> Self {
        self.len = Some(f);
        self
    }

    /// Sets the as_ptr field
    pub const fn as_ptr(mut self, f: SliceAsPtrFn) -> Self {
        self.as_ptr = Some(f);
        self
    }

    /// Builds the [`SliceVTable`] from the current state of the builder.
    ///
    /// # Panics
    ///
    /// This method will panic if any of the required fields are `None`.
    pub const fn build(self) -> SliceVTable {
        SliceVTable {
            len: self.len.unwrap(),
            as_ptr: self.as_ptr.unwrap(),
        }
    }
}
