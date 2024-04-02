use operational_transform::{Operation, OperationSeq};
use serde::{Deserialize, Serialize};
use serde_json;
use std::cmp;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn gen_hello_string(a: &str) -> String {
    format!("hello, {}!", a)
}

#[wasm_bindgen] // Clone [clone], PartialEq, Debug [test]
#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
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

    #[inline]
    pub fn is_noop(&self) -> bool {
        self.0.is_noop()
    }

    #[inline]
    pub fn base_len(&self) -> usize {
        self.0.base_len()
    }

    #[inline]
    pub fn target_len(&self) -> usize {
        self.0.target_len()
    }

    pub fn from_str(s: &str) -> OpSeq {
        serde_json::from_str(s).expect("json deserialization failure")
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).expect("json serialization failure")
    }

    // find current position
    pub fn transform_index(&self, position: u32) -> u32 {
        let mut new_index = position as i32;
        let mut index = position as i32;
        // println!("{:?} ----- ops", self.0.ops());
        for op in self.0.ops() {
            match op {
                Operation::Insert(s) => {
                    let n = bytecount::num_chars(s.as_bytes()) as i32;
                    new_index += n;
                }
                Operation::Retain(n) => {
                    let n = *n as i32;
                    index -= n;
                }
                Operation::Delete(n) => {
                    let n = *n as i32;
                    new_index -= cmp::min(index, n);
                    index -= n;
                }
            }
            // justify every loop
            if index < 0 {
                break;
            }
        }
        new_index as u32
    }
}
