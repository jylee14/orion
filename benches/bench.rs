#![feature(test)]
extern crate orion;
extern crate test;

use orion::hazardous::kdf::hkdf;
use orion::hazardous::kdf::pbkdf2;
use orion::hazardous::mac::hmac;
use orion::hazardous::mac::poly1305;
use orion::hazardous::stream::*;
use orion::hazardous::xof::cshake;
use test::Bencher;

#[bench]
fn bench_hmac(b: &mut Bencher) {
    b.iter(|| {
        let mut mac = hmac::init(&hmac::SecretKey::from_slice(&[0x01; 64]));
        mac.update(&[0x01; 64]).unwrap();
        mac.finalize().unwrap();
    });
}

#[bench]
fn bench_hkdf(b: &mut Bencher) {
    let mut okm_out = [0u8; 64];
    b.iter(|| {
        hkdf::derive_key(
            &hkdf::Salt::from_slice(&[0x01; 64]).unwrap(),
            &[0x01; 64],
            Some(&[0x01; 64]),
            &mut okm_out,
        ).unwrap();
    });
}

#[bench]
fn bench_pbkdf2(b: &mut Bencher) {
    let mut dk_out = [0u8; 64];
    b.iter(|| {
        pbkdf2::derive_key(
            &[0x01; 64],
            &pbkdf2::Salt::from_slice(&[0x01; 64]).unwrap(),
            10000,
            &mut dk_out,
        ).unwrap();
    });
}

#[bench]
fn bench_cshake(b: &mut Bencher) {
    let mut hash_out = [0u8; 64];
    b.iter(|| {
        let mut cshake = cshake::init(&[0x01; 64], None).unwrap();
        cshake.update(&[0x01; 64]).unwrap();
        cshake.finalize(&mut hash_out).unwrap();
    });
}

#[bench]
fn bench_chacha20_encrypt(b: &mut Bencher) {
    let plaintext = [0u8; 256];
    let mut ciphertext = [0u8; 256];

    b.iter(|| {
        chacha20::encrypt(
            &chacha20::SecretKey::from_slice(&[0u8; 32]).unwrap(),
            &chacha20::Nonce::from_slice(&[0u8; 12]).unwrap(),
            0,
            &plaintext,
            &mut ciphertext,
        ).unwrap();
    });
}

#[bench]
fn bench_chacha20_decrypt(b: &mut Bencher) {
    let mut plaintext = [0u8; 256];
    let ciphertext = [0u8; 256];

    b.iter(|| {
        chacha20::decrypt(
            &chacha20::SecretKey::from_slice(&[0u8; 32]).unwrap(),
            &chacha20::Nonce::from_slice(&[0u8; 12]).unwrap(),
            0,
            &ciphertext,
            &mut plaintext,
        ).unwrap();
    });
}

#[bench]
fn bench_poly1305(b: &mut Bencher) {
    b.iter(|| {
        let mut mac =
            poly1305::init(&poly1305::OneTimeKey::from_slice(&[0x01; 32]).unwrap()).unwrap();
        mac.update(&[0x01; 64]).unwrap();
        mac.finalize().unwrap();
    });
}

#[bench]
fn bench_xchacha20_encrypt(b: &mut Bencher) {
    let plaintext = [0u8; 256];
    let mut ciphertext = [0u8; 256];

    b.iter(|| {
        xchacha20::encrypt(
            &xchacha20::SecretKey::from_slice(&[0u8; 32]).unwrap(),
            &xchacha20::Nonce::from_slice(&[0u8; 24]).unwrap(),
            0,
            &plaintext,
            &mut ciphertext,
        ).unwrap();
    });
}

#[bench]
fn bench_xchacha20_decrypt(b: &mut Bencher) {
    let mut plaintext = [0u8; 256];
    let ciphertext = [0u8; 256];

    b.iter(|| {
        xchacha20::decrypt(
            &xchacha20::SecretKey::from_slice(&[0u8; 32]).unwrap(),
            &xchacha20::Nonce::from_slice(&[0u8; 24]).unwrap(),
            0,
            &ciphertext,
            &mut plaintext,
        ).unwrap();
    });
}
