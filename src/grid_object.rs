use rand::Rng;

macro_rules! rand_range {
    ($a:expr, $b:expr) => {
        rand::thread_rng().gen_range($a..$b)
    };
}

pub struct GridObject {
    grid: Vec<bool>,
    lenght: usize,
    height: usize,
}

impl GridObject {
    pub fn new(lenght_param: usize, height_param: usize) -> GridObject {
        GridObject {
            grid: vec![false; lenght_param * height_param],
            lenght: lenght_param,
            height: height_param,
        }
    }

    pub fn uniform_to_value(&mut self, value: bool) -> &mut Self {
        self.grid = vec![value; self.height * self.lenght];
        self
    }

    pub fn randomize(&mut self) -> &mut Self {
        for y in 0..self.height {
            for x in 0..self.lenght {
                Self::set_cell(self, rand_range!(0, 2) == 1, x, y);
            }
        }
        self
    }

    pub fn _output(&self) {
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
                let new_value = Self::rule_set(self, x, y);
                Self::set_cell(self, new_value, x, y);
            }
        }
    }

    fn rule_set(&mut self, x: usize, y: usize) -> bool {
        //let orig_value: bool = self.grid[x + self.lenght * y];
        let mut surounding_values: Vec<bool> = Vec::new();

        for y_off in -1..=1 {
            for x_off in -1..=1 {
                surounding_values.push(Self::_get_cell(
                    self,
                    (x as i32 + x_off).max(0) as usize,
                    (y as i32 + y_off).max(0) as usize,
                ))
            }
        }

        let mut living_counter = 0;

        for state in surounding_values {
            if state {
                living_counter += 1;
            }
        }

        !(5..=7).contains(&living_counter)
    }

    pub fn get_lenght(&self) -> usize {
        self.lenght
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn set_cell(&mut self, value: bool, x: usize, y: usize) -> &mut Self {
        self.grid[x + self.lenght * y] = value;
        self
    }

    pub fn _get_cell(&self, x: usize, y: usize) -> bool {
        if x > self.lenght - 1 || y > self.height - 1 {
            return false;
        }
        self.grid[x + self.lenght * y]
    }

    pub fn _flip_cell(&mut self, x: usize, y: usize) -> &mut Self {
        Self::set_cell(self, !Self::_get_cell(self, x, y), x, y);
        self
    }
}
