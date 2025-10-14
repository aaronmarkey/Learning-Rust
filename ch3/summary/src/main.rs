// Functions described in Rust Book
// https://doc.rust-lang.org/book/ch03-05-control-flow.html#summary
fn main() {
    // Temp conversions.
    let cs: [f64; 3] = [0.0, -40.0, 40.0];
    let fs: [f64; 3] = [32.0, -40.0, 104.0];

    for c in cs {
        let f = c_to_f(c);
        println!("{c}C -> {f}F");
    }
    for f in fs {
        let c = f_to_c(f);
        println!("{f}F -> {c}C");
    }
    println!("\n");

    // Fibonacci
    for nth in 0..13 {
        let fvalue = nth_fib(nth);
        let nthvalue = nth + 1;
        println!("Fibonacci({nthvalue}th) -> {fvalue}");
    }
    println!("\n");

    // Twelve Days of Xmas
    twelve_days_of_xmas_lyrics();
}

fn f_to_c(f: f64) -> f64 {
    const F_FREEZE: f64 = 32.0;
    const CONVERSION_RATE: f64 = 9.0 / 5.0;
    (f - F_FREEZE) / CONVERSION_RATE
}

fn c_to_f(c: f64) -> f64 {
    const F_FREEZE: f64 = 32.0;
    const CONVERSION_RATE: f64 = 9.0 / 5.0;
    return c * CONVERSION_RATE + F_FREEZE;
}

fn nth_fib(n: u32) -> u32 {
    let mut counter: u32 = 0;

    let mut current: u32 = 1;
    let mut past: u32 = 0;

    while counter < n {
        let temp = current;
        current += past;
        past = temp;
        counter += 1;
    }
    past
}

fn twelve_days_of_xmas_lyrics() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelve",
    ];
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let mut counter: u32 = 0;
    for day in days {
        println!("On the {day} day of Christmas, my true love sent to me");

        for x in 0..(counter + 1) {
            let lyric = gifts[x as usize];
            println!("{lyric}");
        }

        counter += 1;
        println!();
    }
}
