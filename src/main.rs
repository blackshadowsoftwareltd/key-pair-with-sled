use std::sync::{Mutex, OnceLock};

use libp2p::identity::Keypair;
use sled::{Db, IVec};

const KEYS: &str = "PublicKey";
static DBREF: OnceLock<Mutex<Db>> = OnceLock::new();

#[tokio::main]
async fn main() {
    init_db().await;
    write_db().await;
    read_db().await;
}
fn generate_key_peer_protobuf() -> Vec<u8> {
    let id_keys = Keypair::generate_ed25519();
    id_keys.to_protobuf_encoding().unwrap()
}

async fn write_db() {
    let key = generate_key_peer_protobuf();
    let db = DBREF.get().unwrap().lock().unwrap();
    db.insert(KEYS, key).unwrap();
}

async fn read_db() {
    let db = DBREF.get().unwrap().lock().unwrap();
    let ivec = db.get(KEYS).unwrap().unwrap();
    let key = ivect_to_vec(ivec);
    println!("Key: {:?}", key);
}

fn ivect_to_vec(iv: IVec) -> Vec<u8> {
    iv.as_ref().to_vec()
}

async fn init_db() {
    let doc_path = dirs::document_dir().unwrap().join("DB");
    let db = sled::open(doc_path).unwrap();
    DBREF.get_or_init(|| Mutex::new(db));
}

/*
let id_keys = Keypair::generate_ed25519();
let peer_id = PeerId::from(id_keys.public());
println!("PeerId: {:?}", peer_id);
let protobuf = id_keys.to_protobuf_encoding().unwrap();
println!("Protobuf: {:?}", protobuf.len());
let new_key = Keypair::from_protobuf_encoding(&protobuf).unwrap();
let peer_id = PeerId::from(new_key.public());
println!("PeerId: {:?}", peer_id);
 */
