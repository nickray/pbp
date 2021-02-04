use std::time::SystemTime;

use pbp::{dalek::Keypair, KeyFlags, PgpKey};
use rand::rngs::OsRng;
use sha2::Sha256;

fn main() -> Result<(), anyhow::Error> {
    let keypair = Keypair::generate(&mut OsRng);
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();

    let key = PgpKey::from_dalek::<Sha256>(
        &keypair,
        KeyFlags::NONE,
        timestamp as u32,
        "withoutboats",
    );
    println!("{}", key);

    Ok(())
}
