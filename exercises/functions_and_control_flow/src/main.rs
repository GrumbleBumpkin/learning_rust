fn main() {
    hi_and_bye("World", "Felicia");
    statement_test();
    expression_test();

    let z = return_test() * 2;
    println!("The value of z is {}", z);
    println!("The value of return_test is {}", return_test());

    println!("The value of if_test is {}", if_test(10));
    println!("The value of if_test is {}", if_test(4));
    // println!("The value of if_test is {}", if_test(666));

    loop_test(3);
}

fn hi_and_bye(hi_to: &str, bye_to: &str) {
    println!("Hello, {}", hi_to);
    println!("Bye, {}", bye_to);
}

fn statement_test() {
    // let x = let y = 5;
}

fn expression_test() {
    let y = {
        let x = 2;
        x * 2
    };

    println!("Value of y: {}", y);
}

fn return_test() -> i32 {
    3
}

fn if_test(x: i32) -> i32 {
    let result = if x == 666 {
        panic!()
    } else if x > 5 {
        x - 4
    } else {
        x * 2
    };

    return result;
}

fn loop_test(iterations: i32) {
    let mut count = 0;

    loop {
        count += 1;
        println!("loop {}", count);
        if count == iterations {
            break;
        }
    }

    while count >= 0 {
        println!("while countdown: {}", count);
        count -= 1;
    }

    for i in count..iterations {
        println!("for {}", i);
    }
}
