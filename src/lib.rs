use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn calculate(expression: String) -> String {
    let result = match meval::eval_str(&expression) {
        Ok(val) => val.to_string(),
        Err(_) => "Error".to_string(),
    };
    result
}
