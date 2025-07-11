//! AES decryption support

use crate::aes::Keysizeselect;
use crate::pac;
use aes::Block;
use cipher::inout::InOut;

/// Perform AES decryption using the given key.
///
/// Atsamd AES Hardware does key expansion on the
/// fly given the key
pub(super) fn decrypt<'a, const N: usize>(
    aes: &pac::Aes,
    key: &[u8; N],
    mut blocks: InOut<'a, 'a, Block>,
) {
    // Reset the AES peripheral to ensure clean slate
    aes.ctrla().write(|w| w.swrst().set_bit());
    // Wait for completion
    while aes.ctrla().read().swrst().bit_is_set() {}

    let keysize = match N {
        16 => Keysizeselect::_128bit,
        24 => Keysizeselect::_192bit,
        32 => Keysizeselect::_256bit,
        // TODO Better way to handle this?
        _ => panic!("Invalid AES keysize!"),
    };

    // Set peripheral to decryption mode
    // Set AES keysize based on key length
    // Enable the AES peripheral
    aes.ctrla().write(|w| {
        w.cipher()
            .dec()
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

    // Provide cryptotext to AES module (little endian)
    let in_block = blocks.get_in();
    for (index, _) in in_block.iter().enumerate().step_by(4) {
        // Combine four u8 into one u32
        let data = u32::from_le_bytes([
            in_block[index],
            in_block[index + 1],
            in_block[index + 2],
            in_block[index + 3],
        ]);
        // Write to same indata, hardware increments DATABUFPTR.INDATPTR
        aes.indata().write(|w| unsafe { w.bits(data) });
    }

    // Start AES computation
    aes.ctrlb().modify(|_, w| w.start().set_bit());

    // Wait for completion
    while aes.intflag().read().enccmp().bit_is_clear() {}

    // Read cleartext
    let out_block = blocks.get_out();
    for index in (0..out_block.len()).step_by(4) {
        // Need to split u32 into four u8
        let buf = aes.indata().read().bits();
        for (bytepos, byte) in u32::to_ne_bytes(buf).iter().enumerate() {
            out_block[index + bytepos] = *byte;
        }
    }
}
