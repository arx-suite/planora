use arx_gatehouse::ApiDoc;
use std::fs;
use utoipa::OpenApi;

fn main() {
    let spec = ApiDoc::openapi()
        .to_pretty_json()
        .expect("Failed to serialize OpenAPI");

    fs::create_dir_all("openapi").unwrap();
    fs::write("openapi/openapi.json", spec).unwrap();

    println!("OpenAPI spec written to openapi/openapi.json");
}
