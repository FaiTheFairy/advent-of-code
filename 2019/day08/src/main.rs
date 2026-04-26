use std::io::{Read, Write};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let mut image = SpaceImage::new(25, 6);
    let mut buf = Vec::new();
    std::fs::File::open("input.txt")?.read_to_end(&mut buf)?;
    buf.retain(|b| b.is_ascii_digit());
    image.write_all(&buf)?;

    let sol1 = image.checksum().context("checksum")?;
    println!("Part 1: {sol1}");

    let sol2 = image.render();
    println!("Part 2:\n{sol2}");
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SpaceImage {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
}

impl SpaceImage {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![],
        }
    }

    fn layer_size(&self) -> usize {
        self.width * self.height
    }

    fn layers(&self) -> impl Iterator<Item = &[u8]> {
        self.pixels.chunks(self.layer_size())
    }

    fn layer_with_fewest(&self, n: u8) -> Option<&[u8]> {
        self.layers()
            .min_by_key(|layer| layer.iter().filter(|&&pixel| pixel == n).count())
    }

    fn checksum(&self) -> Option<usize> {
        let layer = self.layer_with_fewest(b'0')?;

        let ones = layer.iter().filter(|&&pixel| pixel == b'1').count();
        let twos = layer.iter().filter(|&&pixel| pixel == b'2').count();

        Some(ones * twos)
    }

    fn decoded_pixels(&self) -> Vec<u8> {
        (0..self.layer_size())
            .map(|idx| {
                self.layers()
                    .map(|layer| layer[idx])
                    .find(|&pixel| pixel != b'2')
                    .unwrap_or(b'2')
            })
            .collect()
    }

    fn render(&self) -> String {
        let pixels = self.decoded_pixels();

        pixels
            .chunks(self.width)
            .map(|row| {
                row.iter()
                    .map(|&pixel| match pixel {
                        b'0' => ' ',
                        b'1' => '#',
                        b'2' => ' ',
                        _ => '?',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl std::io::Write for SpaceImage {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if buf.iter().any(|b| !b.is_ascii_digit()) {
            return Err(std::io::ErrorKind::InvalidData.into());
        }
        self.pixels.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.pixels.flush()
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    fn example_image() -> SpaceImage {
        let mut image = SpaceImage::new(3, 2);
        image.write_all("123456789012".as_bytes()).unwrap();
        image
    }

    #[test]
    fn spaceimage_new() {
        let image = example_image();
        assert_eq!(
            image,
            SpaceImage {
                width: 3,
                height: 2,
                pixels: b"123456789012".to_vec()
            }
        );
    }
}
