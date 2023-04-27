fn main() {
    let x = 123; //i32: Datatype "invisible" and not dynamic

    println!("{}",x);

    let x: &str = "test"; // Shadowing 

    println!("{}", x);
}
