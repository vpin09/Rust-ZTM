// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn main() {

    addNUM(5,4);
}
fn addNUM(one :usize,two:usize){
    let result=one+two;
    println!("Result {:?}",result);
}