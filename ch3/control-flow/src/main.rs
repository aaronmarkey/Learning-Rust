fn main() {
    loops()
}

fn loops() {
    let mut con = true;
    loop {
        if con {
            con = false;
            continue;
        } else {
            break;
        }
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // Assign result to counter times 2.
        }
    };
    println!("result: {result}");

    // Loops can be labeled.
    'top_loop: loop {
        println!("in top_loop");

        // Breaks can then specify the loop to break.
        loop {
            println!("in inner loop and breaking top_loop by label.");
            break 'top_loop;
        }
    }

    // While
    let mut x = 0;
    while x < 5 {
        println!("while: {x}");
        x += 1;
    }

    // For - Iterate over a collection.
    let arr = [1, 2, 3];
    for el in arr {
        println!("for: {el}");
    }

    // For with a range
    for el in (1..4).rev() {
        // Create a range and reverse it.
        println!("for with range: {el}");
    }
}
