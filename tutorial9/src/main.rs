fn main() {
    for i in 1..11 {
        println!("i is {}", i);
    }

    let numbers = 30..51;

    for i in numbers {
        println!("The number is {}", i);
    }

    let animals = vec!["Vogel", "Hurensohn", "Jenny"];

    for a in animals.iter() {
        println!("{} is an animal.", a);
    }

    for (index, a) in animals.iter().enumerate(){
        println!("{} in the list is {}",index, a);
    }
}
