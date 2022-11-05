use actix_web::{web, Responder};
use serde::Serialize;
use solana_sdk::signer::{keypair::read_keypair_file, Signer};
// extern crate byte_unit;
use byte_unit::Byte;
use futures::TryStreamExt;
use shadow_drive_rust::{models::ShadowFile, ShadowDriveClient, StorageAccountVersion};
use tokio_stream::StreamExt;

#[derive(Serialize)]
struct StorageResp {
    bucket_key: Option<String>,
    trans_id: String,
}

#[derive(Serialize)]
struct FileUploadResp {
    message: String,
    finalized_locations : Vec<String>,
}

pub async fn create_storage() -> impl Responder {
    let file_path: &str = "/home/anuj/Desktop/my-solana-wallet/my-solana-wallet/my-keypair.json";
    let keypair = read_keypair_file(file_path).expect("failed to load keypair at path");
    let pubkey = keypair.pubkey();
    println!("pubkey: {:?}", pubkey);

    let (storage_account_key, _) =
        shadow_drive_rust::derived_addresses::storage_account(&pubkey, 0);
    println!("storage_account_key: {:?}", storage_account_key);

    //create shdw drive client
    let shdw_drive_client = ShadowDriveClient::new(keypair, "https://api.mainnet-beta.solana.com");

    //ensure storage account

    let result_v2 = shdw_drive_client
        .create_storage_account(
            "shadow-drive-rust-test-2",
            Byte::from_str("1KB").expect("failed to parse byte string"),
            StorageAccountVersion::v2(),
        )
        .await
        .expect("failed to create storage account");
    println!("bucket-v2: {:?}", result_v2.shdw_bucket);
    println!("trans-v2: {:?}", result_v2.transaction_signature);
    let mut vec: Vec<StorageResp> = Vec::new();
    vec.push(StorageResp {
        bucket_key: (result_v2.shdw_bucket),
        trans_id: (result_v2.transaction_signature),
    });

    return web::Json(vec);

    // format!("created storage!")
}

pub async fn upload_file() -> impl Responder {
    const KEYPAIR_PATH: &str =
        "/home/anuj/Desktop/my-solana-wallet/my-solana-wallet/my-keypair.json";

    //load keypair from file
    let keypair = read_keypair_file(KEYPAIR_PATH).expect("failed to load keypair at path");
    let pubkey = keypair.pubkey();
    println!("pubkey: {:?}", pubkey);

    let (storage_account_key, _) =
        shadow_drive_rust::derived_addresses::storage_account(&pubkey, 21);
    println!("storage_account_key: {:?}", storage_account_key);

    //create shdw drive client
    let shdw_drive_client = ShadowDriveClient::new(keypair, "https://api.mainnet-beta.solana.com");

    let dir = tokio::fs::read_dir("/home/anuj/Desktop/test2")
        .await
        .expect("failed to read multiple uploads dir");

    let mut files = tokio_stream::wrappers::ReadDirStream::new(dir)
        .filter(Result::is_ok)
        .and_then(|entry| async move {
            Ok(ShadowFile::file(
                entry
                    .file_name()
                    .into_string()
                    .expect("failed to convert os string to regular string"),
                entry.path(),
            ))
        })
        .collect::<Result<Vec<_>, _>>()
        .await
        .expect("failed to create shdw files for dir");

    println!("files: {:?}", files);

    files.push(ShadowFile::bytes(
        String::from("buf.txt"),
        &b"this is a buf test"[..],
    ));

    let upload_results = shdw_drive_client
        .store_files(&storage_account_key, files)
        .await
        .expect("failed to upload files");
    println!("upload results: {:#?}", upload_results);
    let mut upload: Vec<FileUploadResp> = Vec::new();
    upload.push(FileUploadResp {
        finalized_locations: upload_results.finalized_locations,
        message: (upload_results.message),
    });

    return web::Json(upload);

    // format!("file upload!")
}
