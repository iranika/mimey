#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32{
    a + b
}

#[cfg(not(target_family = "wasm"))]
pub fn add(a: i32, b: i32) -> i32{
    a + b
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub fn hello(){
    print!("Hello");
}

#[cfg(not(target_family = "wasm"))]
pub fn hello(){
    print!("Hello");
}
