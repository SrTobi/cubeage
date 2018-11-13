use std::ops;
use vek::vec::Vec2;

pub struct Field<T> {
    pub width: usize,
    pub height: usize,
    memory: Vec<T>
}


impl<T> Field<T> {
    pub fn new(width: usize, height: usize, default: T) -> Field<T>  where T: Clone {
        Field {
            width,
            height,
            memory: vec![default; width * height]
        }
    }
}

impl<T> ops::Index<Vec2<usize>> for Field<T> {
    type Output = T;

    fn index(&self, pos: Vec2<usize>) -> &T {
        &self.memory[pos.x + pos.y * self.width]
    }
}

impl<T> ops::IndexMut<Vec2<usize>> for Field<T> {
    //type Output = T;

    fn index_mut<'a>(&'a mut self, pos: Vec2<usize>) -> &'a mut T {
        &mut self.memory[pos.x + pos.y * self.width]
    }
}
