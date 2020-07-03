#[derive(Debug, Default)]
pub struct Probability {
    pub high: u32,
    pub low: u32,
    pub count: u32,
}

const MAX_FREQ: u32 = 0x3FFF;

pub struct Model {
    cumulative_frequency: [u32; 258],
    frozen: bool,
}

impl Model {
    pub fn new() -> Self {
        let mut cumulative_frequency = [0u32; 258];
        for i in 0..258 {
            cumulative_frequency[i] = i as u32;
        }
        Model {
            cumulative_frequency,
            frozen: false,
        }
    }

    fn update(&mut self, c: u32) {
        for i in c + 1..258 {
            self.cumulative_frequency[i as usize] += 1;
        }
        if self.cumulative_frequency[257] >= MAX_FREQ {
            self.frozen = true;
        }
    }

    pub fn get_probability(&mut self, c: u32) -> Probability {
        let p = Probability {
            low: self.cumulative_frequency[c as usize],
            high: self.cumulative_frequency[c as usize + 1],
            count: self.cumulative_frequency[257],
        };

        if !self.frozen {
            self.update(c);
        }

        p
    }
}
