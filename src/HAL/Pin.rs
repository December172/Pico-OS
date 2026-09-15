pub struct Pin {
    N : u8
}

impl Pin {
    pub(crate) fn new(n : u8) -> Pin {
        Pin {
            N : n 
        }
    }

    pub fn get(&self) -> u8 {
        return self.N;
    }
}