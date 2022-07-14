use fuel_vm::consts::VM_MAX_RAM;
use fuels::{prelude::*, tx::ContractId};

use sha2::{Sha256, Digest};

abigen!(
    ContractCallTestContract,
    "test_projects/contract_call/out/debug/contract_call-abi.json"
);
abigen!(
    Asset,
    "test_artifacts/contract_call/out/debug/asset-abi.json"
);

struct CallData {
    arguments: u64,
    function_selector: u64,
    id: ContractId
}

async fn get_contract_call_instance() -> (ContractCallTestContract, Asset, ContractId, ContractId, LocalWallet) {
    let wallet = launch_provider_and_get_wallet().await;

    let asset_id = Contract::deploy(
        "test_artifactos/contract_call/out/debug/asset.bin",
        &wallet,
        TxParameters::default(),
    )
    .await
    .unwrap();
    let asset_instance = Asset::new(id.to_string(), wallet);

    let id = Contract::deploy(
        "test_projects/contract_call/out/debug/contract_call.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "test_artifacts/contract_call/out/debug/contract_call-storage_slots.json".to_string(),
        )),
    )
    .await
    .unwrap();
    let instance = CallFramesTestContract::new(id.to_string(), wallet);

    (instance, asset_instance, id, asset_id, wallet)
}

#[tokio::test]
async fn can_make_contract_call() {
    let (instance, asset_instance, id, asset_id, wallet) = get_contract_call_instance().await;
    let (selector, arguments) = asset_instance.mint_and_send_to_address(100, wallet).await.unwrap();
    let call_data = CallData {
        arguments,
        function_selector: selector,
        id: asset_id,
    };
    instance.make_contract_call(call_data, 0, AssetId::BaseAssetId, 10000);
}