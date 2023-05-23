fn main() {
    let mut x = 10;

    {
        let x = 15;
    }
    println!("{}", x);

    let x = "X is a string";
    println!("{}", x);

    let x = false;
    println!("{}", x);
}
