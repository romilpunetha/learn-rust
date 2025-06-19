use std::process::Command;
use std::path::Path;

fn main() {
    let thrift_files = [
        "thrift/common.thrift",
        "thrift/user.thrift", 
        "thrift/post.thrift",
        "thrift/group.thrift",
        "thrift/associations.thrift",
    ];

    // Create the generated models directory
    std::fs::create_dir_all("src/models").expect("Failed to create models directory");

    // Generate Rust code for each thrift file
    for thrift_file in &thrift_files {
        if Path::new(thrift_file).exists() {
            let output = Command::new("thrift")
                .arg("--gen")
                .arg("rs")
                .arg("-out")
                .arg("src/models")
                .arg(thrift_file)
                .output()
                .expect("Failed to execute thrift compiler");

            if !output.status.success() {
                panic!(
                    "Thrift compilation failed for {}: {}",
                    thrift_file,
                    String::from_utf8_lossy(&output.stderr)
                );
            }

            println!("cargo:rerun-if-changed={}", thrift_file);
        }
    }

    println!("cargo:rerun-if-changed=build.rs");
}