// reference: https://github.com/risc0/risc0/blob/release-1.1/examples/ecdsa/methods/guest/src/bin/ecdsa_verify.rs

use k256::{
    ecdsa::{signature::Verifier, Signature, VerifyingKey},
    EncodedPoint,
};
use risc0_zkvm::guest::env;

fn main() {
    // Decode the verifying key, message, and signature from the inputs.
    let (encoded_verifying_key, message, signature): (EncodedPoint, Vec<u8>, Signature) =
        env::read();
    let verifying_key = VerifyingKey::from_encoded_point(&encoded_verifying_key).unwrap();

    // Verify the signature, panicking if verification fails.
    verifying_key
        .verify(&message, &signature)
        .expect("ECDSA signature verification failed");

    // Commit to the journal the verifying key and message that was signed.
    env::commit(&(encoded_verifying_key, message));
}
