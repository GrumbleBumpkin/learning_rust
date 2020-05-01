fn main() {
    // String parsing
    let parsed_string: u32 = "42".parse().expect("Not a number!");
    println!("Value of parsed string: {}", parsed_string);

    // Test .expect statement
    // let parsed_string: u32 = "Bacon".parse().expect("Not a number!");
    // println!("Value of parsed string: {}", parsed_string);

    // Literals
    let _dec = 100_000_000;
    let _bad_dec = 10_00000_00; // Testing visual separators
    // let _overflow_dec: u8 = 1000;
    // let _unsigned_test: u8 = -1;
    let _hex = 0xFFFF;
    let _oct = 0o7777;
    let _bin = 0b1111_0000;
    let _bad_bin = 0b11_110000; // Testing visual separators
    let _byte = b'X';
    // let _bad_byte: u32 = b'X'; // Testing datatype hints

    let _default_float = 2.5;
    let _32_float: f32 = 2.5;

    // Operations
    let a = (5 + 5) / 3;
    let _b: u8 = a;
    println!("Value of _b: {}", _b);
    let _c: u32 = 32;
    // let _types_test: u32 = _b / _c;

    // Compound types

    let tup: (u8, f32, usize) = (5, 5.0, 5);
    let (x, y, _z) = tup;
    println!("({}, {}, {})", x, y, tup.2);

    let ary = ["Bacon", "Sandwich", "Om", "Nom", "Nom"];
    println!("{}", ary[2]);

    let mut ary2 = [2; 6];
    ary2[1] = 5;
    println!("[{}, {}]", ary2[0], ary2[1]);

    // Test array out of bounds
    // let _bad_index = ary2[9];
}