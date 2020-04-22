mod utils;

use wasm_bindgen::prelude::*;
use js_sys::Array;
use rand::Rng;

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

static EPSILON:f64 =0.0000001;
fn are_equal(one:f64,another:f64) -> bool {
    // if difference between two compared values is less than small value we assume that the values are equal
    (one - another).abs()< EPSILON
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

    pub fn add(&self,other: &Vector)-> Vector {
        // create new instance of a Vector where x is self.x-other.x,
        // where other is &Vector returned vector? 
        Vector::new(self.x + other.x,self.y + other.y)
    }
    //  create new vector 
    pub fn scale_by(&self,number:f64) -> Vector {
        Vector::new(self.x*number,self.y*number)
    }

    pub fn length(&self) ->f64{
        self.x.hypot(self.y)
    }
    // this will take self vector and return value from 0 to 1 according to the max number of current vector
    pub fn normalize(&self) -> Vector{
        // 
        self.scale_by(1_f64/self.length())
    }
}

pub struct Segment<'a>{
    pub start: &'a Vector,
    // vouu after initialize start as vector it automatically suggest end with same syntax
    pub end: &'a Vector,
}
// create constructor for the segment
impl<'a> Segment<'a>{
    // init public function new(start: &'a Vector, end: &'a Vector) is will return Segment
    pub fn new(start: &'a Vector, end: &'a Vector) -> Segment<'a>{
        Segment{start: start, end: end}
    }

    pub fn get_vector(&self) -> Vector{
        // subtracting end from start to get length of the snake
        self.end.subtract(&self.start)
    }

    pub fn length(&self) ->f64{
        // actuallz get length on the vector
        self.get_vector().length()
    }

    pub fn is_point_inside(&self,point:&Vector)->bool{
        // is the point inside of the snake
        let first=Segment::new(self.start,point);
        let second=Segment::new(point,self.end);
        are_equal(self.length(),first.length()+second.length())
    }
}

// 
fn get_segments_from_vectors(vectors:&[Vector])-> Vec<Segment> {
        // simple, if you have 5 edges you shoud have 4 segments
    let pairs = vectors[..vectors.len()-1].iter().zip(&vectors[1..]);
    pairs
        .map(|(s,e)| Segment::new(s,e))
        .collect::<Vec<Segment>>()
}

// get_food() will take width,height, snake vector and it will return vector
fn get_food(width: i32, height: i32, snake: &[Vector])->Vector{
    // TODO implement

    let segments = get_segments_from_vectors(snake);
    // 
    let mut free_positions: Vec<Vector> = Vec::new();
    for x in 0..width {
        for y in 0..height{
            // go from 0 up to current width and height of the window
            // let point be vector or new x and y coordinates originated from width and height converted to f64 from i32 +0.5 
            let point = Vector::new(f64::from(x) + 0.5,f64::from(y) + 0.5);
            // checking that none of the segmets of snake intersect the point
            if segments.iter().all(|s| !s.is_point_inside(&point)) {
                // if there are no intersetions push point to free_positions
                free_positions.push(point);
            }
        }
    }
    // create random indexes in range of (0,free_positions.len())
    let index = rand::thread_rng().gen_range(0,free_positions.len());
    free_positions[index]

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
        let food = get_food(width,height,&snake);

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

    fn process_movement(&mut self,timespan:f64) {
        // 1 block/per second * 2 seconds duration of last update
        let distance = self.speed*timespan;
        // tail will be mutable and also be new()vector
        let mut tail : Vec<Vector>= Vec::new();
        let mut snake_distance = distance;
        while self.snake.len()>1{
            // remove first element of the snake
            let point = self.snake.remove(0);
            // let next to be new snake [0] element and store it as point
            let next = &self.snake[0];
            // create segment as new Segment with &point, next as argument
            let segment =Segment::new(&point,next);
            let length=segment.length();
            if length>= snake_distance {
                let vector = segment.get_vector().normalize().scale_by(snake_distance);
                // push to the tail point + &vector
                tail.push(point.add(&vector));
                break;
            }else{
                // 
                // subtract length from snake_distance
                snake_distance-=length;
            }

        }
        // updating the tail with what has last from the snake
        tail.append(&mut self.snake);
        self.snake =tail;
        // old head = popped Game.snake
        let old_head = self.snake.pop().unwrap();
        // adding Game.direction scaled by distance and assigning it to new head
        let new_head = old_head.add(&self.direction.scale_by(distance));
        // push new head into Game.snake
        self.snake.push(new_head);
    }

    // function process which receives self and timespan which is duration from last update
    pub fn process(&mut self,timespan:f64){
        self.process_movement(timespan);
    }
}
