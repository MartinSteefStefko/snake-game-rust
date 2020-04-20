mod utils;

use wasm_bindgen::prelude::*;
use js_sys::Array;

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
    pub speed: f64,
    pub score: i32,
    pub direction: Vector,
    pub food: Vector,
    // snake is sequence of vetors
    snake:Vec<Vector>
}

// constructor of vector struct
#[wasm_bindgen]
impl Vector{
    #[wasm_bindgen(constructor)]
    pub fn new(x:f64,y:f64)-> Vector{
        Vector{x,y}
    }
    // subtract returns a reference to another Vector
    pub fn subtract(&self,other: &Vector)-> Vector {
        // create new instance of a Vector where x is self.x-other.x,
        // where other is &Vector returned vector? 
        Vector::new(self.x - other.x,self.y - other.y)
    }
    //  create new vector 
    pub fn scale_by(&self,number:f64) -> Vector {
        Vector::new(self.x*number,self.y*number)
    }
}

// this is constructor for the Game
#[wasm_bindgen]
impl Game{
    #[wasm_bindgen(constructor)]
    pub fn new(width: i32,height:i32,speed:f64,snake_length:i32,direction:Vector)-> Game{
        // divide width/2 round() it and substract half of the cell adn assign it to head_x
        let head_x = (f64::from(width)/2_f64).round()-0.5;
        let head_y = (f64::from(height)/2_f64).round()-0.5;
        let head = Vector::new(head_x,head_y);
        // subtracting direction scaled by snake_length from head
        let tailtip = head.subtract(&direction.scale_by(f64::from(snake_length)));
        // snake will be a vector of tailtip and head
        let snake = vec![tailtip,head];
        // food will be half of the cells located in the first cell
        // TODO: place foo in random cells
        let food = Vector::new(0.5,0.5);

        // Returning the instance of the game struct
        Game {
            width: width,
            height: height,
            // number of cells snake cross in one second
            speed: speed,
            snake: snake,
            direction: direction,
            food: food,
            score: 0
        }

    }

    pub fn get_snake(&self)->Array{
            self.snake.clone().into_iter().map(JsValue::from).collect()
            // colectin all the snake from self.snake.clone().into_iter() and mappi it to JsValue::
    }
}
