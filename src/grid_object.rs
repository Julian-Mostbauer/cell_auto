pub struct GridObject {
    grid: Vec<bool>,
    lenght: usize,
    height: usize,
}

impl GridObject {
    pub fn new(lenght_param: usize, height_param: usize) -> GridObject {
        return GridObject {
            grid: vec![false; lenght_param * height_param],
            lenght: lenght_param,
            height: height_param,
        };
    }

    pub fn uniform_to_value(&mut self, value: bool) {
        self.grid = vec![value; self.height * self.lenght];
    }

    pub fn output(&self) {
        for y in 0..self.height {
            for x in 0..self.lenght {
                print!("{}", self.grid[x + self.lenght * y] as u8);
            }
            println!();
        }
    }

    pub fn apply_rules(&mut self) {
        for y in 0..self.height {
            for x in 0..self.lenght {
                Self::set_cell(
                    self,
                    Self::rule_set(self.grid[x + self.lenght * y], x, y),
                    x,
                    y,
                )
            }
        }
    }

    fn rule_set(value: bool, x: usize, y: usize) -> bool {
        if (x % 2 == 0) && (y % 3 == 0) {
            return !value;
        }
        return false;
    }

    pub fn set_cell(&mut self, value: bool, x: usize, y: usize) {
        self.grid[x + self.lenght * y] = value;
    }

    pub fn get_cell(&self, x: usize, y: usize) -> bool {
        return self.grid[x + self.lenght * y];
    }

    pub fn flip_cell(&mut self, x: usize, y: usize) {
        Self::set_cell(self, !Self::get_cell(self, x, y), x, y);
    }
}
