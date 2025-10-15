fn main() {
    // Basic scope within whole block of function.
    // These are string literals. Immutable, known size at compile time, put on stack.
    let s = "string";

    {
        // Valid in this block.
        let t = "tring";
    }
    // t no longer valid.

    // Heap-allocated strings. Strings can be mutated.
    let mut hs = String::from("string");

    // Assign 5 to x, and y is a copy of x, two 5's are put onto the stack.
    // Because the size of x is known at compile time, it's trivially quick to make copies
    // on the stack, thus, for stack-allocated variables, all copies are deep.
    let x = 5;
    let y = x;

    // s1 is a pointer, the pointer contains the address, the length, the capacity.
    // The data is stored in the heap.
    // s2 is copy of the pointer, NOT the data.
    let s1 = String::from("hello");
    let s2 = s1; // s1 is no longer valid, attempting to use it will result in a compilation error.

    // println!("{s1}"); // This would fail becuase s1 is no longer valid.

    // s is a point, data on heap
    let mut s = String::from("taco");
    // s is a pointer to new data, old data is dropped from heap.
    s = String::from("bell");

    // A deep copy. A new ptr is made, along with a new allocation of data on heap.
    // Deep copies are much more expensive than moving/shallow copies.
    let s2 = s.clone();
    take_ownership(s2);
    // println!("{s2}"); // Would cause a comp error because s2 was moved in prior fn call.

    let x: u32 = 5;
    makes_copy(x); // A copy is made, because integers are stored on stack.
    makes_copy(x + 1); // Another copy is made.
    println!("x is still valid in parent: {x}");

    // Example of return s3 back from function call.
    let s3 = String::from("Hello");
    let (s3, len) = calculate_length(s3); // s3 reassigned via shadowing.
    println!("s3({s3}) is {len} long.");
}

fn take_ownership(s: String) {
    // s is moved into scope, calls to s after this function call in parent will result in
    // a compile error.
    println!("take_ownership owns s: {s}");
} // s goes out of scope, is dropped.

fn makes_copy(n: u32) {
    // n is copied into scope, calls to x afterward in parent would work as expected.
    println!("makes_copy copied n: {n}");
} // n goes out of scope, is dropped.

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();

    (s, len)
}
