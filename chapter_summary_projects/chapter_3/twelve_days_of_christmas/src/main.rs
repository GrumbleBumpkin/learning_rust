// There is almost certainly a better way to do this than global arrays, but at this stage in the book, this is the best
static ITEMS: &'static [&'static str] = &[
    "beer",
    "turtlenecks",
    "French toasts",
    "pounds of backbacon",
    "GOOOOOLDEN TOOOOOOOOQUES",
    "packs of two-four",
    "packs of smokes",
    "comic books",
    "doughnuts",
    "doughnuts",
    "doughnuts",
    "doughnuts",
];

static DAYS_AS_TEXT: &'static [&'static str] = &[
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

fn main() {
    for i in 0..12 {
        print_song_lines(i);
        println!();
    }
}

fn print_song_lines(n_day: usize) {
    println!(
        "On the {} day of Christmas, my true love gave to me:",
        DAYS_AS_TEXT[n_day]
    );
    for i in (0..n_day + 1).rev() {
        if i > 0 {
            println!("{} {}", i + 1, ITEMS[i]);
        } else {
            if n_day == 0 {
                println!("A {}, in a tree!", ITEMS[i]);
            } else {
                println!("And a {}, in a tree!", ITEMS[i]);
            }
        }
    }
}
