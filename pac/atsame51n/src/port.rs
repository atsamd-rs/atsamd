#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    group: (),
}
impl RegisterBlock {
    #[doc = "0x00..0x120 - GROUP\\[%s\\]"]
    #[inline(always)]
    pub const fn group(&self, n: usize) -> &Group {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(0)
                .add(128 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x120 - GROUP\\[%s\\]"]
    #[inline(always)]
    pub fn group_iter(&self) -> impl Iterator<Item = &Group> {
        (0..3).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(0)
                .add(128 * n)
                .cast()
        })
    }
}
#[doc = "GROUP\\[%s\\]"]
pub use self::group::Group;
#[doc = r"Cluster"]
#[doc = "GROUP\\[%s\\]"]
pub mod group;
