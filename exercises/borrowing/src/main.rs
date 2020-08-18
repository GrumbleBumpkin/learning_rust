fn main() {
    let mut s = String::from("thingy!");

    let len = calculate_length(&s);
    println!("The length of \"{}\" is {}.", s, len);

    change(&mut s);
    println!("{}", s);

    // Can't borrow more than once in scope.
    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);

    let r1 = &mut s;
    println!("{}", r1);

    let r2 = &mut s; // Can borrow more than once so long as the first borrow isn't referenced later.
    println!("{}", r2);

    // Can't mix mutable and immutable borrows.
    // let r4 = &s;
    // let r5 = &s;
    // let r3 = &mut s;

    // println!("{}, {}, {}", r3, r4, r5);

    // let dangling_reference = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push('a');
}

// Can't create a dangling reference
// fn dangle() -> &String {
//     let s = String::from("Hello World!");
//     &s // Should return value with ownership as well.
// }