use std::process::Command;
use std::env;

fn main() {

    // Kør OpenAPI Generator CLI kommandoen
    let output = Command::new("cmd")
        .args(&["/C", "openapi-generator-cli generate -i openapi/bookstore.yaml -g rust -o ./bookstore_api"])
        .output()
        .expect("Failed to execute OpenAPI Generator");

    if !output.status.success() {
        panic!(
            "OpenAPI Generator failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    println!("cargo:rerun-if-changed=openapi/bookstore.yaml");
}
