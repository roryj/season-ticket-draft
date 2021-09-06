use std::fmt::Display;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SomeStruct {
    text: String,
}

#[wasm_bindgen]
impl SomeStruct {
    pub fn new(text: String) -> Self {
        SomeStruct { text }
    }

    #[wasm_bindgen]
    pub fn render(&self) -> String {
        format!("{}", self)
    }
}

impl Display for SomeStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (0..100).into_iter().for_each(|num| {
            writeln!(f, "{} - {}", self.text, num);
        });

        Ok(())
    }
}
