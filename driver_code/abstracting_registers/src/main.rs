use core::{mem, ptr};
use core::fmt::Debug;

// type-state programming

#[derive(Debug, Default)]
struct  Point<T, U>{
    x: T, 
    y: U
}

impl<T, U > Point<T, U> 
where T: Default + Debug,
      U: Default + Debug,
{
    fn new() -> Point<T, U>{
        Point { x: T::default(), y: U::default() }
    }
}
impl Display for Point{
    
}



fn main(){

}