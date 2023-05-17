enum Directions{
    Up,
    Down,
    Left,
    Right
}

fn main() {
   let throw_direction:Directions = Directions::Up;

   match throw_direction{
    Directions::Up => print!("Vogle was thown up."),
    Directions::Down => print!("Jenny was smashed to the ground."),
    Directions::Left => print!("Vogel was hit from the Left."),
    Directions::Right => print!("Jenny got a right hook."),
   }
}
