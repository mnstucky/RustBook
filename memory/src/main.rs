fn main() {
    println!("Copying...");
    let array = [0; 20];
    println!("{:?}", array);
    let mut array_copy = array; // I have to declare this as mutable to change a value!!!!! YAY
    array_copy[0] = 1;
    println!("{:?}", array_copy);

    let x = Box::new(1);
    let y = x; // have to call clone here b/c borrowing
    println!("{x}");
}
