
use rand::prelude::*;


trait Bite{
    fn bite(self: &mut Self){

    }

}

fn main() {
    // Once you finish #1 above, this part should work.
    let mut carrot = Carrot { percent_left: 100.0 };
    carrot.bite();
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    // 4. Uncomment and adjust the code below to match how you defined your
    // Grapes struct.
    //
    let mut grapes = Grapes { grapes_left: 9 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);


    // Challenge: Uncomment the code below. Create a generic `bunny_nibbles`
    // function that:
    // - takes a mutable reference to any type that implements Bite
    // - calls `.bite()` several times
    // Hint: Define the generic type between the function name and open paren:
    //       fn function_name<T: Bite>(...)
    //
    bunny_nibbles(&mut carrot);
    println!("Bunny nibbles for awhile: {:?}", carrot);
    let mut grapes2 = Grapes {grapes_left: 25};
    bunny_nibbles(&mut grapes2);
    println!("Bunny nibbles for awhile: {:?}", grapes2);
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}

#[derive(Debug)]
struct Grapes {
    grapes_left: u32,
}

impl Bite for Grapes {
    fn bite(self: &mut Self) {
        if self.grapes_left >= 4 {
            self.grapes_left -= 4
        } else if self.grapes_left > 0{
            self.grapes_left = 0
        }

    }
}


fn bunny_nibbles<T: Bite>(item: &mut T) {
    let num = rand::thread_rng().gen_range(3,7);
    let mut count = 0;
    while count < num {
        item.bite();
        count +=1;
        println!("took a bite!")
    }


}
