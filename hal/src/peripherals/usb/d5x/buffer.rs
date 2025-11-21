use core::ops::{Deref, DerefMut};
use cortex_m::singleton;
use usb_device::{Result as UsbResult, UsbError};

/// Size of the buffer that holds ALL USB endpoint buffers
pub const BUFFER_SIZE: usize = {
    #[cfg(feature = "usb-buffer-1k")]
    {
        1024
    }
    #[cfg(feature = "usb-buffer-4k")]
    {
        4098
    }
    #[cfg(feature = "usb-buffer-8k")]
    {
        8192
    }
    #[cfg(not(any(
        feature = "usb-buffer-1k",
        feature = "usb-buffer-4k",
        feature = "usb-buffer-8k"
    )))]
    {
        2048 // Default
    }
};

/// Size of each USB endpoints buffer (Always guaranteed to be divisible by 4)
pub const ALLOC_SIZE_MAX_PER_EP: usize = BUFFER_SIZE / 16;

/// USB endpoint storage buffer, aligned to 4 bytes
#[repr(C, align(4))]
pub struct Buffer {
    inner: [u8; BUFFER_SIZE],
}

impl Buffer {
    pub const fn new() -> Self {
        Self {
            inner: [0; BUFFER_SIZE],
        }
    }
}

impl Deref for Buffer {
    type Target = [u8; BUFFER_SIZE];
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

fn buffer() -> &'static mut Buffer {
    singleton!(: Buffer = Buffer::new() ).unwrap()
}

pub struct BufferAllocator {
    buffers: &'static mut Buffer,
    next_buf: usize,
}

impl BufferAllocator {
    pub fn new() -> Self {
        Self {
            next_buf: 0,
            buffers: buffer(),
        }
    }

    /// Allocates a fixed buffer of [`ALLOC_SIZE_MAX_PER_EP`]
    pub fn allocate_buffer(&mut self) -> UsbResult<*mut u8> {
        // Do all range checks first
        if self.next_buf >= self.buffers.len()
            || self.next_buf + ALLOC_SIZE_MAX_PER_EP > self.buffers.len()
        {
            return Err(UsbError::EndpointMemoryOverflow);
        }

        let start_addr = &mut self.buffers[self.next_buf] as *mut u8;
        self.next_buf += ALLOC_SIZE_MAX_PER_EP;
        Ok(start_addr)
    }
}
