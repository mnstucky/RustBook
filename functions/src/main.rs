fn main() {
    println!("Hello, world!");
    let result = another_function(5, "items");
    println!("{:?}", result);
}

fn another_function(n: u32, label: &str) -> u32 {
    println!("Another function. The value is {n} {label}.");
    let x = {
        let z = 2;
        z + 1
    };
    n + x
}
