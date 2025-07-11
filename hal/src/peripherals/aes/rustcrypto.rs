// TODO Is there a way to ensure RustCrypto with hardware backing cannot
// be used at the same time as "regular" AES peripheral interface?

use super::*;
mod decrypt;
use decrypt::decrypt;
mod encrypt;
use cipher::{BlockBackend, ParBlocksSizeUser, inout::InOut};
use encrypt::encrypt;

macro_rules! define_aes_impl {
    (
        $name:ident,
        $name_enc:ident,
        $name_dec:ident,
        $key_size:ty,
        $key_length:tt,
        $doc:expr
    ) => {
        #[doc=$doc]
        #[doc = "block cipher"]
        pub struct $name {
            aes: Aes,
            key: GenericArray<u8, $key_size>,
        }
        impl $name {
            #[inline]
            pub(super) fn new(aes: Aes, key: &GenericArray<u8, $key_size>) -> Self {
                Self { aes, key: *key }
            }

            /// Destroy the cipher and release the AES peripheral
            pub fn free(self) -> Aes {
                self.aes
            }
        }

        impl BlockSizeUser for $name {
            type BlockSize = U16;
        }

        impl BlockCipher for $name {}

        impl BlockEncrypt for $name {
            #[inline]
            fn encrypt_with_backend(&self, f: impl BlockClosure<BlockSize = Self::BlockSize>) {
                f.call(&mut $name_enc { cipher: self })
            }
        }

        impl BlockDecrypt for $name {
            #[inline]
            fn decrypt_with_backend(&self, f: impl BlockClosure<BlockSize = Self::BlockSize>) {
                f.call(&mut $name_dec { cipher: self })
            }
        }

        #[doc=$doc]
        #[doc = "block cipher (encrypt-only)"]
        #[derive(Clone)]
        pub struct $name_enc<'a> {
            cipher: &'a $name,
        }

        impl BlockSizeUser for $name_enc<'_> {
            type BlockSize = U16;
        }

        impl ParBlocksSizeUser for $name_enc<'_> {
            type ParBlocksSize = U1;
        }

        impl BlockBackend for $name_enc<'_> {
            fn proc_block(&mut self, blocks: InOut<'_, '_, Block>) {
                encrypt(&self.cipher.aes.aes, self.cipher.key.as_ref(), blocks);
            }
        }

        #[doc=$doc]
        #[doc = "block cipher (decrypt-only)"]
        #[derive(Clone)]
        pub struct $name_dec<'a> {
            cipher: &'a $name,
        }

        impl BlockSizeUser for $name_dec<'_> {
            type BlockSize = U16;
        }

        impl ParBlocksSizeUser for $name_dec<'_> {
            type ParBlocksSize = U1;
        }

        impl BlockBackend for $name_dec<'_> {
            fn proc_block(&mut self, blocks: InOut<'_, '_, Block>) {
                decrypt(&self.cipher.aes.aes, self.cipher.key.as_ref(), blocks);
            }
        }

        opaque_debug::implement!($name);
        opaque_debug::implement!($name_enc<'_>);
        opaque_debug::implement!($name_dec<'_>);
    };
}

define_aes_impl!(Aes128, Aes128Enc, Aes128Dec, U16, 16, "AES-128");
define_aes_impl!(Aes192, Aes192Enc, Aes192Dec, U24, 24, "AES-192");
define_aes_impl!(Aes256, Aes256Enc, Aes256Dec, U32, 32, "AES-256");
