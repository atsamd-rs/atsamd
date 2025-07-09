//! AES encryption support

use crate::aes::Keysizeselect;
use aes::Block;

/// Perform AES encryption using the given key.
///
/// Atsamd AES Hardware does key expansion on the
/// fly given the key
pub(super) unsafe fn encrypt<const N: usize>(key: &[u8; N], block: &mut Block) {
    let aes = unsafe { crate::pac::Aes::steal() };

    // Reset the AES peripheral to ensure clean slate
    aes.ctrla().write(|w| w.swrst().set_bit());
    // Wait for completion
    while aes.ctrla().read().swrst().bit_is_set() {}

    let keysize = match N {
        16 => Keysizeselect::_128bit,
        24 => Keysizeselect::_192bit,
        32 => Keysizeselect::_256bit,
        _ => panic!("Invalid AES keysize!"),
    };

    // Set peripheral to encryption mode
    // Set AES keysize based on key length
    // Enable the AES peripheral
    aes.ctrla().write(|w| {
        w.cipher()
            .enc()
            .keysize()
            .variant(keysize)
            .enable()
            .set_bit()
    });

    // Write AES key to hardware
    for (index, _) in key.iter().enumerate().step_by(4) {
        // Combine four u8 into one u32
        let data = u32::from_le_bytes([key[index], key[index + 1], key[index + 2], key[index + 3]]);
        aes.keyword(index / 4).write(|w| unsafe { w.bits(data) });
    }

    // Provide cleartext to AES module (little endian)
    for (index, _) in block.iter().enumerate().step_by(4) {
        // Combine four u8 into one u32
        let data = u32::from_le_bytes([
            block[index],
            block[index + 1],
            block[index + 2],
            block[index + 3],
        ]);
        // Write to same indata, hardware increments DATABUFPTR.INDATPTR
        aes.indata().write(|w| unsafe { w.bits(data) });
    }

    // Start AES computation
    aes.ctrlb().write(|w| w.start().set_bit());

    // Wait for completion
    while aes.intflag().read().enccmp().bit_is_clear() {}

    // Read cryptotext
    for index in (0..block.len()).into_iter().step_by(4) {
        // Need to split u32 into four u8
        let buf = aes.indata().read().bits();
        for (bytepos, byte) in u32::to_ne_bytes(buf).iter().enumerate() {
            block[index + bytepos] = *byte;
        }
    }
}
