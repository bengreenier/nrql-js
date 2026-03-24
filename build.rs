use std::env;

fn main() {
    // `napi_build::setup` is for native Node-API artifacts.
    // Skip it for wasm targets so experimental wasm builds don't fail in build.rs.
    if env::var("CARGO_CFG_TARGET_ARCH").ok().as_deref() != Some("wasm32") {
        napi_build::setup();
    }
}
