// Method for finding fibonacci number by quick doubling found at https://www.hackerearth.com/practice/notes/fast-doubling-method-to-find-nth-fibonacci-number/

use std::io;

fn main() {
    const MOD: u32 = 1_000_000_007;
    loop {
        println!("Give a number or 'q' to quit.");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read line.");

        if input.trim() == "q" {
            break;
        }

        let value: u32 = input.trim().parse().expect("Value was not a number.");

        if value < 2 {
            println!("{}", value);
        } else {
            let mut iteration: u32 = 0;
            let mut answer: u32 = 0;
            let mut n1: u32 = 0;
            let mut n2: u32 = 1;

            while iteration < value {
                answer = (n1 + n2) % MOD;
                n2 = n1;
                n1 = answer;
                iteration += 1;
            }
            println!("{}", answer);
        }
    }

    println!("\nGoodbye.")
}
