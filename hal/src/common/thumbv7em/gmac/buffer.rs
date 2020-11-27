#![allow(dead_code)]

use super::private;
use generic_array::{ArrayLength, GenericArray};
use vcell::VolatileCell;

#[repr(C, align(8))]
pub struct TxBufferDescriptor {
    address: VolatileCell<u32>,
    status: VolatileCell<u32>,
}
#[allow(clippy::len_without_is_empty)]
impl TxBufferDescriptor {
    pub fn address(&self) -> u32 {
        self.address.get()
    }
    pub(crate) fn set_address(&mut self, address: u32) {
        self.address.set(address);
    }
    pub(crate) fn reset_status(&mut self) {
        self.status.set(1 << 31); // only `used` bit
    }
    pub(crate) fn set_wrap(&mut self, wrap: bool) {
        if wrap {
            self.address.set(self.address.get() | (1 << 30));
        } else {
            self.address.set(self.address.get() & !(1 << 30));
        }
    }
    pub(crate) fn reset_with_len(&mut self, len: usize, last_buf: bool) {
        self.status.set((len as u32) & ((1 << 14) - 1));
        if last_buf {
            self.address.set(self.address.get() | (1 << 16));
        }
    }
    pub fn len(&self) -> u16 {
        (self.status.get() & ((1 << 14) - 1)) as u16
    }
    pub fn last_buf(&self) -> bool {
        (self.status.get() >> 16) & 1 == 1
    }
    pub fn no_crc(&self) -> bool {
        (self.status.get() >> 17) & 1 == 1
    }
    pub fn checksum_err(&self) -> u8 {
        (self.status.get() >> 20 & 0x7) as u8
    }
    pub fn lco(&self) -> bool {
        (self.status.get() >> 26) & 1 == 1
    }
    pub fn exhausted(&self) -> bool {
        (self.status.get() >> 27) & 1 == 1
    }
    pub fn error(&self) -> bool {
        (self.status.get() >> 29) & 1 == 1
    }
    pub fn wrap(&self) -> bool {
        (self.status.get() >> 30) & 1 == 1
    }
    pub fn used(&self) -> bool {
        (self.status.get() >> 31) & 1 == 1
    }

    pub fn set_used(&mut self) {
        self.status.set(self.status.get() | (1 << 31));
    }
}

#[repr(C, align(8))]
pub struct RxBufferDescriptor {
    address: VolatileCell<u32>,
    status: VolatileCell<u32>,
}
#[allow(clippy::len_without_is_empty)]
impl RxBufferDescriptor {
    pub fn ownership(&self) -> bool {
        self.address.get() & 1 == 1
    }
    pub fn wrap(&self) -> bool {
        (self.address.get() >> 1) & 1 == 1
    }
    pub fn address(&self) -> u32 {
        self.address.get() >> 2
    }
    pub(crate) fn set_address(&mut self, address: u32) {
        assert!(address < (1 << 30));
        self.address.set(address & ((1 << 30) - 1));
    }
    pub(crate) fn reset_status(&mut self) {
        self.status.set(0);
    }
    pub(crate) fn set_wrap(&mut self, wrap: bool) {
        if wrap {
            self.address.set(self.address.get() | (1 << 1));
        } else {
            self.address.set(self.address.get() & !(1 << 1));
        }
    }

    pub fn len(&self) -> u16 {
        (self.status.get() & ((1 << 13) - 1)) as u16
    }
    pub fn fcs(&self) -> bool {
        (self.status.get() >> 13) & 1 == 1
    }
    pub fn sof(&self) -> bool {
        (self.status.get() >> 14) & 1 == 1
    }
    pub fn eof(&self) -> bool {
        (self.status.get() >> 15) & 1 == 1
    }
    pub fn cfi(&self) -> bool {
        (self.status.get() >> 16) & 1 == 1
    }
    pub fn vlan_priority(&self) -> u8 {
        (self.status.get() >> 17 & 0x7) as u8
    }
    pub fn priority_detected(&self) -> bool {
        (self.status.get() >> 20) & 1 == 1
    }
    pub fn vlan_detected(&self) -> bool {
        (self.status.get() >> 21) & 1 == 1
    }
    pub fn type_id_match(&self) -> u8 {
        (self.status.get() >> 22 & 0x3) as u8
    }
    pub fn checksumoffload(&self) -> bool {
        (self.status.get() >> 24) & 1 == 1
    }
    pub fn addrmatch(&self) -> u8 {
        ((self.status.get() >> 25) & 0x3) as u8
    }
    pub fn ext_addr_match(&self) -> bool {
        (self.status.get() >> 27) & 1 == 1
    }
    pub fn uni_hash_match(&self) -> bool {
        (self.status.get() >> 29) & 1 == 1
    }
    pub fn multi_hash_match(&self) -> bool {
        (self.status.get() >> 30) & 1 == 1
    }
    pub fn boardcast_detect(&self) -> bool {
        (self.status.get() >> 31) & 1 == 1
    }
}

impl Default for RxBufferDescriptor {
    fn default() -> Self {
        RxBufferDescriptor {
            address: VolatileCell::new(0),
            status: VolatileCell::new(0),
        }
    }
}
impl Default for TxBufferDescriptor {
    fn default() -> Self {
        TxBufferDescriptor {
            address: VolatileCell::new(0),
            status: VolatileCell::new(0),
        }
    }
}

