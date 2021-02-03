extern crate ed25519_dalek as dalek;
extern crate pbp;
extern crate rand;
extern crate sha2;

use std::time::SystemTime;

use dalek::Keypair;
use failure::Error;
use pbp::{KeyFlags, PgpKey};
use rand::rngs::OsRng;
use sha2::{Sha256, Sha512};

fn main() -> Result<(), Error> {
    let keypair = Keypair::generate(&mut OsRng);
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();

    let key = PgpKey::from_dalek::<Sha256, Sha512>(
        &keypair,
        KeyFlags::NONE,
        timestamp as u32,
        "withoutboats",
    );
    println!("{}", key);

    Ok(())
}
