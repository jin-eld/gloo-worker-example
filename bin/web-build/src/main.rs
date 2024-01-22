use std::process::Command;

fn main() {
    let output = Command::new("cargo")
        .arg("build")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .arg("--release")
        .arg("--manifest-path")
        .arg("lib/controller/Cargo.toml")
        .output()
        .expect("Failed to execute cargo build");

    if !output.status.success() {
        panic!("controller: cargo build failed");
    }

    let output = Command::new("wasm-bindgen")
        .arg("target/wasm32-unknown-unknown/release/controller.wasm")
        .arg("--out-dir")
        .arg("./www")
        .arg("--target")
        .arg("web")
        .output()
        .expect("Failed to execute wasm-bindgen");

    if !output.status.success() {
        panic!("controller: wasm-bindgen failed");
    }

    let output = Command::new("cargo")
        .arg("build")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .arg("--release")
        .arg("--manifest-path")
        .arg("lib/worker/Cargo.toml")
        .output()
        .expect("Failed to execute cargo build");

    if !output.status.success() {
        panic!("worker: cargo build failed");
    }

    let output = Command::new("wasm-bindgen")
        .arg("target/wasm32-unknown-unknown/release/worker.wasm")
        .arg("--out-dir")
        .arg("./www")
        .arg("--target")
        .arg("no-modules") // we need no-modules for the worker!
        .output()
        .expect("Failed to execute wasm-bindgen");

    if !output.status.success() {
        panic!("worker: wasm-bindgen failed");
    }
}
