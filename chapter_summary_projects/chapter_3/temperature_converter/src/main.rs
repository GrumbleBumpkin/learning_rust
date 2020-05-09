use std::io;

fn main() {
    loop {
        println!("Give a number or 'q' to quit.");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read line.");

        if input.trim() == "q" {
            break;
        }

        let value: f32 = input.trim().parse().expect("Value was not a number.");

        let fahrenheit_value = value * (9.0 / 5.0) + 32.0;

        let celsius_value = (value - 32.0) * (5.0 / 9.0);

        println!(
            "{}C == {}F",
            value,
            fahrenheit_value
        );
        println!(
            "{}F == {}C",
            value,
            celsius_value
        );
        print!("\n");
    }

    println!("\nGoodbye.")
}
