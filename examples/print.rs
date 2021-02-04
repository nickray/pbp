use std::time::SystemTime;

use pbp::{dalek::Keypair, KeyFlags, PgpKey};
use rand::rngs::OsRng;

fn main() -> Result<(), anyhow::Error> {
    let keypair = Keypair::generate(&mut OsRng);
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();

    let key = PgpKey::from_dalek(
        &keypair,
        KeyFlags::NONE,
        timestamp as u32,
        "withoutboats",
    );
    println!("{}", key);

    Ok(())
}
