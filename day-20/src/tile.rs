
pub fn invert_side(width: u32, side: u32) -> u32 {
    (0..width).fold(0, |total, n|
        (total << 1) + ((side >> n) & 1)
    )
}

#[derive(Debug)]
pub struct Tile {
    pub label: u32,
    pub data: Vec<bool>,
    pub width: u32,
    pub sides: [u32; 4]
}

impl From<&str> for Tile {
    fn from(input: &str) -> Self {
        let mut line_iter = input.lines();
        let label_line = line_iter.next().unwrap();
        let label: u32 = label_line[5..label_line.len()-1].parse().unwrap();
        let mut data = Vec::new();
        let mut width: u32 = 0;

        for row in line_iter {
            width = row.len() as u32;
            row.chars().for_each(|c| data.push(c == '#'))
        }

        let top = (0..width)
            .map(|i| if data[i as usize] { 1 } else { 0 })
            .fold(0, |total, n| (total << 1) + n);

        let right = (0..width)
            .map(|i| if data[((i + 1) * width - 1) as usize] { 1 } else { 0 })
            .fold(0, |total, n| (total << 1) + n);

        let bottom = (0..width)
            .map(|i| if data[data.len() - (i + 1) as usize] { 1 } else { 0 })
            .fold(0, |total, n| (total << 1) + n);

        let left = (0..width)
            .map(|i| if data[data.len() - (width * (i + 1)) as usize] { 1 } else { 0 })
            .fold(0, |total, n| (total << 1) + n);

        let sides = [top, right, bottom, left];

        Tile { label, data, sides, width }
    }
}

impl Tile {
    // Rotate and then flip
    pub fn side_with_translations(&self, index: usize, rotated: usize, flipped: bool) -> u32 {
        let side_index = match flipped {
            true => (8 - (index + rotated)) % 4,
            false => (index + rotated) % 4
        };

        let raw_value = self.sides[side_index];

        if flipped { invert_side(10, raw_value) } else { raw_value }
    }

    pub fn mates(&self, edge: u32) -> Option<(usize, bool)> {
        for (side_index, &side) in self.sides.iter().enumerate() {
            let inverted = invert_side(10, side);

            if side == edge {
                return Some((side_index, true))
            } else if inverted == edge {
                return Some((side_index, false))
            }
        }

        None
    }

    pub fn index(&self, x: usize, y: usize, rotation: usize, flipped: bool) -> bool {
        let (x, y) = super::index_rotated_grid(
            x,
            y,
            self.width as usize - 2,
            self.width as usize - 2,
            rotation,
            flipped
        );

        self.data[(y + 1) * self.width as usize + (x + 1)]
    }

    pub fn show(&self, rotation: usize, flipped: bool) {
        println!("\n{} {}", rotation, flipped);
        for y in 0..8 {
            for x in 0..8 {
                print!("{}", if self.index(x, y, rotation, flipped) { "#" } else { "." });
            }

            println!("");
        }
    }

    pub fn trues(&self) -> usize {
        let mut true_count = 0;

        for y in 0..8 {
            for x in 0..8 {
                if self.index(x, y, 0, false) {
                    true_count += 1;
                }
            }
        }

        true_count
    }
}