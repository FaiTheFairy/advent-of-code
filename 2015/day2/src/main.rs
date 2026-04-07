use anyhow::{Context, Result, ensure};
use std::{fs, str::FromStr};

fn main() -> Result<()> {
    let boxes: Boxes = fs::read_to_string("input.txt")?.parse()?;

    let sol1 = boxes.solve_part_1();
    println!("Part 1: {sol1}");

    let sol2 = boxes.solve_part_2();
    println!("Part 2: {sol2}");

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Boxes(Vec<Box>);

impl Boxes {
    fn solve_part_1(&self) -> usize {
        self.iter().map(Box::wrapping_paper_sqft).sum()
    }

    fn solve_part_2(&self) -> usize {
        self.iter().map(Box::ribbon_ft).sum()
    }

    fn iter(&self) -> impl Iterator<Item = Box> {
        self.0.iter().copied()
    }
}

impl FromStr for Boxes {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let boxes = s
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        Ok(Self(boxes))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Box {
    l: usize,
    w: usize,
    h: usize,
}

impl Box {
    fn wrapping_paper_sqft(self) -> usize {
        self.surface_area() + self.smallest_side()
    }

    fn ribbon_ft(self) -> usize {
        self.volume() + self.smallest_perimeter()
    }

    fn surface_area(self) -> usize {
        let Self { l, w, h } = self;
        (2 * l * w) + (2 * w * h) + (2 * h * l)
    }

    fn volume(self) -> usize {
        self.l * self.w * self.h
    }

    fn smallest_side(self) -> usize {
        let Self { l, w, h } = self;

        (l * w).min(w * h).min(h * l)
    }

    fn smallest_perimeter(self) -> usize {
        let Self { l, w, h } = self;
        (2 * l + 2 * w).min(2 * w + 2 * h).min(2 * l + 2 * h)
    }
}

impl FromStr for Box {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.trim().split('x');
        let length = iter.next().context("missing length")?.parse()?;
        let width = iter.next().context("missing width")?.parse()?;
        let height = iter.next().context("missing height")?.parse()?;

        ensure!(iter.next().is_none(), "box has more than 3 dimensions");

        Ok(Self {
            l: length,
            w: width,
            h: height,
        })
    }
}
