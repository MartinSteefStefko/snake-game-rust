mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, rust-js-snake-game!");
}

#[wasm_bindgen]
#[derive(Copy,Clone)]

pub struct Vector{
    pub x: f64,
    pub y: f64
}

// to export wasm bindign to javascript this below is needed

#[wasm_bindgen]
pub struct Game{
    pub width: i32,
    pub height: i32,
    // some type for measuring time
    pub speed: f64;
    pub score: i32,
    pub direction: Vector,
    pub food: Vector,
    // snake is sequence of vetors
    snake:Vec<Vector>
}

// this is constructor for the Game
impl Game{
    pub fn new(width: i32,height:i32,speed:f64,snake_length:i32,direction:Vector)-> Game{
        // divide width/2 round() it and substract half of the cell adn assign it to head_x
        let head_x = (f64::from(width)/2_f64).round()-0.5;
        let head_y = (f64::from(height)/2_f64).round()-0.5;
        let head = Vector::new(head_x,head_y)
    }
}
