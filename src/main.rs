#[derive(PartialEq, Eq)]
enum Color {
    Red,
    Orange,
    Purple,
    Yellow,
    Turquoise,
    Pink,
    Blue,
    White,
    DarkGreen,
    LightGreen,
}

struct Block {
    top: Color,
    right: Color,
    bottom: Color,
    left: Color,
}

impl Block {
    fn new(top: Color, right: Color, bottom: Color, left: Color) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }
}

fn main() {
    use Color::*;
    let blocks = vec![
        Block::new(Red, Red, Orange, Purple),
        Block::new(Red, DarkGreen, Yellow, Orange),
        Block::new(Orange, Turquoise, Pink, Red),
        Block::new(Yellow, Blue, Red, Orange),
        Block::new(Pink, Pink, LightGreen, Yellow),
        Block::new(Red, Red, Turquoise, Purple),
        Block::new(LightGreen, LightGreen, Turquoise, Red),
        Block::new(Turquoise, Orange, Blue, Orange),
        Block::new(Blue, Purple, White, Yellow),
        Block::new(Purple, Orange, Yellow, Red),
        Block::new(White, Red, Orange, Turquoise),
        Block::new(Yellow, Orange, DarkGreen, Turquoise),
        Block::new(Orange, Yellow, Yellow, Pink),
        Block::new(DarkGreen, Purple, Purple, Pink),
        Block::new(Yellow, Red, Orange, White),
        Block::new(Purple, Orange, Yellow, LightGreen),
    ];
}
