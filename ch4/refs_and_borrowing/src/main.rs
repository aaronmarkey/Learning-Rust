fn main() {
    // 1
    let s = String::from("Part 1");
    let len = calculate_length(&s); // Passed by reference, aka, borrowing.
    println!("s({s}) is {len} long."); // s is still avaialble.

    // 2
    let mut s2 = String::from("Part 2");
    change(&mut s2);
    println!("{s2}");

    // 3
    let mut s = String::from("Part 3");
    let r1 = &mut s;
    let r2 = &mut s;
    let r3 = &mut s;
    // println!("{r1}"); // Causes comp error because mutiple mut refs to s are avaiable.
    calculate_length(r3); // Note that r1 and r2 are both unused.
    // calculate_length(r2); // Fails for the same reason.

    // 4
    let mut s = String::from("Part 4");
    {
        let r1 = &mut s;
        calculate_length(r1);
    } // r1 is dropped at end of scope.
    let r2 = &mut s; // r2 can exist because r1 is already dropped. Only one mut reference in scope.
    calculate_length(r2);

    // 5
    let mut s = String::from("Part 5");
    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s; // This fails because an immut ref to s is used _after_ it.
    println!("{r1} {r2}");
    println!("{r1} {r2}");

    let r3 = &mut s;
    change(r3);
    println!("{r3}");
}

fn calculate_length(s: &String) -> usize {
    // This function borrows s.
    // &String is reference to an immutable String.
    s.len()
} // s is dropped, but parent still has access, b/c s is a reference.

fn change(s: &mut String) {
    // s is a mutable reference, a mutable String.
    s.push_str(" (this was added by change().)");
}
