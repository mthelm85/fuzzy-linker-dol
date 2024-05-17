pub mod bktree;
pub mod levenshtein;
mod utils;

use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    utils::set_panic_hook();
    console_error_panic_hook::set_once();
    Ok(())
}

#[wasm_bindgen]
pub struct BKTreeWrapper {
    tree: bktree::BKTree,
}

#[wasm_bindgen]
impl BKTreeWrapper {
    #[wasm_bindgen]
    pub fn new() -> BKTreeWrapper {
        BKTreeWrapper {
            tree: bktree::BKTree::new(),
        }
    }

    pub fn insert(&mut self, value: String, row_data: Vec<String>) {
        self.tree.insert(value, row_data);
    }

    pub fn search(&self, value: String, tolerance: f64) -> JsValue {
        let results = self.tree.search(&value, tolerance);
        to_value(&results).unwrap()
    }
}
