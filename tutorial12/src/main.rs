fn main() {
    let tup = (12, "abc", (1,2,3));

    println!("{}",tup.1);
    println!("{}",(tup.2).1);

    let (a, b, c) = tup;

    println!("{}",a);
    println!("{}",b);
    println!("{}",c.1);
}
