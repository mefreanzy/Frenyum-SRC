use serde::{Deserialize, Serialize};
use crate::util::key::Adress;

pub struct Wallet
{
    adress: Vec<Adress>,    // Adress is a keypair.
    total_received: u64,
    total_sent: u64,
}