pub trait BufferDescriptor: private::Sealed {
    fn init(&mut self, buf: &[u8], is_last: bool);
}
impl BufferDescriptor for RxBufferDescriptor {
    fn init(&mut self, buf: &[u8], is_last: bool) {
        self.set_address(buf.as_ptr() as u32);
        self.reset_status();
        self.set_wrap(is_last);
    }
}
impl BufferDescriptor for TxBufferDescriptor {
    fn init(&mut self, buf: &[u8], is_last: bool) {
        self.set_address(buf.as_ptr() as u32);
        self.reset_status();
        self.set_wrap(is_last);
    }
}

/// Wrapper type to ensure proper alignment of buffers
#[repr(align(32))]
pub struct GmacBuffer<Size: ArrayLength<u8>> {
    buffer: GenericArray<u8, Size>,
}
impl<Size: ArrayLength<u8>> core::ops::Deref for GmacBuffer<Size> {
    type Target = GenericArray<u8, Size>;
    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}
impl<Size: ArrayLength<u8>> core::ops::DerefMut for GmacBuffer<Size> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}
impl<Size: ArrayLength<u8>> core::convert::From<GenericArray<u8, Size>> for GmacBuffer<Size> {
    fn from(buf: GenericArray<u8, Size>) -> Self {
        GmacBuffer { buffer: buf }
    }
}
impl<Size: ArrayLength<u8>> Default for GmacBuffer<Size> {
    fn default() -> Self {
        GmacBuffer {
            buffer: GenericArray::default(),
        }
    }
}

pub struct GmacBufferSet<T, Count, Size>
where
    T: 'static + BufferDescriptor,
    Count: 'static + ArrayLength<T> + ArrayLength<GmacBuffer<Size>>,
    Size: 'static + ArrayLength<u8>,
{
    descriptors: &'static mut GenericArray<T, Count>,
    buffers: &'static mut GenericArray<GmacBuffer<Size>, Count>,
}

impl<T, Count, Size> GmacBufferSet<T, Count, Size>
where
    T: BufferDescriptor,
    Count: ArrayLength<T> + ArrayLength<GmacBuffer<Size>>,
    Size: ArrayLength<u8>,
{
    pub fn new(
        descriptors: &'static mut GenericArray<T, Count>,
        buffers: &'static mut GenericArray<GmacBuffer<Size>, Count>,
    ) -> Self {
        let count = Count::to_usize();
        for (idx, (descriptor, buffer)) in descriptors.iter_mut().zip(buffers.iter()).enumerate() {
            descriptor.init(&buffer[..], idx >= count - 1);
        }
        GmacBufferSet {
            descriptors,
            buffers,
        }
    }

    pub(crate) fn get_buffer(&self, index: usize) -> &[u8] {
        &self.buffers[index][..]
    }
    pub(crate) fn get_buffer_mut(&mut self, index: usize) -> &mut [u8] {
        &mut self.buffers[index][..]
    }
    pub(crate) fn get_descriptor(&self, index: usize) -> &T {
        &self.descriptors[index]
    }
    pub(crate) fn get_descriptor_mut(&mut self, index: usize) -> &mut T {
        &mut self.descriptors[index]
    }
}

#[cfg(feature = "nightly")]
impl<T, Count, Size> GmacBufferSet<T, Count, Size>
where
    T: BufferDescriptor + Default,
    Count: ArrayLength<T> + ArrayLength<GmacBuffer<Size>>,
    Size: ArrayLength<u8>,
{
    pub const fn create() -> Self {
        let mut descriptors: GenericArray<T, Count> = GenericArray::default();
        let buffers: GenericArray<GmacBuffer<Size>, Count> = GenericArray::default();
        let count = Count::to_usize();
        let mut idx = 0;
        while count > 0 {
            let descriptor = &mut descriptors[idx];
            let buffer = &buffers[idx];
            descriptor.init(&buffer[..], idx >= count - 1);
            idx -= 1;
        }
        GmacBufferSet {
            descriptors,
            buffers,
        }
    }
}

pub trait BufferSet<T>: private::Sealed {
    const COUNT: usize;
    const SIZE: usize;
    fn address(&self) -> u32;
    fn get_buffer(&self, index: usize) -> &[u8];
    fn get_buffer_mut(&mut self, index: usize) -> &mut [u8];
    fn get_descriptor(&self, index: usize) -> &T;
    fn get_descriptor_mut(&mut self, index: usize) -> &mut T;
}

impl<T, Count, Size> BufferSet<T> for GmacBufferSet<T, Count, Size>
where
    T: BufferDescriptor,
    Count: ArrayLength<T> + ArrayLength<GmacBuffer<Size>>,
    Size: ArrayLength<u8>,
{
    const COUNT: usize = Count::USIZE;
    const SIZE: usize = Size::USIZE;

    fn address(&self) -> u32 {
        self.descriptors.as_ptr() as u32
    }
    fn get_buffer(&self, index: usize) -> &[u8] {
        &self.get_buffer(index)[..]
    }
    fn get_buffer_mut(&mut self, index: usize) -> &mut [u8] {
        &mut self.get_buffer_mut(index)[..]
    }
    fn get_descriptor(&self, index: usize) -> &T {
        self.get_descriptor(index)
    }
    fn get_descriptor_mut(&mut self, index: usize) -> &mut T {
        self.get_descriptor_mut(index)
    }
}
