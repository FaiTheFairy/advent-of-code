use std::{fs, str::FromStr};

use anyhow::{Result, bail, ensure};

fn main() -> Result<()> {
    let group: Group = fs::read_to_string("input.txt")?.parse()?;

    let sol1 = group.score();
    println!("Part 1: {sol1}");

    let sol2 = group.garbage_count();
    println!("Part 2: {sol2}");
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Group {
    children: Vec<Node>,
}

impl Group {
    fn score(&self) -> u32 {
        self.score_from(1)
    }

    fn score_from(&self, depth: u32) -> u32 {
        let mut total = depth;

        for child in &self.children {
            total += child.score_from(depth + 1);
        }

        total
    }

    fn garbage_count(&self) -> u32 {
        let mut total = 0;

        for child in &self.children {
            total += child.garbage_count();
        }

        total
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Node {
    Group(Group),
    Garbage(Garbage),
}

impl Node {
    fn score_from(&self, depth: u32) -> u32 {
        match self {
            Node::Group(group) => group.score_from(depth),
            Node::Garbage(_) => 0,
        }
    }

    fn garbage_count(&self) -> u32 {
        match self {
            Node::Group(group) => group.garbage_count(),
            Node::Garbage(garbage) => garbage.count(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Garbage {
    count: u32,
}

impl Garbage {
    fn count(self) -> u32 {
        self.count
    }
}

impl FromStr for Group {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.trim().as_bytes();
        let mut pos = 0;

        let group = parse_group(bytes, &mut pos)?;
        ensure!(pos == bytes.len(), "trailing input");

        Ok(group)
    }
}

fn parse_group(bytes: &[u8], pos: &mut usize) -> Result<Group> {
    if *pos >= bytes.len() || bytes[*pos] != b'{' {
        bail!("expected '{{' at {pos}");
    }
    *pos += 1;

    let mut children = Vec::new();

    if *pos < bytes.len() && bytes[*pos] == b'}' {
        *pos += 1;
        return Ok(Group { children });
    }

    loop {
        children.push(parse_node(bytes, pos)?);

        if *pos >= bytes.len() {
            bail!("unexpected end while parsing group");
        }

        match bytes[*pos] {
            b',' => *pos += 1,
            b'}' => {
                *pos += 1;
                break;
            }
            other => bail!("unexpected char '{}' at {}", other as char, *pos),
        }
    }

    Ok(Group { children })
}

fn parse_node(bytes: &[u8], pos: &mut usize) -> Result<Node> {
    if *pos >= bytes.len() {
        bail!("unexpected end while parsing node");
    }

    match bytes[*pos] {
        b'{' => Ok(Node::Group(parse_group(bytes, pos)?)),
        b'<' => Ok(Node::Garbage(parse_garbage(bytes, pos)?)),
        other => bail!("unexpected char '{}' at {}", other as char, *pos),
    }
}

fn parse_garbage(bytes: &[u8], pos: &mut usize) -> Result<Garbage> {
    if bytes[*pos] != b'<' {
        bail!("expected '<' at {}", *pos);
    }
    *pos += 1;

    let mut count = 0;
    let mut cancel = false;

    while *pos < bytes.len() {
        let b = bytes[*pos];
        *pos += 1;

        if cancel {
            cancel = false;
            continue;
        }

        match b {
            b'!' => cancel = true,
            b'>' => return Ok(Garbage { count }),
            _ => count += 1,
        }
    }

    bail!("unterminated garbage")
}
