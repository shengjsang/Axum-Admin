use rand::Rng;

pub struct Random {
    pub length: usize,
}

impl Random {
    pub fn new(length: usize) -> Self {
        Self { length }
    }

    pub fn generate(&self) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
        let mut rng = rand::thread_rng();

        let value: String = (0..self.length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        value
    }
}
