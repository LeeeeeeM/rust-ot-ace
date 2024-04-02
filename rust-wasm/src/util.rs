use wasm_bindgen::prelude::*;

#[wasm_bindgen] // test reture String
pub fn gen_hello_string(a: &str) -> String {
    format!("hello, {}!", a)
}

#[wasm_bindgen] // test Class
pub struct Test {
    name: String,
    age: usize,
}

#[wasm_bindgen]
impl Test {
    #[wasm_bindgen(constructor)]
    pub fn new(age: usize, name: &str) -> Self {
        Self {
            age,
            name: name.to_string(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_age(&self) -> usize {
        self.age
    }
}