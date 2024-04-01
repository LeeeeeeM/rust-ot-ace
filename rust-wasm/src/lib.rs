use operational_transform::OperationSeq;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn gen_hello_string(a: &str) -> String {
    format!("hello, {}!", a)
}

#[wasm_bindgen]
#[derive(Clone, Default)]
pub struct OpSeq(OperationSeq);

#[wasm_bindgen] // for transform
#[derive(Clone, Default)]
pub struct OpSeqPair(OpSeq, OpSeq);

#[wasm_bindgen]
impl OpSeqPair {
    pub fn first(&self) -> OpSeq {
        self.0.clone()
    }

    pub fn second(&self) -> OpSeq {
        self.1.clone()
    }
}

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
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn compose(&self, other: &OpSeq) -> Option<OpSeq> {
        self.0.compose(&other.0).ok().map(Self)
    }

    pub fn delete(&mut self, n: u32) {
        // js cannot recognize u64, for u32 [-2^53~+2^53]
        self.0.delete(n as u64)
    }

    pub fn insert(&mut self, s: &str) {
        self.0.insert(s)
    }

    pub fn retain(&mut self, n: u32) {
        self.0.retain(n as u64)
    }

    pub fn apply(&self, s: &str) -> Option<String> {
        self.0.apply(s).ok()
    }

    pub fn invert(&self, s: &str) -> Self {
        Self(self.0.invert(s))
    }

    pub fn transform(&self, other: &OpSeq) -> Option<OpSeqPair> {
        let (a, b) = self.0.transform(&other.0).ok()?;
        Some(OpSeqPair(Self(a), Self(b)))
    }
}
