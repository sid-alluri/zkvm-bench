// reference: https://github.com/risc0/risc0/blob/release-1.1/examples/ecdsa/methods/guest/src/bin/ecdsa_verify.rs

#![no_main]
sp1_zkvm::entrypoint!(main);
use k256::{
    ecdsa::{signature::Verifier, Signature, VerifyingKey},
    EncodedPoint,
};

fn main() {
    // Decode the verifying key, message, and signature from the inputs.
    let encoded_verifying_key: EncodedPoint = sp1_zkvm::io::read::<EncodedPoint>();
    let message = sp1_zkvm::io::read::<Vec<u8>>();
    let signature: Signature = sp1_zkvm::io::read::<Signature>();
    let verifying_key = VerifyingKey::from_encoded_point(&encoded_verifying_key).unwrap();

    // Verify the signature, panicking if verification fails.
    verifying_key
        .verify(&message, &signature)
        .expect("ECDSA signature verification failed");

    sp1_zkvm::io::commit(&(encoded_verifying_key, message));
}
