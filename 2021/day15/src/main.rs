#![allow(unused)]

use core::fmt;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fs,
    iter::Sum,
    ops::Add,
    str::FromStr,
};

use anyhow::{Result, anyhow, bail, ensure};
use grid::*;
use owo_colors::OwoColorize;

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let sol1 = solve_part_1(&input)?;
    println!("Part 1. cost of best path = {sol1}");

    let sol2 = solve_part_2(&input)?;
    println!("Part 2. cost of best path when expanded to 5x5 = {sol2}");

    Ok(())
}

fn solve_part_1(input: &str) -> Result<usize> {
    let grid = input.parse::<RiskGrid>()?;
    let (cost, path) = grid.solve();
    std::fs::write("sol1.txt", format!("{}", grid.display_with_path(&path)));
    Ok(cost)
}

fn solve_part_2(input: &str) -> Result<usize> {
    let grid = input.parse::<RiskGrid>()?.expanded_to_5x5();
    let (cost, path) = grid.solve();
    std::fs::write("sol2.txt", format!("{}", grid.display_with_path(&path)));
    Ok(cost)
}

type Coord = (usize, usize);

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct RiskGrid(Grid<RiskLevel>);

struct PathView<'a> {
    grid: &'a RiskGrid,
    path: HashSet<Coord>,
}

impl std::fmt::Display for PathView<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.grid.0.rows() {
            for col in 0..self.grid.0.cols() {
                let risk = self.grid.0[(row, col)].value();

                if self.path.contains(&(row, col)) {
                    write!(f, "{}", risk.bold().red())?;
                } else {
                    write!(f, "{}", risk)?;
                }
            }

            if row + 1 != self.grid.0.rows() {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl RiskGrid {
    fn display_with_path<'a>(&'a self, path: &[Coord]) -> PathView<'a> {
        PathView {
            grid: self,
            path: path.iter().copied().collect(),
        }
    }

    fn solve(&self) -> (usize, Vec<Coord>) {
        let height = self.0.rows();
        let width = self.0.cols();
        let goal = (height - 1, width - 1);

        // `dist[r][c]` represents the best known cost to reach (r, c)
        let mut dist = vec![vec![usize::MAX; width]; height];
        dist[0][0] = 0;

        // `prev[r][c] = Some((pr, pc))` means
        // the best path to (r, c) came from (pr, pc)
        let mut prev = vec![vec![None; width]; height];

        // A `BinaryHeap` is a priority queue.
        // It always removes ("pops") the element with the largest priority.
        let mut heap = BinaryHeap::new();

        // Since Rust's BinaryHeap is a max-heap by default,
        // we reverse the ordering so that smaller costs come out first.
        heap.push(Reverse((0usize, 0usize, 0usize))); // (cost, row, col)

        while let Some(Reverse((cost, row, col))) = heap.pop() {
            // we are done once we reach the goal
            if (row, col) == goal {
                return (cost, reconstruct_path(&prev, goal));
            }

            // skip path if it's not the best known path anymore
            if cost > dist[row][col] {
                continue;
            }

            // explore neighbors
            for (nr, nc) in self.neighbors(row, col) {
                // `neighbors` gurantees that the cell exists in the grid.
                let risk = self.0[(nr, nc)].value();
                let next_cost = cost + risk;

                // update if we find a better path to [nr, nc]
                if next_cost < dist[nr][nc] {
                    dist[nr][nc] = next_cost;

                    // record where we came from
                    prev[nr][nc] = Some((row, col));

                    heap.push(Reverse((next_cost, nr, nc)));
                }
            }
        }

        unreachable!("goal should always be reachable")
    }

    fn neighbors(&self, row: usize, col: usize) -> impl Iterator<Item = Coord> {
        const DIRS: [(isize, isize); 4] = [
            (1, 0),  // down
            (-1, 0), // up
            (0, 1),  // right
            (0, -1), // left
        ];
        DIRS.into_iter().filter_map(move |(dr, dc)| {
            let nr = row as isize + dr;
            let nc = col as isize + dc;

            if nr < 0 || nc < 0 {
                return None;
            }

            let nr = nr as usize;
            let nc = nc as usize;

            self.0.get(nr, nc).map(|_| (nr, nc))
        })
    }

    fn increment_all(&mut self) {
        for level in self.0.iter_mut() {
            level.increment();
        }
    }

    fn tiled(&self, n: usize) -> Self {
        let base_rows = self.0.rows();
        let base_cols = self.0.cols();

        // this is a nxn grid of RiskGrids
        let mut tiles = Grid::init(n, n, self.clone());

        for tile_row in 0..n {
            for tile_col in 0..n {
                let mut grid = self.clone();
                for _ in 0..(tile_row + tile_col) {
                    grid.increment_all();
                }
                tiles[(tile_row, tile_col)] = grid;
            }
        }

        let mut out = Grid::new(0, base_cols * n);

        for tile_row in 0..n {
            for inner_row in 0..base_rows {
                let mut merged_row = Vec::with_capacity(base_cols * n);

                for tile in tiles.iter_row(tile_row) {
                    for inner_col in 0..base_cols {
                        merged_row.push(tile.0[(inner_row, inner_col)]);
                    }
                }

                out.push_row(merged_row);
            }
        }

        RiskGrid(out)
    }

