// Here we run our codes
// cd my_project
// cargo run

//1 byte = 8bits

fn main() {
    let (x, y);
    (x, ..) = (1, 2);
    [.., y] = [1, 2];

    assert_eq!([x, y], [1, 2]);
    println!("Success!");
}

//Exercise.. Remove something to make it work

fn main() {
    let x: i32 = 5;
    let mut y: u32 = 5; //let y: u32 = 5;
    y = x;
    let z = 10; //let z:i32 = 10;
    println!("success!")
}

// Then it will work

//Fill in the blanks to make the code compile.
fn main() {
    let v: u16 = 38_u8 as __; // We can directly annotate a type to a variable.
                              //So this variable v is expecting a u16 value, but it got a u8 value, so we need to convert it to same type using the 'as' keyword
                              //So it is now  { let v: u16 = 38_u8 as __;}
    println!("success!")
}
