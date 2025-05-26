// build.rs
use std::process::Command;

fn main() {
    let output = Command::new("buf")
        .args(&[
            "generate",
            "buf.build/cometbft/cometbft",
            "--template",
            "buf.gen.yaml",
        ])
        .status()
        .expect("Failed to run buf generate");

    if !output.success() {
        panic!("buf generate failed");
    }
}
