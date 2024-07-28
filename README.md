## Usage


```rust
let signature = "function approve(address,uint256)";
let selector = Selector::new(signature.to_string());
let selector_bytes = selector.turn_function_name_to_bytes().unwrap();
``` 
