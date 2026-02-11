use arx_gatehouse::ApiDoc;
use std::fs;
use utoipa::OpenApi;

const OPENAPI_FOLDER: &str = "openapi";
const OPENAPI_FILE: &str = "openapi.yaml";

fn main() {
    let spec = ApiDoc::openapi()
        .to_yaml()
        .expect("Failed to serialize OpenAPI");

    let api_file = format!("{OPENAPI_FOLDER}/{OPENAPI_FILE}");

    fs::create_dir_all(OPENAPI_FOLDER).expect("Failed to create spec folder");
    fs::write(format!("{OPENAPI_FOLDER}/{OPENAPI_FILE}"), spec)
        .expect("Failed to create openapi file");

    println!("OpenAPI spec written to {api_file}");
}
