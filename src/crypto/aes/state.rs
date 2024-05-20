use super::utils;

#[derive(Debug)]
pub struct State {
    state: [[u8; 4]; 4],
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl State {
    pub fn new(in_array: [u8; 16]) -> State {
        let mut state = [[0, 0, 0, 0]; 4];
        for c in 0..4 {
            for r in 0..4 {
                state[r][c] = in_array[r + 4 * c];
            }
        }

        State { state }
    }

    pub fn get_current_state(self) -> [u8; 16] {
        let mut out = [0; 16];
        for c in 0..4 {
            for r in 0..4 {
                out[r + 4 * c] = self.state[r][c];
            }
        }
        out
    }

    pub fn sub_bytes(&mut self) {
        for c in 0..4 {
            for r in 0..4 {
                self.state[r][c] = utils::sbox(self.state[r][c]);
            }
        }
    }

    pub fn inv_sub_bytes(&mut self) {
        for c in 0..4 {
            for r in 0..4 {
                self.state[r][c] = utils::inv_sbox(self.state[r][c]);
            }
        }
    }

    pub fn shift_rows(&mut self) {
        (1..4).for_each(|r| {
            (0..r).for_each(|_| {
                self.state[r] = [
                    self.state[r][1],
                    self.state[r][2],
                    self.state[r][3],
                    self.state[r][0],
                ];
            });
        });
    }

    pub fn inv_shift_rows(&mut self) {
        (1..4).for_each(|r| {
            (0..r).for_each(|_| {
                self.state[r] = [
                    self.state[r][3],
                    self.state[r][0],
                    self.state[r][1],
                    self.state[r][2],
                ];
            });
        });
    }

    pub fn mix_columns(&mut self) {
        for c in 0..4 {
            let s_0c = utils::gf8_mul(0x02, self.state[0][c])
                ^ utils::gf8_mul(0x03, self.state[1][c])
                ^ self.state[2][c]
                ^ self.state[3][c];
            let s_1c = self.state[0][c]
                ^ utils::gf8_mul(0x02, self.state[1][c])
                ^ utils::gf8_mul(0x03, self.state[2][c])
                ^ self.state[3][c];
            let s_2c = self.state[0][c]
                ^ self.state[1][c]
                ^ utils::gf8_mul(0x02, self.state[2][c])
                ^ utils::gf8_mul(0x03, self.state[3][c]);
            let s_3c = utils::gf8_mul(0x03, self.state[0][c])
                ^ self.state[1][c]
                ^ self.state[2][c]
                ^ utils::gf8_mul(0x02, self.state[3][c]);

            self.state[0][c] = s_0c;
            self.state[1][c] = s_1c;
            self.state[2][c] = s_2c;
            self.state[3][c] = s_3c;
        }
    }

    pub fn inv_mix_columns(&mut self) {
        for c in 0..4 {
            let s_0c = utils::gf8_mul(0x0e, self.state[0][c])
                ^ utils::gf8_mul(0x0b, self.state[1][c])
                ^ utils::gf8_mul(0x0d, self.state[2][c])
                ^ utils::gf8_mul(0x09, self.state[3][c]);
            let s_1c = utils::gf8_mul(0x09, self.state[0][c])
                ^ utils::gf8_mul(0x0e, self.state[1][c])
                ^ utils::gf8_mul(0x0b, self.state[2][c])
                ^ utils::gf8_mul(0x0d, self.state[3][c]);
            let s_2c = utils::gf8_mul(0x0d, self.state[0][c])
                ^ utils::gf8_mul(0x09, self.state[1][c])
                ^ utils::gf8_mul(0x0e, self.state[2][c])
                ^ utils::gf8_mul(0x0b, self.state[3][c]);
            let s_3c = utils::gf8_mul(0x0b, self.state[0][c])
                ^ utils::gf8_mul(0x0d, self.state[1][c])
                ^ utils::gf8_mul(0x09, self.state[2][c])
                ^ utils::gf8_mul(0x0e, self.state[3][c]);

            self.state[0][c] = s_0c;
            self.state[1][c] = s_1c;
            self.state[2][c] = s_2c;
            self.state[3][c] = s_3c;
        }
    }

    pub fn add_round_key(&mut self, round_key: [[u8; 4]; 4]) {
        for c in 0..4 {
            for (r, key_row) in round_key.iter().enumerate() {
                self.state[r][c] ^= key_row[c];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_state() {
        let input = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let correct_state = State {
            state: [[0, 4, 8, 12], [1, 5, 9, 13], [2, 6, 10, 14], [3, 7, 11, 15]],
        };

        let state = State::new(input);
        assert_eq!(correct_state, state);
    }

    #[test]
    fn get_current_state() {
        let input = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

        let state = State::new(input);

        assert_eq!(state.get_current_state(), input);
    }

    #[test]
    fn sub_bytes() {
        let mut state = State {
            state: [
                [0x00, 0x1F, 0x0E, 0x54],
                [0x3C, 0x4E, 0x08, 0x59],
                [0x6E, 0x22, 0x1B, 0x0B],
                [0x47, 0x74, 0x31, 0x1A],
            ],
        };

        state.sub_bytes();

        let valid_state_after = State {
            state: [
                [0x63, 0xC0, 0xAB, 0x20],
                [0xEB, 0x2F, 0x30, 0xCB],
                [0x9F, 0x93, 0xAF, 0x2B],
                [0xA0, 0x92, 0xC7, 0xA2],
            ],
        };

        assert_eq!(state, valid_state_after);
    }

    #[test]
    fn inv_sub_bytes() {
        let mut state = State {
            state: [
                [0x63, 0xC0, 0xAB, 0x20],
                [0xEB, 0x2F, 0x30, 0xCB],
                [0x9F, 0x93, 0xAF, 0x2B],
                [0xA0, 0x92, 0xC7, 0xA2],
            ],
        };

        state.inv_sub_bytes();

        let valid_state_after = State {
            state: [
                [0x00, 0x1F, 0x0E, 0x54],
                [0x3C, 0x4E, 0x08, 0x59],
                [0x6E, 0x22, 0x1B, 0x0B],
                [0x47, 0x74, 0x31, 0x1A],
            ],
        };

        assert_eq!(state, valid_state_after);
    }

    #[test]
    fn shift_rows() {
        let mut state = State {
            state: [[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11], [12, 13, 14, 15]],
        };

        state.shift_rows();

        let valid_state_after = State {
            state: [[0, 1, 2, 3], [5, 6, 7, 4], [10, 11, 8, 9], [15, 12, 13, 14]],
        };

        assert_eq!(state, valid_state_after);
    }

    #[test]
    fn inv_shift_rows() {
        let mut state = State {
            state: [[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11], [12, 13, 14, 15]],
        };

        state.inv_shift_rows();

        let valid_state_after = State {
            state: [[0, 1, 2, 3], [7, 4, 5, 6], [10, 11, 8, 9], [13, 14, 15, 12]],
        };

        assert_eq!(state, valid_state_after);
    }

    #[test]
    fn mix_columns() {
        let mut state = State {
            state: [
                [0x63, 0xEB, 0x9F, 0xA0],
                [0x2F, 0x93, 0x92, 0xC0],
                [0xAF, 0xC7, 0xAB, 0x30],
                [0xA2, 0x20, 0xCB, 0x2B],
            ],
        };

        state.mix_columns();

        let valid_state_after = State {
            state: [
                [0xBA, 0x84, 0xE8, 0x1B],
                [0x75, 0xA4, 0x8D, 0x40],
                [0xF4, 0x8D, 0x06, 0x7D],
                [0x7A, 0x32, 0x0E, 0x5D],
            ],
        };

        assert_eq!(state, valid_state_after);
    }

    #[test]
    fn inv_mix_columns() {
        let mut state = State {
            state: [
                [0xBA, 0x84, 0xE8, 0x1B],
                [0x75, 0xA4, 0x8D, 0x40],
                [0xF4, 0x8D, 0x06, 0x7D],
                [0x7A, 0x32, 0x0E, 0x5D],
            ],
        };
        state.inv_mix_columns();

        let valid_state_after = State {
            state: [
                [0x63, 0xEB, 0x9F, 0xA0],
                [0x2F, 0x93, 0x92, 0xC0],
                [0xAF, 0xC7, 0xAB, 0x30],
                [0xA2, 0x20, 0xCB, 0x2B],
            ],
        };

        assert_eq!(state, valid_state_after);
    }

    #[test]
    fn add_round_key() {
        let mut state = State {
            state: [
                [0x54, 0x4F, 0x4E, 0x20],
                [0x77, 0x6E, 0x69, 0x54],
                [0x6F, 0x65, 0x6E, 0x77],
                [0x20, 0x20, 0x65, 0x6F],
            ],
        };

        let round_key = [
            [0x54, 0x73, 0x20, 0x67],
            [0x68, 0x20, 0x4B, 0x20],
            [0x61, 0x6D, 0x75, 0x46],
            [0x74, 0x79, 0x6E, 0x75],
        ];

        let valid_state_after = State {
            state: [
                [0x00, 0x3C, 0x6E, 0x47],
                [0x1F, 0x4E, 0x22, 0x74],
                [0x0E, 0x08, 0x1B, 0x31],
                [0x54, 0x59, 0x0B, 0x1A],
            ],
        };

        state.add_round_key(round_key);

        assert_eq!(state.state, valid_state_after.state);
    }
}
