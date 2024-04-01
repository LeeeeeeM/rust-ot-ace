use operational_transform::OperationSeq;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn gen_hello_string(a: &str) -> String {
    format!("hello, {}!", a)
}

#[wasm_bindgen]
#[derive(Clone, Default)]
pub struct OpSeq(OperationSeq);

#[wasm_bindgen]
#[derive(Clone, Default)]
pub struct OpSeqPair(OpSeq, OpSeq);

#[wasm_bindgen]
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

#[wasm_bindgen]
impl OpSeq {
    pub fn new() -> Self {
        Self::default()
    }
}

#[wasm_bindgen]
impl OpSeqPair {
    pub fn first(&self) -> OpSeq {
        self.0.clone()
    }

    pub fn second(&self) -> OpSeq {
        self.1.clone()
    }
}
