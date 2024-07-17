/// Iterate over all bits that are `1`, returning the bit's position.
/// Shamelessly stolen from [embassy](https://github.com/embassy-rs/embassy/blob/3d1501c02038e5fe6f6d3b72bd18bd7a52595a77/embassy-stm32/src/exti.rs#L67)
pub struct BitIter(pub u32);

impl Iterator for BitIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.trailing_zeros() {
            32 => None,
            b => {
                self.0 &= !(1 << b);
                Some(b)
            }
        }
    }
}
