use wasm_bindgen::prelude::{JsValue, wasm_bindgen};
use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
struct JsHashable {
    value: JsValue,
    hashed: u64,
}

impl JsHashable {
    fn new(value: JsValue) -> Self {
        Self {
            value,
            hashed: 0
        }
    }
}


impl Hash for JsHashable {
    fn hash<H: Hasher>(&self, state: &mut H) {
            if let Some(s) = self.value.as_string() {
                s.hash(state);
            }
    }

}

#[wasm_bindgen]
pub fn hasher(value: JsValue) -> JsValue {
    let mut hashable = JsHashable::new(value);
    let mut hasher = DefaultHasher::new();
    hashable.hash(&mut hasher);
    hashable.hashed = hasher.finish();
    JsValue::from_f64(hashable.hashed as f64)
}
