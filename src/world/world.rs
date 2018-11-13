use utils::Field;


struct World {
    map: Field<u32>
}

impl World {
    pub fn new(width: usize, heigth: usize) -> World {
        World {
            map: Field::new(width, heigth, 2)
        }
    }
}