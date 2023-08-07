use std::str::FromStr;

use bitcoin::secp256k1::{constants, PublicKey, SecretKey};
use bitcoin::util::bip32::{ChildNumber, ExtendedPrivKey};
use bitcoin::util::key::Secp256k1;
use bitcoin::Network;
use lightning::util::message_signing;

pub struct Signer {
    pub secret_key_bytes: [u8; 32],
    pub node_id: String,
}

impl Signer {
    // If you do not need a separate on-chain wallet or you already derived the seed yourself for the lightning wallet,
    //  you can use this method.
    //  The lightning node's key is derived directly from the seed.
    pub fn from_seed(seed: [u8; 64]) -> Signer {
        // Note that when we aren't serializing the key, network doesn't matter
        let master_key = ExtendedPrivKey::new_master(Network::Bitcoin, &seed).unwrap();
        let ldk_seed_bytes: [u8; constants::SECRET_KEY_SIZE] =
            master_key.private_key.secret_bytes();
        Signer::from_ldk_seed(ldk_seed_bytes)
    }

    // If you have a separate on-chain wallet and you want to manage it with the same mnemonic as the lightning node's mnemonic,
    //  you will have a derived lightning seed and you can use this method.
    pub fn from_ldk_seed(seed: [u8; 32]) -> Signer {
        let secp_ctx = Secp256k1::new();
        // Note that when we aren't serializing the key, network doesn't matter
        match ExtendedPrivKey::new_master(Network::Bitcoin, &seed) {
            Ok(master_key) => {
                let node_secret = master_key
                    .ckd_priv(&secp_ctx, ChildNumber::from_hardened_idx(0).unwrap())
                    .expect("Your RNG is busted")
                    .private_key;
                let node_id = node_secret.public_key(&secp_ctx);
                let secret_key_bytes: [u8; 32] = node_secret.secret_bytes();
                Self {
                    secret_key_bytes,
                    node_id: hex::encode(node_id.serialize()),
                }
            }
            Err(_) => panic!("Your rng is busted"),
        }
    }

    pub fn sign(&self, message: String) -> String {
        let secret_key = SecretKey::from_slice(&self.secret_key_bytes).unwrap();
        let msg = message.as_str().as_bytes();
        message_signing::sign(msg, &secret_key).unwrap()
    }
}

impl From<Signer> for SecretKey {
    fn from(signer: Signer) -> Self {
        SecretKey::from_slice(&signer.secret_key_bytes).unwrap()
    }
}

pub fn verify(message: String, signature: String, public_key: String) -> bool {
    let msg = message.as_str().as_bytes();
    message_signing::verify(
        msg,
        signature.as_str(),
        &PublicKey::from_str(public_key.as_str()).unwrap(),
    )
}

pub fn recover_node_id(message: String, signature: String) -> String {
    let msg = message.as_str().as_bytes();
    let pub_key = message_signing::recover_pk(msg, signature.as_str()).unwrap();
    hex::encode(pub_key.serialize())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn signing_from_seed_works() {
        let seed_hex = "1572722d7ed2a295469048fcf0d9bf13449c300328a25011d9dc1303cd45516d8a4045681e9a10f5366fbd78abd3c0ea33d650e34ebec991c9c6e97279354e56";
        let seed_bytes = hex::decode(seed_hex).unwrap();
        let seed_array = match <[u8; 64]>::try_from(&seed_bytes[..]) {
            Ok(array) => array,
            Err(_) => panic!("Invalid seed length"),
        };
        let signer = Signer::from_seed(seed_array);
        let message = String::from("test message");
        let zbase32_sig = signer.sign(message);

        assert_eq!(zbase32_sig, "dh6g5jcckfwrpc8mhwgm74m5cddkc755t4x1p477baf8bjwd769ie8chgtauc3jukce5ctgdujw5b3hsj5mpmp6d9ijfkih6ymn91jg9");
    }

    #[test]
    fn signing_from_ldk_seed_works() {
        let ldk_seed_hex = "426540629d356f207fd792c0215e787ded943a1c405a4353f7174926bb6fe129";
        let ldk_seed_bytes = hex::decode(ldk_seed_hex).unwrap();
        let ldk_seed_array = match <[u8; 32]>::try_from(&ldk_seed_bytes[..]) {
            Ok(array) => array,
            Err(_) => panic!("Invalid seed length"),
        };
        let signer = Signer::from_ldk_seed(ldk_seed_array);
        let message = String::from("test message");
        let zbase32_sig = signer.sign(message);
        assert_eq!(zbase32_sig, "rdgd7i3odxbap66cgwpwu7wtachqgfs8naxrpb459e9uxa9kuce8g9cg3nstbziq5wpw7wpz5gjht1zn1s5nnngyc4jpc3wkowezm9kx");
    }

    #[test]
    fn verifying_works() {
        let message = String::from("test message");
        let zbase32_sig = "rdgd7i3odxbap66cgwpwu7wtachqgfs8naxrpb459e9uxa9kuce8g9cg3nstbziq5wpw7wpz5gjht1zn1s5nnngyc4jpc3wkowezm9kx";
        let node_id = "036bb0074b227987242144f3c50c7bddc75e1521020ea67f9a80290f7faaf6702a";
        assert!(verify(
            message,
            zbase32_sig.to_string(),
            node_id.to_string()
        ));
    }

    #[test]
    fn verifying_fails() {
        let message = String::from("failing message");
        let zbase32_sig = "rdgd7i3odxbap66cgwpwu7wtachqgfs8naxrpb459e9uxa9kuce8g9cg3nstbziq5wpw7wpz5gjht1zn1s5nnngyc4jpc3wkowezm9kx";
        let node_id = "036bb0074b227987242144f3c50c7bddc75e1521020ea67f9a80290f7faaf6702a";
        assert!(!verify(
            message,
            zbase32_sig.to_string(),
            node_id.to_string()
        ));
    }

    #[test]
    fn recovering_node_id_works() {
        let seed_hex = "1572722d7ed2a295469048fcf0d9bf13449c300328a25011d9dc1303cd45516d8a4045681e9a10f5366fbd78abd3c0ea33d650e34ebec991c9c6e97279354e56";
        let seed_bytes = hex::decode(seed_hex).unwrap();
        let seed_array = match <[u8; 64]>::try_from(&seed_bytes[..]) {
            Ok(array) => array,
            Err(_) => panic!("Invalid seed length"),
        };
        let signer = Signer::from_seed(seed_array);
        let node_id = signer.node_id.clone();
        let message = String::from("test message");
        let zbase32_sig = signer.sign(message.clone());

        assert_eq!(recover_node_id(message, zbase32_sig), node_id);
    }
}
