fn main() {
    // Part One
    let p1 = String::from("Part1 One");
    let part = &p1[0..4]; // Slices are references, slices are made with a range.
    let part = &p1[..4]; // Same thing.
    let one = &p1[5..8];
    let one = &p1[5..]; // Same thing.
    let p1all = &p1[..]; // Same as p1.

    // Part Two
    let mut p2 = String::from("Part2 Two");
    let first_word = first_word(&p2);
    println!("First word: {first_word}");
    // Allowed, as long as first_word is not used further.
    // first_word is an immutable reference to p2, a mutable variable.
    p2.clear();
    // But this is not, because we've referenced first_word, which is borrowing
    // from p2, which has been cleared (clear uses a mutable reference to it's caller.)
    // println!("first word is {first_word}");

    // Part Three
    let p3_str = String::from("Part3 Three");
    let word = better_first_word(&p3_str);
    let word = better_first_word(&p3_str[0..5]);
    let word = better_first_word(&p3_str[..]);
    let p3_literal = "Part3L Three";
    let word = better_first_word(&p3_literal[0..8]);
    let word = better_first_word(&p3_literal[..]);
    let word = better_first_word(p3_literal);

    // Part 4
    let a = [1, 2, 3, 4];
    let aslice = &a[0..2]; // A slice of an [i32]. Still a reference.

    let t = (1, 2, 3);
    // let tslice = &t[0..2]; // Tuples cannot be sliced.
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes(); // Convert to bytes.

    for (i, &item) in bytes.iter().enumerate() {
        // Create iterator, and enumerate over.
        if item == b' ' {
            return &s[..i]; // If space is found, return string slice.
        }
    }
    &s[..] // Else, return slice of whole string.
} // s is dropped.

fn better_first_word(s: &str) -> &str {
    // Same as first_word(), but accept a string slice and a &String.
    // This takes advantage of deref coercisions, more in Ch15.
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
