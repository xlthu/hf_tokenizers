//! C++ interface of Huggingface Tokenizers

use std::vec::Vec;

use tokenizers::{models::bpe::BPE, Tokenizer as HFTokenizer};

#[cxx::bridge(namespace = "hf")]
mod ffi {
    extern "Rust" {
        type Tokenizer;
        pub fn default_tokenizer() -> Box<Tokenizer>;
        pub fn load_tokenizer(file_path: &str) -> Box<Tokenizer>;
        pub fn ok(&self) -> bool;
        pub fn encode(&self, text: &str, add_special_tokens: bool) -> Vec<u32>;
        pub fn decode(&self, ids: &[u32], skip_special_tokens: bool) -> String;
        pub fn token_to_id(&self, token: &str) -> u32;
        pub fn id_to_token(&self, id: u32) -> String;
    }
}

pub struct Tokenizer {
    tokenizer: HFTokenizer,
    ok: bool
}

pub fn default_tokenizer() -> Box<Tokenizer> {
    Box::new(Tokenizer{
        tokenizer : HFTokenizer::new(BPE::default()),
        ok: false
    })
}

pub fn load_tokenizer(file_path: &str) -> Box<Tokenizer> {
    match HFTokenizer::from_file(file_path) {
        Ok(t) => Box::new(Tokenizer{tokenizer : t, ok : true}),
        _ => default_tokenizer(),
    }
}

impl Tokenizer {
    pub fn ok(&self) -> bool {
        self.ok
    }

    pub fn encode(&self, text: &str, add_special_tokens: bool) -> Vec<u32> {
        match self.tokenizer.encode(text, add_special_tokens) {
            Ok(e) => e.get_ids().to_vec(),
            _ => vec![],
        }
    }

    pub fn decode(&self, ids: &[u32], skip_special_tokens: bool) -> String {
        match self.tokenizer.decode(ids, skip_special_tokens) {
            Ok(s) => s,
            _ => String::new(),
        }
    }

    pub fn token_to_id(&self, token: &str) -> u32 {
        self.tokenizer.token_to_id(token).unwrap()
    }

    pub fn id_to_token(&self, id: u32) -> String {
        self.tokenizer.id_to_token(id).unwrap()
    }
}
