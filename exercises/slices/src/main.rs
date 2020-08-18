fn main() {
    let s = String::from("Boom crash smash!");
    let word = first_word(&s); // No need to pass full slice due to deref https://doc.rust-lang.org/std/string/struct.String.html#deref
    // s.clear();
    println!("{}", word);

    let s2 = String::from("Bacon");
    let word2 = first_word(&s2); // No need to pass full slice due to deref https://doc.rust-lang.org/std/string/struct.String.html#deref
    // s2.clear();
    println!("{}", word2);

    let a = [1, 2, 3, 4, 5]; 
    let a_slice = &a[1..3];

    for &item in a_slice.iter() {
        println!("{}", &item);
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
