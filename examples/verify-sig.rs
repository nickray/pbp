use std::{env, fs, path::PathBuf};

use pbp::{PgpKey, PgpSig};

fn main() -> Result<(), anyhow::Error> {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let props = root.join("examples").join("props");

    let sig: String = fs::read_to_string(props.join("sig.txt")).unwrap();
    let public_key: String = fs::read_to_string(props.join("key.txt")).unwrap();
    let data: String = fs::read_to_string(props.join("data.txt")).unwrap();

    let sig = PgpSig::from_ascii_armor(&sig).unwrap();
    let public_key = PgpKey::from_ascii_armor(&public_key).unwrap();

    if sig.verify_dalek(&public_key.to_dalek().unwrap(), data.as_bytes()) {
        Ok(println!("Verified signature."))
    } else {
        Err(anyhow::anyhow!("Could not verify signature."))
    }
}
