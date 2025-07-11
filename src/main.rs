use alloy::primitives::U256;
use alloy::providers::Provider;
use alloy::providers::ProviderBuilder;

use alloy::rpc::types::TransactionRequest;
use angstrom_types::primitive::AngstromSigner;
use hsm_signer::{Pkcs11Signer, Pkcs11SignerConfig};

#[tokio::main]
async fn main() {
    let hsm_public_key_label = std::env::var("PKL");
    let hsm_private_key_label = std::env::var("PRIV_L");
    let pkcs11_lib_path = std::env::var("PKCS11");
    let signer = Pkcs11Signer::new(
        Pkcs11SignerConfig::from_env_with_defaults(
            hsm_public_key_label.as_ref().unwrap(),
            hsm_private_key_label.as_ref().unwrap(),
            pkcs11_lib_path.clone().unwrap().into(),
            None,
        ),
        Some(1),
    )
    .map(AngstromSigner::new)
    .unwrap();

    let address_to_send = alloy::primitives::address!("0x4838B106FCe9647Bdf1E7877BF73cE8B0BAD5f97");
    let provider = ProviderBuilder::default()
        .with_recommended_fillers()
        .wallet(signer)
        .connect("")
        .await
        .unwrap();

    let tx_req = TransactionRequest::default()
        .to(address_to_send)
        .value(U256::from(670232182593564194u128));

    let tx = provider.send_transaction(tx_req).await.unwrap();
    let hash = tx.watch().await.unwrap();
    println!("{:?}", hash);
}
