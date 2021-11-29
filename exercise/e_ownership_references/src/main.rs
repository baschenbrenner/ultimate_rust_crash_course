// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut)]

fn main() {
    // This fancy stuff either gets the first argument as a String, or prints
    // usage and exits if an argument was not supplied to the program.
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });
    println!("{}", arg);
    // 1. Write a function `inspect` that takes a reference to a String, returns nothing, but
    // prints whether the contents of the String is plural or singular. Then uncomment and run this
    // code with `cargo run apple` and `cargo run apples'.  Hint: use `.ends_with("s")` on the
    // String reference
    //
    inspect(&arg);

    // 2. Write a function `change` that takes a *mutable* reference to a String and adds an "s" to
    // the String if it doesn't already end with "s". Then uncomment and run the code below with
    // `cargo run apple`.  Hint: use `.push_str("s")` on the mutable String reference to add an "s".
    //
    change(&mut arg);
    println!("I have many {}", arg);

    // 3. Write a function `eat` that accepts ownership of (consumes) a String and returns a bool
    // indicating whether or not the String both starts with a "b" AND contains an "a".
    // Hint 1: use `.starts_with("b")` and `.contains("a")`
    // Hint 2: `&&` is the boolean "AND" operator
    //
    if eat(arg) {
       println!("Might be bananas");
    } else {
       println!("Not bananas");
    }

    // Try running this program with "boat", "banana", and "grapes" as the arguments :-)

    // Challenge: Write a function "add" that takes *references* to two integer arguments,
    // dereferences them and adds them together, and returns the result.
    //
    let x=1;
    let y=2;
println!("1 + 2 = {}, even via references", add(x, y));
println!("x is {} and y is {}", x,y);

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
 // special happens.




fn inspect(arg: &String) {
    if arg.ends_with("s") {
        println!("plural")
    } else {
        println!("singular")
    }
}

fn change(arg2: &mut String) -> &mut String {

    if arg2.ends_with("s") {
        return arg2;
    } else {
        arg2.push_str("s");
        return arg2;
    }

}

fn eat(str: String) -> bool {
    return str.starts_with("b") && str.contains("a");

}

fn add(int1: i32, int2: i32) -> i32 {
    int1 + int2
}
