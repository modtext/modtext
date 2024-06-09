#![allow(dead_code, unused_imports)]

/// NOTE: `generated` is generated by build.rs and a symlink is created here.  This is necessary as
/// cargo has no conception of either A) a multi-phase build within a crate, or B) depending on
/// generated code in the build.rs OUT_DIR.  Thus one must either include!() some code from the
/// OUT_DIR or manipulate the source tree in a hacky way (like we do here) to make cargo/rustc aware
/// of the "new" code.
mod generated;

pub mod mod_text {
    pub use super::generated::mod_text::types::*;
}
