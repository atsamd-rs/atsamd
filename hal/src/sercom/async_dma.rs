//! Use the DMA Controller to perform async transfers using the SERCOM
//! peripheral
//!
//! See the [`mod@uart`], [`mod@i2c`] and [`mod@spi`] modules for the
//! corresponding DMA transfer implementations.

use crate::dmac::{AnyChannel, Beat, Buffer, Error, ReadyFuture, TriggerAction};
use core::ops::Range;

use super::Sercom;

/// Wrapper type over an `&[T]` that can be used as a source buffer for DMA
/// transfers. This is an implementation detail to make async SERCOM-DMA
/// transfers work. Should not be used outside of this crate.
///
/// # Safety
///
/// [`ImmutableSlice`]s should only ever be used as **source** buffers for DMA
/// transfers, and never as destination buffers.
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

/// Wrapper type over Sercom instances to get around lifetime issues when using
/// one as a DMA source/destination buffer. This is an implementation detail to
/// make async SERCOM-DMA transfers work. Should not be used outside of this
/// crate.
#[doc(hidden)]
#[derive(Clone)]
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

pub(super) async fn read_dma<T: Beat, S: Sercom>(
    channel: &mut impl AnyChannel<Status = ReadyFuture>,
    sercom_ptr: SercomPtr<T>,
    words: &mut [T],
) -> Result<(), Error> {
    #[cfg(feature = "min-samd51g")]
    let trigger_action = TriggerAction::BURST;

    #[cfg(any(feature = "samd11", feature = "samd21"))]
    let trigger_action = TriggerAction::BEAT;

    channel
        .as_mut()
        .transfer_future(sercom_ptr, words, S::DMA_RX_TRIGGER, trigger_action)
        .await
}

pub(super) async fn write_dma<T: Beat, S: Sercom>(
    channel: &mut impl AnyChannel<Status = ReadyFuture>,
    sercom_ptr: SercomPtr<T>,
    words: &[T],
) -> Result<(), Error> {
    // SAFETY: Using ImmutableSlice is safe because we hold on
    // to words as long as the transfer hasn't completed.
    let words = ImmutableSlice::from_slice(words);

    #[cfg(feature = "min-samd51g")]
    let trigger_action = TriggerAction::BURST;

    #[cfg(any(feature = "samd11", feature = "samd21"))]
    let trigger_action = TriggerAction::BEAT;

    channel
        .as_mut()
        .transfer_future(words, sercom_ptr, S::DMA_TX_TRIGGER, trigger_action)
        .await
}
