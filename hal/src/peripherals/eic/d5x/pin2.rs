use crate::ehal::digital::{InputPin,ErrorType};
use crate::eic::*;

// This conflicts with async path already implementing it
#[cfg(not(feature = "async"))]
impl<P, Id> ErrorType for ExtInt<P, Id, EicFuture>
where
    P: EicPin,
    Id: ChId,
    Self: InputPin<Error = Infallible>,
{
    type Error = Infallible;
}


impl<P, Id, F> InputPin for ExtInt<P, Id, F>
 where Self: ErrorType,
       P: EicPin,
       Id: ChId,
{
    #[inline]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.pin._is_high())
    }

    #[inline]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.pin._is_low())
    }
}
