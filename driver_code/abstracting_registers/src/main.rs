// mod sys_tick;
use core::ptr;


#[derive(Debug)]
struct Purei32{
    data: i32
}

fn main(){

    // every time you run a `let` statement, a new address in the stack must be instatiated. eg 
    let x = 10;  
    println!("address of x: {:?}", ptr::addr_of!(x));
    let x = 20; 
    println!("new address of x: {:?}", ptr::addr_of!(x));


    let mut x = Purei32{data: 20}; 
    let x_ptr = ptr::addr_of_mut!(x);

    // let mut y = unsafe { *x_ptr };
    let mut y = x;
    let y_ptr = ptr::addr_of!(y);
    y.data = 40 ; 

    // println!("x = {:?}", x); // fails
    println!("x = {:?}", x_ptr);
    println!("y = {:?}", y_ptr);
}