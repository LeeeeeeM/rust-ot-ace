use wasm_bindgen::prelude::*;
use operational_transform::OperationSeq;


#[wasm_bindgen]
pub fn gen_hello_string (a: &str) -> String {
    format!("hello, {}!", a)
}


#[wasm_bindgen]
pub struct OpSeq(OperationSeq);

#[wasm_bindgen]
pub struct OpSeqPair(OpSeq, OpSeq);


impl OpSeq {
    
}

impl OpSeqPair {
    
}