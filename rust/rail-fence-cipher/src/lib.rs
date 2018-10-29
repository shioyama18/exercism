pub struct RailFence {
    rails: usize,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence { rails: rails as usize }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut rails = vec![String::new(); self.rails];
        let mut rail: isize = 0;
        let mut direction: isize = 1;

        for c in text.chars() {
            rails[rail as usize].push(c);

            if rail == 0 {
                direction = 1;
            }

            if rail == rails.len() as isize - 1 {
                direction = -1;
            }

            rail += direction;
        }

        rails.join("")
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut decoded = vec![String::new(); cipher.len()];
        let mut it = cipher.chars();

        for rail_i in 0..self.rails {
            for cipher_i in 0..cipher.len() {
                let pos = cipher_i % (2 * self.rails - 2);
                if pos == rail_i || pos == 2 * self.rails - 2 - rail_i {
                    if let Some(c) = it.next() {
                        decoded[cipher_i].push(c);
                    }
                }
            }
        }

        decoded.join("")
    }
}
