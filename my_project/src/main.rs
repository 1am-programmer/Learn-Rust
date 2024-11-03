// Here we run our codes
// cd my_project
// cargo run


fn main() {
    let (x,y);
    (x,..) = (1,2);
    [..,y] = [1,2];

    assert_eq!([x,y], [1,2]);
    println!("Success!");
}
