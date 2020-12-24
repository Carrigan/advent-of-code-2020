#[derive(Debug, Clone, Copy)]
pub struct Orientation {
    pub rotation: Rotation,
    pub flipped: bool
}

impl Orientation {
    pub fn neutral() -> Orientation {
        Orientation { rotation: Rotation::RightSideUp, flipped: false }
    }

    pub fn index_zero_side(&self) -> u32 {
        let conv = self.rotation as u32;

        match self.flipped {
            false => if conv % 2 == 1 { (conv + 2) % 4 } else { conv },
            true => conv
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Rotation {
    RightSideUp = 0,
    RotatedOnceClockwise = 1,
    UpsideDown = 2,
    RotatedOnceCounterClockwise = 3
}

#[derive(Debug)]
pub enum MatingSide {
    NormalTop,
    NormalRight,
    NormalBottom,
    NormalLeft,
    FlippedTop,
    FlippedRight,
    FlippedBottom,
    FlippedLeft
}

impl From<u32> for Rotation {
    fn from(value: u32) -> Self {
        let value = value % 4;

        match value {
            0 => Rotation::RightSideUp,
            1 => Rotation::RotatedOnceClockwise,
            2 => Rotation::UpsideDown,
            3 => Rotation::RotatedOnceCounterClockwise,
            _ => panic!()
        }
    }
}

impl Rotation {
    pub fn rotate_cw(self, amount: u32) -> Rotation {
        Rotation::from((self as u32 + amount) % 4)
    }

    pub fn rotate_ccw(self, amount: u32) -> Rotation {
        Rotation::from((4 + self as u32 - amount) % 4)
    }
}

pub fn index_rotated_grid(x: usize, y: usize, width: usize, height: usize, orientation: Orientation) -> (usize, usize) {
    let height_index = height - 1;
    let width_index = width - 1;

    match orientation.flipped {
        false => {
            match orientation.rotation {
                Rotation::RightSideUp => (x, y),
                Rotation::RotatedOnceClockwise => (y, width_index - x),
                Rotation::UpsideDown => (width_index - x, height_index - y),
                Rotation::RotatedOnceCounterClockwise => (height_index - y, x),
            }
        }
        true => {
            match orientation.rotation {
                Rotation::RightSideUp => (width_index - x, y),
                Rotation::RotatedOnceClockwise => (height_index - y, width_index - x),
                Rotation::UpsideDown => (x, height_index - y),
                Rotation::RotatedOnceCounterClockwise => (y, x),
            }
        }
    }
}
