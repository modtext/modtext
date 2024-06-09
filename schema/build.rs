fn main() {
    let schema_files = ["flatbuffers/storage.fbs"];
    flatbuffers_build::BuilderOptions::new_with_files(&schema_files)
        .set_symlink_directory("src/generated")
        .compile()
        .expect("flatbuffer compilation failed");

    for f in schema_files {
        println!("cargo::rerun-if-changed={}", f);
    }
}
