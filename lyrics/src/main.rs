const LYRIC_ITERATION_ADDITIONS: [&str; 12] = 
    [
        "partridge in a pear tree",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five gold rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,"
    ];

const DAYS: [&str; 12] = 
    [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

fn main() {
    println!("The 12 Days of Christmas Lyrics");
    println!("");
    line_separator();
    line_separator();
    for day in 0..DAYS.len() {
        if day > 0 {
            println!("");
            line_separator();
            println!("");
        }
        print_preamble(day);
        print_refrain(
            day, 
            day == 0, 
            day == DAYS.len() - 1
        );
    }
}

fn line_separator() {
    println!("---------------------------------------------------------");
}

fn print_refrain(day: usize, start: bool, end: bool) {
    if day != 0 {
        for cur_day in LYRIC_ITERATION_ADDITIONS[1..=day].iter().rev() {
            println!("{}", cur_day);
        }
    }
    println!("{}{}{}",
            if start { "A "} else { "And a " },
            LYRIC_ITERATION_ADDITIONS[0],
            if end { "!" } else { "." }
    );
}

fn print_preamble(day: usize) {
    println!("On the {} day of Christmas,", DAYS[day]);
    println!("my true love sent to me")
}