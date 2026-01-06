
#[cfg(target_arch = "wasm32")]
pub fn random_f32() -> f32 {
    js_sys::Math::random() as f32
}

#[cfg(not(target_arch = "wasm32"))]
pub fn random_f32() -> f32 {
    rand::random::<f32>()
}

