pub struct Sled {
    slope: (i32, i32),
    initial_position: (i32, i32)
}

impl Sled {
    pub fn new() -> Sled {
        Sled {
            slope: (0, 0),
            initial_position: (0, 0)
        }
    }

    pub fn set_position(&mut self, position: (i32, i32)) {
        self.initial_position = position;
    }
    
    pub fn set_slope(&mut self, slope: (i32, i32)) {
        self.slope = slope;
    }

    pub fn get_position_at_time(&self, t:i32) -> (i32,i32){
        let (mut x, mut y) = self.initial_position;
        let (x_offset, y_offset) = self.slope;
        x += x_offset * t;
        y += y_offset * t; 
        return (x,y);
    }
}