fn main() {
    let s = give_ownership();

    takes_ownership(s);

    // println!("{}", s); // s dropped after takes_ownership

    let s2 = String::from("More bacon");

    println!("{}", s2);

    let s3 = takes_and_gives_back(s2);

    // println!("{}", s2); // s2 dropped in takes_and_gives_back
    println!("{}", s3);

    let (s4, len) = calculate_and_pass(s3);

    println!("The length of \"{}\" is {}", s4, len);

    let x = 5;

    makes_copy(x);

    println!("{}", x);
}

fn calculate_and_pass(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn give_ownership() -> String {
    let s = String::from("Bacon");
    s
}

fn makes_copy(some_num: i32) {
    println!("{}", some_num);
}

fn takes_and_gives_back(stringy_thingy: String) -> String {
    stringy_thingy + "!!!"
}

fn takes_ownership(stringy_thingy: String) {
    println!("{}", stringy_thingy);
}
