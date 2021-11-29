// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_mut, unused_variables)]

fn main() {
    // This collects any command-line arguments into a vector of Strings.
    // For example:
    //
    //     cargo run apple banana
    //
    // ...produces the equivalent of
    //
    //     vec!["apple".to_string(), "banana".to_string()]
    let args: Vec<String> = std::env::args().skip(1).collect();

    // This consumes the `args` vector to iterate through each String
    for what in args {
        if what == "sum" {
            sum()
        } else if what == "double" {
            double()
        } else {
            count(what)
        }
        // 1a. Your task: handle the command-line arguments!
        //
        // - If arg is "sum", then call the sum() function
        // - If arg is "double", then call the double() function
        // - If arg is anything else, then call the count() function, passing "arg" to it.


        // 1b. Now try passing "sum", "double" and "bananas" to the program by adding your argument
        // after "cargo run".  For example "cargo run sum"
    }
}

fn sum() {
    let mut sum = 0;
    for num in 7..=23 {
        sum = sum + num
    }


    println!("The sum is {}", sum);
}

fn double() {
    let mut count = 0;
    let mut x = 3;

    while x < 500 {
        x *= 2;
        count += 1;
    }


    println!("You can double x {} times until x is larger than 500", count);
}

fn count(arg: String) {
    loop {
        let mut count=0;
        while count<8 {
            println!("{}",arg);
            count +=1;
        }

        
        break

    }
    // Challenge: Use an unconditional loop (`loop`) to print `arg` 8 times, and then break.
    // You will need to count your loops, somehow.  Run it with `cargo run bananas`
    //
    // print!("{} ", arg); // Execute this line 8 times, and then break. `print!` doesn't add a newline.


    println!(); // This will output just a newline at the end for cleanliness.
}
