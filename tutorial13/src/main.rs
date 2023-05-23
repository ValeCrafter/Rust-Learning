fn main() {
    print_loop(10);

    println!("Is 30 even? {}", is_even(30));
}

fn print_loop(num: u32){
    for n in 1..num+1 {
        println!("{}",n);
    }
}

fn is_even(num: u32) -> bool{
    if num % 2 == 0{
        return true;
    }else{
        return false;
    }
}