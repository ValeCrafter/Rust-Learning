fn main() {
    //references can not be changed, since they are not owned.
    let x = 10;
    let x_ref = &x;
    let x_ref_2 = &x;

    println!("x is {}", x);
    println!("x as x_ref is {}", x_ref);
    println!("x as x_ref_2 is {}", x_ref_2);

    //borrowed variables can be changed, but only one can borrow it at a time.

    let mut y = 5;
    
    let y_bor = &mut y;
    *y_bor += 1;
    println!("y_bor is {}", y_bor); 

    println!("y is {}",y);

    
}
