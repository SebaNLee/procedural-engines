
#[cfg(target_arch = "wasm32")]
pub fn random_f32() -> f32 {
    js_sys::Math::random() as f32
}

#[cfg(not(target_arch = "wasm32"))]
pub fn random_f32() -> f32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();
    nanos as f32 % 1.0
}

