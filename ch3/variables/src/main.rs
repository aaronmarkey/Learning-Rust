fn main() {
    // 3.2 Stuff

    // Int addition
    let sum = 3 + 10;

    // F64 subtraction
    let diff = 90.8 - 0.8;

    // F64 division
    let quot = 56.7 / 2.5;

    // Trunicated Int division
    let trun = -5 / 3; // -1

    // Remainder
    let rem = 43 % 5; // 3

    // Character type, supports unicode.
    let c = "z";
    let z: char = 'ℤ'; // Explicit typing of char requires single quotes.
    let emoji = "⏰";

    // Tuple
    let tup: (u32, f64) = (400, 40.0);
    let (x, y) = tup; // Tuple spreading.
    tup.0; // Dot access for tuples.

    // Arrays
    let arr = [1, 2, 3]; // Arrays are single typed, fixed length.
    let arr2: [i32; 3] = [4, 5, 6]; // Explicit typing.
    let arr3 = [3; 5]; // [3, 3, 3, 3, 3]
    arr[0]; //Bracket access for arrays.
}
