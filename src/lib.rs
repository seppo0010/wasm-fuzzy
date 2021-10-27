use wasm_bindgen::prelude::*;
use wasm_bindgen::throw_str;
use serde::{Deserialize, Serialize};

use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

#[derive(Serialize, Deserialize)]
pub struct Document {
    content: String,
    key: String,
}
#[wasm_bindgen]
pub struct Matcher {
    documents: Vec<Document>,
}

#[wasm_bindgen]
impl Matcher {
    #[wasm_bindgen]
    pub fn from_documents(data: JsValue) -> Result<Matcher, JsValue> {
        let docs: Vec<Document> = serde_wasm_bindgen::from_value(data)?;
        Ok(Matcher { documents: docs })
    }

    #[wasm_bindgen]
    pub fn from_json(data: &str) -> Result<Matcher, JsValue> {
        let docs: Vec<Document> = match serde_json::from_str(data) {
            Ok(d) => d,
            Err(_) => throw_str("failed to parse json"),
        };
        Ok(Matcher { documents: docs })
    }

    #[wasm_bindgen]
    pub fn indices(&self, criteria: &str) -> Result<JsValue, JsValue> {
        let matcher = SkimMatcherV2::default();
        Ok(serde_wasm_bindgen::to_value(
            &self.documents
                .iter()
                .filter_map(|d| {
                    matcher.fuzzy_match(&d.content, criteria)
                    .map(|_| d.key.clone())
                })
                .collect::<Vec<String>>()
        )?)
    }
}
