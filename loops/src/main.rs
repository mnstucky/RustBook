fn main() {
    let mut counter = 0;
    let x = loop {
        counter = counter + 1;
        if counter > 5 {
            break counter;
        }
    };
    println!("{x}");

    let array = [1; 5];
    for element in array {
        println!("{element}");
    }

    // exclusive
    for index in 1..5 {
        println!("{index}");
    }

    // inclusive
    for index in 1..=5 {
        println!("{index}");
    }

    // inclusive reversed
    for index in (1..=5).rev() {
        println!("{index}");
    }
}
