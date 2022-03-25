use dryoc::constants::*;
use napi::bindgen_prelude::*;
use sodiumoxide::{crypto::secretbox, init};
use std::ops::DerefMut;

#[napi(object, js_name = "Secret_Box")]
#[allow(non_camel_case_types)]
pub struct Secret_Box {
    pub ciphertext: Uint8Array,
    pub mac: Uint8Array,
}

#[napi]
pub struct SecretBox {}

#[napi]
impl SecretBox {
    #[napi(constructor)]
    pub fn new() -> Self {
        init().unwrap();
        SecretBox {}
    }

    #[napi(js_name = "crypto_secretbox_detached")]
    pub fn crypto_secretbox_detached(
        &self,
        mut m: Uint8Array,
        n: Uint8Array,
        k: Uint8Array,
    ) -> Secret_Box {
        let mut ms = m.deref_mut();
        let mac = secretbox::seal_detached(
            &mut ms,
            &secretbox::Nonce::from_slice(&n).unwrap(),
            &secretbox::Key::from_slice(&k).unwrap(),
        );

        Secret_Box {
            ciphertext: Uint8Array::new(ms.to_vec()),
            mac: Uint8Array::new(mac.as_ref().to_vec()),
        }
    }

    #[napi(js_name = "crypto_secretbox_easy")]
    pub fn crypto_secretbox_easy(&self, m: Uint8Array, n: Uint8Array, k: Uint8Array) -> Uint8Array {
        let c = secretbox::seal(
            &m,
            &secretbox::Nonce::from_slice(&n).unwrap(),
            &secretbox::Key::from_slice(&k).unwrap(),
        );

        Uint8Array::new(c)
    }

    #[napi(js_name = "crypto_secretbox_keygen")]
    pub fn crypto_secretbox_keygen(&self) -> Uint8Array {
        let k = secretbox::gen_key();

        Uint8Array::new(k.as_ref().to_vec())
    }

    #[napi(js_name = "crypto_secretbox_nonce")]
    pub fn crypto_secretbox_nonce(&self) -> Uint8Array {
        let n = secretbox::gen_nonce();

        Uint8Array::new(n.as_ref().to_vec())
    }

    #[napi(js_name = "crypto_secretbox_open_detached")]
    pub fn crypto_secretbox_open_detached(
        &self,
        mut c: Uint8Array,
        mac: Uint8Array,
        n: Uint8Array,
        k: Uint8Array,
    ) -> Uint8Array {
        let mut ct = c.deref_mut();

        secretbox::open_detached(
            &mut ct,
            &secretbox::Tag::from_slice(&mac).unwrap(),
            &secretbox::Nonce::from_slice(&n).unwrap(),
            &secretbox::Key::from_slice(&k).unwrap(),
        )
        .unwrap();

        Uint8Array::new(ct.to_vec())
    }

    #[napi(js_name = "crypto_secretbox_open_easy")]
    pub fn crypto_secretbox_open_easy(
        &self,
        c: Uint8Array,
        n: Uint8Array,
        k: Uint8Array,
    ) -> Uint8Array {
        let pt = secretbox::open(
            &c,
            &secretbox::Nonce::from_slice(&n).unwrap(),
            &secretbox::Key::from_slice(&k).unwrap(),
        )
        .unwrap();

        Uint8Array::new(pt)
    }

    #[napi(getter)]
    pub fn crypto_secretbox_keybytes(&self) -> u32 {
        CRYPTO_SECRETBOX_KEYBYTES as u32
    }

    #[napi(getter)]
    pub fn crypto_secretbox_macbytes(&self) -> u32 {
        CRYPTO_SECRETBOX_MACBYTES as u32
    }

    #[napi(getter)]
    pub fn crypto_secretbox_messagebytes_max(&self) -> u32 {
        CRYPTO_SECRETBOX_MESSAGEBYTES_MAX as u32
    }

    #[napi(getter)]
    pub fn crypto_secretbox_noncebytes(&self) -> u32 {
        CRYPTO_SECRETBOX_NONCEBYTES as u32
    }

    #[napi(getter)]
    pub fn crypto_secretbox_primitive(&self) -> String {
        CRYPTO_SECRETBOX_PRIMITIVE.to_string()
    }
}
