/*
START A RUST PROGRAM
    fn main (){
 }
this is the beginning of any rust program
we declare the function using fn 

PRINT
println!("Hello, world!");
that is how to print in rust

VARAIBLES
- Assigned with the "let" keyword
- A variable can only be used if it has been initialized.
- print! and println! are basically the same thing but the println! adds a new line

SCOPE OF A VARIABLE
- is defined by the curly braces, ie the block of code in which it is declared

FUNCTIONS are reuseable blocks of code

SHADOWING allows a variabble to be redeclared with the same name in the same scope

ASSERT EQUALITY

    assert_eq!(1, 1);
- It takes in two arguments and checks if they are equal.
- If they are equal; the program continues to run, if they are not, the program will panic and exit.


BINDING AND MUTABILITY 

- Initializing a value means assigning it a value at the moment it is created. This involves defining a variable and giving it an initial value.
- Here’s a simple example:

    let x = 5; // Here, `x` is initialized with the value 5.

- Immutability by Default:
 In Rust, variables are immutable by default. This means that once a variable is initialized, you cannot change its value unless you explicitly declare it as mutable:
    let mut y = 10; // Now `y` can be changed later.
    y = 15; // This is allowed because `y` is mutable.
*/

fn main () {
    println!("Hello, world!");
    let x = 5;
    println!("The value of x is: {}", x);
    let x = 6;
    println!("The value of x is: {}", x);
}



