
use rand::seq::SliceRandom;

fn main() {
    let names: [&str; 10] = ["A---", "A--", "B----", "D-", "D---", "F------", "J--", "L-----", "M---", "N---"];
    let name = match names.choose(&mut rand::thread_rng()) {
        Some(n) => {n}
        None => {"oops"}
    };
    println!("{}", name)
}
