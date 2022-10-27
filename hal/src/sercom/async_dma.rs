//! Use the DMA Controller to perform async transfers using the SERCOM
//! peripheral
//!
//! See the [`mod@uart`], [`mod@i2c`] and [`mod@spi`] modules for the
//! corresponding DMA transfer implementations.

use crate::dmac::{Beat, Buffer};
use core::ops::Range;

// Implementation detail to make async I2C-DMA transfers work. Should not be
// used outside of this crate.
#[doc(hidden)]
pub struct ImmutableSlice<T: Beat>(Range<*mut T>);

impl<T: Beat> ImmutableSlice<T> {
    #[inline]
    pub(in super::super) fn from_slice(slice: &[T]) -> Self {
        let ptrs = slice.as_ptr_range();

        let ptrs = Range {
            start: ptrs.start.cast_mut(),
            end: ptrs.end.cast_mut(),
        };

        ImmutableSlice(ptrs)
    }
}

unsafe impl<T: Beat> Buffer for ImmutableSlice<T> {
    type Beat = T;
    #[inline]
    fn dma_ptr(&mut self) -> *mut Self::Beat {
        if self.incrementing() {
            self.0.end
        } else {
            self.0.start
        }
    }

    #[inline]
    fn incrementing(&self) -> bool {
        self.buffer_len() > 1
    }

    #[inline]
    fn buffer_len(&self) -> usize {
        self.0.end as usize - self.0.start as usize
    }
}

// Implementation detail to make async SERCOM-DMA transfers work. Should not be
// used outside of this crate.
#[doc(hidden)]
pub struct SercomPtr<T: Beat>(pub(in super::super) *mut T);

unsafe impl<T: Beat> Buffer for SercomPtr<T> {
    type Beat = T;

    #[inline]
    fn dma_ptr(&mut self) -> *mut Self::Beat {
        self.0
    }

    #[inline]
    fn incrementing(&self) -> bool {
        false
    }

    #[inline]
    fn buffer_len(&self) -> usize {
        1
    }
}
