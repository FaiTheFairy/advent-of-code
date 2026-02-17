use std::str::Split;

#[derive(Debug, Clone)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

#[derive(Debug, Clone)]
struct Map {
    rules: Vec<Rule>,
}

#[derive(Debug, Clone, Copy)]
struct Rule {
    dst_start: u64,
    src_start: u64,
    len: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    start: u64,
    end: u64, // exclusive
}

impl Almanac {
    fn parse_part1(input: &str) -> Self {
        let mut blocks = input.split("\n\n");
        let seeds = parse_seeds(&mut blocks);
        let maps: Vec<Map> = blocks.map(Map::parse).collect();
        Self { seeds, maps }
    }

    fn parse_part2(input: &str) -> (Vec<Range>, Vec<Map>) {
        let mut blocks = input.split("\n\n");

        let seed_nums = parse_seeds(&mut blocks);
        assert!(seed_nums.len().is_multiple_of(2));

        let seeds: Vec<Range> = seed_nums
            .chunks_exact(2)
            .map(|p| Range {
                start: p[0],
                end: p[0] + p[1],
            })
            .collect();

        let maps: Vec<Map> = blocks.map(Map::parse).collect();
        (seeds, maps)
    }

    fn seed_to_location(&self, seed: u64) -> u64 {
        self.maps.iter().fold(seed, |x, map| map.apply(x))
    }

    fn lowest_location_part1(&self) -> u64 {
        self.seeds
            .iter()
            .copied()
            .map(|s| self.seed_to_location(s))
            .min()
            .unwrap()
    }
}

fn parse_seeds(blocks: &mut Split<'_, &str>) -> Vec<u64> {
    blocks
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

impl Map {
    fn parse(block: &str) -> Self {
        let mut lines = block.lines();
        let _header = lines.next().unwrap();

        let mut rules: Vec<Rule> = lines
            .filter(|l| !l.trim().is_empty())
            .map(Rule::parse)
            .collect();

        rules.sort_by_key(|r| r.src_start);

        Self { rules }
    }

    fn apply(&self, x: u64) -> u64 {
        self.rules.iter().find_map(|r| r.map(x)).unwrap_or(x)
    }

    fn apply_ranges(&self, ranges: Vec<Range>) -> Vec<Range> {
        let mut out: Vec<Range> = Vec::new();
        for r in ranges {
            out.extend(self.apply_range(r));
        }
        merge_ranges(out)
    }

    fn apply_range(&self, range: Range) -> Vec<Range> {
        let mut out: Vec<Range> = Vec::new();
        let mut pos = range.start;

        while pos < range.end {
            let mut next_rule: Option<Rule> = None;

            for &rule in &self.rules {
                if rule.src_end() <= pos {
                    continue;
                }
                next_rule = Some(rule);
                break;
            }

            match next_rule {
                None => {
                    out.push(Range {
                        start: pos,
                        end: range.end,
                    });
                    break;
                }
                Some(rule) => {
                    let rs = rule.src_start;
                    let re = rule.src_end();

                    if pos < rs {
                        let chunk_end = range.end.min(rs);
                        out.push(Range {
                            start: pos,
                            end: chunk_end,
                        });
                        pos = chunk_end;
                    } else {
                        let chunk_end = range.end.min(re);
                        let mapped_start = rule.dst_start + (pos - rs);
                        let mapped_end = mapped_start + (chunk_end - pos);
                        out.push(Range {
                            start: mapped_start,
                            end: mapped_end,
                        });
                        pos = chunk_end;
                    }
                }
            }
        }

        out
    }
}

fn merge_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    ranges.sort_by_key(|r| r.start);

    let mut out: Vec<Range> = Vec::new();
    for r in ranges {
        if let Some(last) = out.last_mut() {
            if r.start <= last.end {
                last.end = last.end.max(r.end);
            } else {
                out.push(r);
            }
        } else {
            out.push(r);
        }
    }
    out
}

impl Rule {
    fn parse(line: &str) -> Self {
        let mut it = line.split_whitespace();
        let dst_start = it.next().unwrap().parse::<u64>().unwrap();
        let src_start = it.next().unwrap().parse::<u64>().unwrap();
        let len = it.next().unwrap().parse::<u64>().unwrap();
        Self {
            dst_start,
            src_start,
            len,
        }
    }

    fn map(&self, x: u64) -> Option<u64> {
        if self.src_start <= x && x < self.src_end() {
            Some(self.dst_start + (x - self.src_start))
        } else {
            None
        }
    }

    fn src_end(&self) -> u64 {
        self.src_start + self.len
    }
}

pub fn solve_part1(input: &str) -> u64 {
    Almanac::parse_part1(input).lowest_location_part1()
}

pub fn solve_part2(input: &str) -> u64 {
    let (mut ranges, maps) = Almanac::parse_part2(input);

    for map in maps {
        ranges = map.apply_ranges(ranges);
    }

    ranges.iter().map(|r| r.start).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(EXAMPLE), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(EXAMPLE), 46);
    }
}
