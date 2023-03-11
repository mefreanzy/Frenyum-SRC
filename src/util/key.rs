use secp256k1::{PublicKey,SecretKey,rand::{rngs,SeedableRng}};
use anyhow::Result;

// What is private and public key? Document.
// https://www.ibm.com/docs/en/zos/2.4.0?topic=certificates-public-private-keys
pub struct Adress
{
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
}

pub trait KeyPair {
    fn create_keypair() -> (Vec<u8>, Vec<u8>);
}

impl KeyPair for Adress
{
    // Create private and public key.
    fn create_keypair() -> (Vec<u8>, Vec<u8>)
    {
        let secp = secp256k1::Secp256k1::new();
        let mut rng = rngs::StdRng::seed_from_u64(89899);
        let (sk, pk) = secp.generate_keypair(&mut rng);

        // Convert key pair to vector.
        let sk_hex = hex::encode(&sk[..]);
        let sk_vec = sk_hex.as_bytes().to_vec();

        let pk_hex = hex::encode(&pk.serialize_uncompressed()[..]);
        let pk_vec = pk_hex.as_bytes().to_vec();

        // Return key pair.
        (sk_vec, pk_vec)
    }
}

impl Adress
{
    pub fn new() -> Adress {
        let (sk_vec, pk_vec) = create_keypair();

        Adress {
            private_key: sk_vec,
            public_key: pk_vec,
        }
    }
}
