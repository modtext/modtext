use flatbuffers_build::BuilderOptions;

fn main() {
    BuilderOptions::new_with_files(["flatbuffers/storage.fbs"])
        .set_symlink_directory("generated/flatbuffers")
        .compile()
        .expect("flatbuffer compilation failed")
}
