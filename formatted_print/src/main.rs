fn main() {
    // print! : text is printed in console (io::stdout)
    println!("---------- print! ----------");
    print!("Statement 1");
    print!("Statement 2");
    print!("\n");

    //println! : same as print! but a newline is appended
    println!("---------- println! ----------");
    println!("Statement 1");
    println!("Statement 2");

    // Passing argument in print
    println!("{} days", 31);

    /*
    Positional argument: specifying an integer inside '{}' determines
    which additional argument will be replaced.
    Arguments starts at 0, immediately after format string
    */
    println!(
        "{0} and {1} are siblings, but {1} is elder than {0}.",
        "Alice", "Bob"
    );

    // Rust checks to make sure corrent number of argument is used, uncomment to see the result
    // println!("{0} and {1} neighbour", "Alice");

    // name argument:
    println!(
        "{name} of age {age} lives in city {city}",
        name = "Alice",
        city = "Bengaluru",
        age = 32
    );

    // Different formatting can be invoked by specifying the format character after a ':'
    println!("Base 10:              {}", 69420); // add argument as it is
    println!("Base 2 (binary):      {:b}", 69420); // :b -> converts to binary
    println!("Base 8 (octal):       {:o}", 69420); // :o -> converts to octal
    println!("Base 16 (hexadecimal) {:x}", 69420); // :x -> converts to hexadecimal

    /*
    text-justify:
    :> right-justify
    */
    println!("{number:>10}", number = 4);

    /*
    Pad number:
    left-padding: ':' padding_number '>' repetetion_count (excluding end)
    example: 7:0>5 --> 00007
    right-padding: ':' padding_number '<' repetetion_count (excluding end)
    example: 7:0<5 --> 70000
     */
    println!("{number:1>10}", number = 7);
    println!("{number:0<5}", number = 1);
    // named argument in the format specifier by appending '$'
    println!("{number:0>width$}", number = 1, width = 10);
}
