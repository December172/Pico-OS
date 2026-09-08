pub struct Pin {
    N : u8;
}

impl Pin {
    pub fn new(n : &u8) -> Pin {
        Pin {
            N : n 
        }
    }

    pub fn get(&self) -> u8 {
        return self.N;
    }

    pub fn set(self, n : &u8) {
        let self.N = n;
    }
}