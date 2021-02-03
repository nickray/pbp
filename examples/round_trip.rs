extern crate ed25519_dalek as dalek;
extern crate pbp;
extern crate rand;
extern crate sha2;

use std::time::SystemTime;

use dalek::Keypair;
use failure::Error;
use pbp::{KeyFlags, PgpKey, PgpSig, SigType};
use rand::rngs::OsRng;
use sha2::Digest;
use sha2::{Sha256, Sha512};

const DATA: &[u8] = b"How will I ever get out of this labyrinth?";

fn main() -> Result<(), Error> {
    let keypair = Keypair::generate(&mut OsRng);
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();
    let key = PgpKey::from_dalek::<sha2::Sha256, sha2::Sha512>(
        &keypair,
        KeyFlags::NONE,
        timestamp as u32,
        "withoutboats",
    );
    let sig = PgpSig::from_dalek::<Sha256, Sha512>(
        &keypair,
        DATA,
        key.fingerprint(),
        SigType::BinaryDocument,
        timestamp as u32,
    );
    if sig.verify_dalek::<Sha256, Sha512, _>(&keypair.public, |hasher| {
        hasher.update(DATA);
    }) {
        println!("Verified successfully.");
    } else {
        println!("Could not verify.");
    }

    Ok(())
}
