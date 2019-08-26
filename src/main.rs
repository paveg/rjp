#[derive(Default)]
// define Position struct
struct Position {
    line: String,
    line_number: usize,
    line_position: usize,
}

// implement getter / setter
impl Position {
    fn line(&self) -> &String { &self.line }
    fn line_mut(&mut self) -> &mut String { &mut self.line }
    fn line_number(&self) -> &usize { &self.line_number }
    fn line_number_mut(&mut self) -> &mut usize { &mut self.line_number }
    fn line_position(&self) -> &usize { &self.line_position }
    fn line_position_mut(&mut self) -> &mut usize { &mut self.line_position }
}

fn main() {
    let mut position = Position::default();

    *position.line_mut() = String::from("abcde");
    *position.line_number_mut() = 3;
    *position.line_position_mut() = 0;
    println!("{}", position.line());
    println!("{}", position.line_number());
    println!("{}", position.line_position());
}
