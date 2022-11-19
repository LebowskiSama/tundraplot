pub struct AccDataFrame {
    pub timestamp: Vec<i32>,
    pub x: Vec<i32>,
    pub y: Vec<i32>,
    pub z: Vec<i32>,
}

impl AccDataFrame {
    pub fn new() -> Self {
        AccDataFrame {
            timestamp: Vec::new(),
            x: Vec::new(),
            y: Vec::new(),
            z: Vec::new(),
        }
    }
    
    pub fn push(&mut self, t: i32, x: i32, y: i32, z: i32) {
        self.timestamp.push(t);
        self.x.push(x);
        self.y.push(y);
        self.z.push(z);
    }
}


impl Default for AccDataFrame {
    fn default() -> Self {
        Self::new()
    }
}