    fn expanded_to_5x5(&self) -> Self {
        self.tiled(5)
    }
}

fn reconstruct_path(prev: &[Vec<Option<Coord>>], mut current: Coord) -> Vec<Coord> {
    let mut path = Vec::new();

    while let Some(parent) = prev[current.0][current.1] {
        path.push(current);
        current = parent;
    }

    path.push(current);
    path.reverse();
    path
}

impl FromStr for RiskGrid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        ensure!(!lines.is_empty(), "grid cannot be empty");
        let cols = lines[0].len();
        ensure!(
            lines.iter().all(|line| line.len() == cols),
            "grid must be rectangular"
        );

        let data = lines
            .iter()
            .flat_map(|line| line.bytes())
            .map(|b| RiskLevel::try_from(b - b'0'))
            .collect::<Result<Vec<_>>>()?;

        let grid = Grid::from_vec(data, cols);

        Ok(RiskGrid(grid))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct RiskLevel(usize);

impl RiskLevel {
    fn value(&self) -> usize {
        self.0
    }

    /// Rotating incrementation by 1, risk levels at 9 increment to 1.
    fn increment(&mut self) {
        if self.0 == 9 { self.0 = 1 } else { self.0 += 1 }
    }
}

impl TryFrom<u8> for RiskLevel {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        ensure!((1..=9).contains(&value), "risk level must be in 1..=9");
        Ok(Self(value as usize))
    }
}

impl FromStr for RiskLevel {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        s.parse::<u8>()?.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    const EXAMPLE_5X5: &str = "11637517422274862853338597396444961841755517295286
13813736722492484783351359589446246169155735727126
21365113283247622439435873354154698446526571955763
36949315694715142671582625378269373648937148475914
74634171118574528222968563933317967414442817852555
13191281372421239248353234135946434524615754563572
13599124212461123532357223464346833457545794456865
31254216394236532741534764385264587549637569865174
12931385212314249632342535174345364628545647573965
23119445813422155692453326671356443778246755488935
22748628533385973964449618417555172952866628316397
24924847833513595894462461691557357271266846838237
32476224394358733541546984465265719557637682166874
47151426715826253782693736489371484759148259586125
85745282229685639333179674144428178525553928963666
24212392483532341359464345246157545635726865674683
24611235323572234643468334575457944568656815567976
42365327415347643852645875496375698651748671976285
23142496323425351743453646285456475739656758684176
34221556924533266713564437782467554889357866599146
33859739644496184175551729528666283163977739427418
35135958944624616915573572712668468382377957949348
43587335415469844652657195576376821668748793277985
58262537826937364893714847591482595861259361697236
96856393331796741444281785255539289636664139174777
35323413594643452461575456357268656746837976785794
35722346434683345754579445686568155679767926678187
53476438526458754963756986517486719762859782187396
34253517434536462854564757396567586841767869795287
45332667135644377824675548893578665991468977611257
44961841755517295286662831639777394274188841538529
46246169155735727126684683823779579493488168151459
54698446526571955763768216687487932779859814388196
69373648937148475914825958612593616972361472718347
17967414442817852555392896366641391747775241285888
46434524615754563572686567468379767857948187896815
46833457545794456865681556797679266781878137789298
64587549637569865174867197628597821873961893298417
45364628545647573965675868417678697952878971816398
56443778246755488935786659914689776112579188722368
55172952866628316397773942741888415385299952649631
57357271266846838237795794934881681514599279262561
65719557637682166874879327798598143881961925499217
71484759148259586125936169723614727183472583829458
28178525553928963666413917477752412858886352396999
57545635726865674683797678579481878968159298917926
57944568656815567976792667818781377892989248891319
75698651748671976285978218739618932984172914319528
56475739656758684176786979528789718163989182927419
67554889357866599146897761125791887223681299833479";

    #[test]
    fn test_expand_to_5x5() {
        let result = EXAMPLE.parse::<RiskGrid>().unwrap().expanded_to_5x5();
        let expected = EXAMPLE_5X5.parse::<RiskGrid>().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_part_1() {
        let grid = EXAMPLE.parse::<RiskGrid>().unwrap();
        let cost = grid.solve().0;
        assert_eq!(cost, 40);
    }

    #[test]
    fn test_increment_grid() {
        let mut result = "129\n131\n111".parse::<RiskGrid>().unwrap();
        result.increment_all();
        let expected = RiskGrid(grid![
            [RiskLevel(2), RiskLevel(3), RiskLevel(1)]
            [RiskLevel(2), RiskLevel(4), RiskLevel(2)]
            [RiskLevel(2), RiskLevel(2), RiskLevel(2)]
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_risk_grid() {
        let result = "123\n231\n111".parse::<RiskGrid>().unwrap();
        let expected = RiskGrid(grid![
            [RiskLevel(1), RiskLevel(2), RiskLevel(3)]
            [RiskLevel(2), RiskLevel(3), RiskLevel(1)]
            [RiskLevel(1), RiskLevel(1), RiskLevel(1)]
        ]);

        assert_eq!(result, expected);
    }
}
