// Here we run our codes
// cd my_project
// cargo run

//1 byte = 8bits
fn main() {
    let x: i32 = 5;
    let mut y: u32 = 5; //let y: u32 = 5;
    y = x;
    let z = 10; //let z:i32 = 10;
    println!("success!")
}
//Since type of y is u32, it will not allow us to assign i32 to it, so we just take out the type of y, so rust infers the default int type [i32] to it.

//Solution
fn main() {
    let x: i32 = 5;
    let mut y = 5;
    y = x;
    let z = 10; //let z:i32 = 10;
    println!("success!")
}

// Then it will work
