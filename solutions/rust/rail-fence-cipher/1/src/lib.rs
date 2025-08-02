pub struct RailFence(u32);

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        Self(rails)
    }

    pub fn encode(&self, text: &str) -> String {
        let rails = self.0 as usize;
        let mut cipher = vec![Vec::new(); rails];
        let mut go_down = true;
        let mut y = 0;

        for (i, c) in text.chars().enumerate() {
            for v in cipher.iter_mut() {
                v.push(' ');
            }

            let x = i;

            cipher[y][x] = c;

            y = match go_down {
                true if y + 1 < rails => y + 1,
                true if y + 1 >= rails => {
                    go_down = false;
                    y - 1
                }
                false if y > 0 => y - 1,
                false if y == 0 => {
                    go_down = true;
                    y + 1
                }
                _ => panic!(),
            };
        }

        cipher
            .iter()
            .map(|v| v.iter().filter(|c| !c.is_whitespace()).collect::<String>())
            .collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let rails = self.0;

        if rails <= 1 {
            return cipher.to_string();
        }

        let chars: Vec<char> = cipher.chars().collect();
        let len = chars.len();

        let mut positions = vec![0; len];
        let mut rail = 0;
        let mut direction_down = true;

        for position in positions.iter_mut() {
            *position = rail;

            if rail == 0 {
                direction_down = true;
            } else if rail == rails - 1 {
                direction_down = false;
            }

            if direction_down {
                rail += 1;
            } else {
                rail -= 1;
            }
        }

        let mut result = vec![' '; len];
        let mut char_index = 0;

        for current_rail in 0..rails {
            for i in 0..len {
                if positions[i] == current_rail {
                    result[i] = chars[char_index];
                    char_index += 1;
                }
            }
        }

        result.into_iter().collect()
    }
}
