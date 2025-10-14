fn main() {
    let x: i32 = 7;
    num(x);
    num_and_char(x, 'h');
    let y = plus_one(5);
    println!("{y}");
    let z = explict_return(6);
    println!("{z}");
}

fn num(x: i32) {
    println!("Some number: {x}");
}

fn num_and_char(x: i32, c: char) {
    println!("{}{}", x, c)
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn explict_return(x: i32) -> i32 {
    return x + 1;
}
