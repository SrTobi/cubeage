//! Helper methods and structures for working with cubes.
//!
//! ```ignore
//!         3  ---------  2
//!           /       / |
//!          /  up   /  |
//!      6  -------- 7  | 1
//!        |        |  /
//! west   |  south | /  east
//!        |        |/
//!      5  -------- 4
//! ```
//!
//!
//! ```ignore
//!         7  ---------  6
//!           /       / |
//!          /  up   /  |
//!      2  -------- 3  | 5
//!        |        |  /
//! east   |  north | /  west
//!        |        |/
//!      1  -------- 0
//! ```

use std::str::FromStr;
use vek::vec::Vec3;

pub use self::Face::{
    Down,
    Up,
    North,
    South,
    West,
    East,
};

/// Cube faces (clockwise).
pub const QUADS: &'static [[usize; 4]; 6] = &[
    [1, 0, 5, 4], // down
    [7, 6, 3, 2], // up
    [0, 1, 2, 3], // north
    [4, 5, 6, 7], // south
    [5, 0, 3, 6], // west
    [1, 4, 7, 2]  // east
];

/// Cube vertices.
pub const VERTICES: &'static [Vec3<f32>; 8] = &[
    // This is the north surface
    Vec3 { x: 0.0, y: 0.0, z: 0.0}, // 0
    Vec3 { x: 1.0, y: 0.0, z: 0.0}, // 1
    Vec3 { x: 1.0, y: 1.0, z: 0.0}, // 2
    Vec3 { x: 0.0, y: 1.0, z: 0.0}, // 3

    // This is the south surface
    Vec3 { x: 1.0, y: 0.0, z: 1.0}, // 4
    Vec3 { x: 0.0, y: 0.0, z: 1.0}, // 5
    Vec3 { x: 0.0, y: 1.0, z: 1.0}, // 6
    Vec3 { x: 1.0, y: 1.0, z: 1.0},  // 7
];

/// A value representing face direction.
#[repr(usize)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub enum Face {
    /// Facing down.
    Down,
    /// Facing up.
    Up,
    /// Facing north.
    North,
    /// Facing south.
    South,
    /// Facing west.
    West,
    /// Facing east.
    East
}

impl Face {
    /// Computes vertices of the face.
    pub fn vertices(self, base: Vec3<f32>, scale: Vec3<f32>) -> [Vec3<f32>; 4] {
        use array::*;

        QUADS[self as usize].map(|i| VERTICES[i]).map(|v| {
            base + scale * v
        })
    }

    /// Gets the direction of face.
    pub fn direction(self) -> Vec3<i32> {
        match self {
            Down  => Vec3::new(0, -1,  0),
            Up    => Vec3::new(0,  1,  0),
            North => Vec3::new(0,  0, -1),
            South => Vec3::new(0,  0,  1),
            West  => Vec3::new(-1,  0,  0),
            East  => Vec3::new(1,  0,  0)
        }
    }

    /// Gets the face in a specific direction.
    pub fn from_direction(d: Vec3<i32>) -> Option<Self> {
        Some(match (d[0], d[1], d[2]) {
            ( 0, -1,  0) => Down,
            ( 0,  1,  0) => Up,
            ( 0,  0, -1) => North,
            ( 0,  0,  1) => South,
            (-1,  0,  0) => West,
            ( 1,  0,  0) => East,
            _ => return None
        })
    }

    /// Convert number to face.
    pub fn from_usize(number: usize) -> Option<Self> {
        Some(match number {
            0 => Down,
            1 => Up,
            2 => North,
            3 => South,
            4 => West,
            5 => East,
            _ => return None
        })
    }
}

/// The error parsing face from string.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ParseError;

impl FromStr for Face {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Ok(match s {
            "down"  => Down,
            "up"    => Up,
            "north" => North,
            "south" => South,
            "west"  => West,
            "east"  => East,
            _ => return Err(ParseError)
        })
    }
}

/// Iterates through each face on a cube.
#[derive(Copy, Clone)]
pub struct FaceIterator(usize);

impl FaceIterator {
    /// Creates a new face iterator.
    pub fn new() -> Self {
        FaceIterator(0)
    }
}

impl Iterator for FaceIterator {
    type Item = Face;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let face = self.0;
        if face < 6 {
            self.0 += 1;
            Face::from_usize(face)
        } else {
            None
        }
    }
}