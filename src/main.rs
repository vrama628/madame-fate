use clap::Parser;
use std::{collections::BTreeMap, iter::repeat};

#[derive(PartialEq, Eq, Clone)]
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
    Brown,
}

#[derive(Clone)]
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

    fn above(&self, below: &Self) -> bool {
        self.bottom == below.top
    }

    fn left_of(&self, right: &Self) -> bool {
        self.right == right.left
    }
}

#[derive(Clone)]
struct Grid(BTreeMap<(usize, usize), Block>);

impl Grid {
    fn new() -> Self {
        Self(BTreeMap::new())
    }

    fn all_positions() -> impl Iterator<Item = (usize, usize)> {
        (0..4).flat_map(|r| repeat(r).zip(0..4))
    }

    fn remaining_positions(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        Self::all_positions()
            .into_iter()
            .filter(|pos| !self.0.contains_key(pos))
    }

    fn above(&self, (r, c): (usize, usize)) -> Option<&Block> {
        r.checked_sub(1).and_then(|rr| self.0.get(&(rr, c)))
    }

    fn right(&self, (r, c): (usize, usize)) -> Option<&Block> {
        (c + 1 < 4).then(|| self.0.get(&(r, c + 1))).flatten()
    }

    fn below(&self, (r, c): (usize, usize)) -> Option<&Block> {
        (r + 1 < 4).then(|| self.0.get(&(r + 1, c))).flatten()
    }

    fn left(&self, (r, c): (usize, usize)) -> Option<&Block> {
        c.checked_sub(1).and_then(|cc| self.0.get(&(r, cc)))
    }

    fn try_place(&self, block: &Block, pos: (usize, usize)) -> Option<Self> {
        (!self.0.contains_key(&pos)
            && self.above(pos).map_or(true, |a| a.above(&block))
            && self.right(pos).map_or(true, |r| block.left_of(r))
            && self.below(pos).map_or(true, |b| block.above(b))
            && self.left(pos).map_or(true, |l| l.left_of(&block)))
        .then(|| {
            let mut new = self.clone();
            new.0.insert(pos, block.clone());
            new
        })
    }
}

#[derive(Parser)]
enum Puzzle {
    Three,
    Four,
}

impl Puzzle {
    fn blocks(&self) -> Vec<Block> {
        use Color::*;
        match self {
            Self::Three => vec![
                Block::new(LightGreen, Pink, Turquoise, Red),
                Block::new(Purple, DarkGreen, Pink, Purple),
                Block::new(Orange, Purple, Turquoise, Yellow),
                Block::new(Blue, Purple, Blue, Turquoise),
                Block::new(Yellow, Brown, Red, Orange),
                Block::new(Turquoise, Orange, DarkGreen, Turquoise),
                Block::new(White, Red, Pink, LightGreen),
                Block::new(DarkGreen, Turquoise, Blue, Pink),
                Block::new(Brown, Red, Brown, Pink),
                Block::new(Blue, DarkGreen, Yellow, Purple),
                Block::new(Purple, Turquoise, LightGreen, Blue),
                Block::new(Turquoise, Turquoise, Red, LightGreen),
                Block::new(Pink, LightGreen, DarkGreen, Yellow),
                Block::new(Brown, Blue, White, Orange),
                Block::new(Red, White, Purple, Turquoise),
                Block::new(Orange, Yellow, Purple, Red),
            ],
            Self::Four => vec![
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
            ],
        }
    }
}

fn main() {
    let blocks = Puzzle::parse().blocks();
    let grid = Grid::new();
    let res = search(&blocks, &grid).unwrap();
    println!("{res:?}");
}

fn search(blocks: &[Block], grid: &Grid) -> Option<Vec<(usize, usize)>> {
    let Some((block, blocks)) = blocks.split_first() else {
        return Some(vec![]);
    };
    grid.remaining_positions().find_map(|pos| {
        let grid = grid.try_place(block, pos)?;
        let mut res = search(blocks, &grid)?;
        res.insert(0, pos);
        Some(res)
    })
}
