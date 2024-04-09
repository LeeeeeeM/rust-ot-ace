use operational_transform::OperationSeq;
use serde::{Deserialize, Serialize};
use serde_json;
use wasm_bindgen::prelude::*;

mod util;

#[wasm_bindgen] // Clone [clone], PartialEq, Debug [test]
#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
pub struct OpSeq(OperationSeq);

#[wasm_bindgen] // for transform
#[derive(Clone, Default, Serialize)]
pub struct OpSeqPair(OpSeq, OpSeq);

#[wasm_bindgen]
impl OpSeqPair {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn first(&self) -> OpSeq {
        self.0.clone()
    }

    pub fn second(&self) -> OpSeq {
        self.1.clone()
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).expect("json serialization failure")
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
}
