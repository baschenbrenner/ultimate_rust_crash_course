use rand::random;
use rand::thread_rng;
use rand::Rng;

pub fn greet() {
    if random() {

        println!("hey you big {} year old sexy papa", thread_rng().gen_range(40,50));
    }

}
