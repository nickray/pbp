use std::time::SystemTime;

use pbp::{dalek::Keypair, KeyFlags, PgpKey, PgpSig, SigType};
use sha2::Sha256;

const DATA: &[u8] = b"How will I ever get out of this labyrinth?";

fn main() -> Result<(), anyhow::Error> {
    let keypair = Keypair::generate(&mut rand::rngs::OsRng);
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();
    let public_key = PgpKey::from_dalek::<Sha256>(
        &keypair,
        KeyFlags::NONE,
        timestamp as u32,
        "withoutboats",
    );
    let sig = PgpSig::from_dalek::<Sha256>(
        &keypair,
        DATA,
        public_key.fingerprint(),
        SigType::BinaryDocument,
        timestamp as u32,
    );
    if sig.verify_dalek::<Sha256>(&keypair.public, DATA) {
        Ok(println!("Verified successfully."))
    } else {
        Err(anyhow::anyhow!("Could not verify."))
    }
}
