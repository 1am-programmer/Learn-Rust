/**
 * DESTRUCTURING
 * Destructuring is a way to break down a complex data structure into smaller parts.
 * With destructuring, we can declare multiple variables at once.
 * Tuples are data structure that can hold multiple values.
 */

fn main() {
    let (mut x, y)=(2, 3);
    // If you do not add mut to x, you can't change x, because x is immutable.
    x+=2;
    assert_eq!(x,4);
    assert_eq!(y,3);

    println!("success")
}


//Destructuring 
fn main() {
    let (x,y);
    (x,..) = (1,2);
    [..,y] = [1,2];

    assert_eq!([x,y], [1,2]);
    println!("Success!");
}


