// TODO Is there a way to ensure RustCrypto with hardware backing cannot
// be used at the same time as "regular" AES peripheral interface?

use super::*;
mod decrypt;
use decrypt::decrypt;
mod encrypt;
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
        #[derive(Clone)]
        pub struct $name {
            encrypt: $name_enc,
            decrypt: $name_dec,
        }

        impl NewBlockCipher for $name {
            type KeySize = $key_size;

            #[inline]
            fn new(key: &GenericArray<u8, $key_size>) -> Self {
                let encrypt = $name_enc::new(key);
                let decrypt = $name_dec::from(&encrypt);
                Self { encrypt, decrypt }
            }
        }

        impl BlockCipher for $name {
            type BlockSize = U16;
            type ParBlocks = U1;
        }

        impl BlockEncrypt for $name {
            #[inline]
            fn encrypt_block(&self, block: &mut Block) {
                self.encrypt.encrypt_block(block)
            }
        }

        impl BlockDecrypt for $name {
            #[inline]
            fn decrypt_block(&self, block: &mut Block) {
                self.decrypt.decrypt_block(block)
            }
        }

        #[doc=$doc]
        #[doc = "block cipher (encrypt-only)"]
        #[derive(Clone)]
        pub struct $name_enc {
            key: GenericArray<u8, $key_size>,
        }

        impl NewBlockCipher for $name_enc {
            type KeySize = $key_size;

            fn new(key: &GenericArray<u8, $key_size>) -> Self {
                Self { key: *key }
            }
        }

        impl BlockCipher for $name_enc {
            type BlockSize = U16;
            type ParBlocks = U1;
        }

        impl BlockEncrypt for $name_enc {
            fn encrypt_block(&self, block: &mut Block) {
                unsafe { encrypt(&self.key.into(), block) }
            }
        }

        #[doc=$doc]
        #[doc = "block cipher (decrypt-only)"]
        #[derive(Clone)]
        pub struct $name_dec {
            key: GenericArray<u8, $key_size>,
        }

        impl NewBlockCipher for $name_dec {
            type KeySize = $key_size;

            fn new(key: &GenericArray<u8, $key_size>) -> Self {
                $name_enc::new(key).into()
            }
        }

        impl From<$name_enc> for $name_dec {
            fn from(enc: $name_enc) -> $name_dec {
                Self::from(&enc)
            }
        }

        impl From<&$name_enc> for $name_dec {
            fn from(enc: &$name_enc) -> $name_dec {
                let key = enc.key;
                Self { key }
            }
        }

        impl BlockCipher for $name_dec {
            type BlockSize = U16;
            type ParBlocks = U8;
        }

        impl BlockDecrypt for $name_dec {
            fn decrypt_block(&self, block: &mut Block) {
                unsafe { decrypt(&self.key.into(), block) }
            }
        }

        opaque_debug::implement!($name);
        opaque_debug::implement!($name_enc);
        opaque_debug::implement!($name_dec);
    };
}

define_aes_impl!(Aes128, Aes128Enc, Aes128Dec, U16, 16, "AES-128");
define_aes_impl!(Aes192, Aes192Enc, Aes192Dec, U24, 24, "AES-192");
define_aes_impl!(Aes256, Aes256Enc, Aes256Dec, U32, 32, "AES-256");
