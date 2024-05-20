use super::utils;

#[derive(PartialEq, Debug, Clone, Copy)]
struct State {
    data: [u32; 16],
}

impl State {
    pub fn new(key: [u8; 32], nonce: [u8; 12], counter: u32) -> Self {
        let mut data = [0u32; 16];
        (data[0], data[1], data[2], data[3]) = (0x61707865, 0x3320646e, 0x79622d32, 0x6b206574);

        for (i, key_seg) in key.chunks(4).enumerate() {
            data[i + 4] = (key_seg[3] as u32).rotate_left(24)
                + (key_seg[2] as u32).rotate_left(16)
                + (key_seg[1] as u32).rotate_left(8)
                + (key_seg[0] as u32);
        }

        data[12] = counter;

        for (i, nonce_seg) in nonce.chunks(4).enumerate() {
            data[i + 13] = (nonce_seg[3] as u32).rotate_left(24)
                + (nonce_seg[2] as u32).rotate_left(16)
                + (nonce_seg[1] as u32).rotate_left(8)
                + (nonce_seg[0] as u32);
        }

        State { data }
    }

    /// RFC 7539 - Section 2.2 - A Quarter Round on the ChaCha State
    pub fn quarter_round(&mut self, x: usize, y: usize, z: usize, w: usize) {
        (self.data[x], self.data[y], self.data[z], self.data[w]) =
            utils::quarter_round_on_vector(self.data[x], self.data[y], self.data[z], self.data[w]);
    }

    fn eight_quarter_rounds(&mut self) {
        self.quarter_round(0, 4, 8, 12);
        self.quarter_round(1, 5, 9, 13);
        self.quarter_round(2, 6, 10, 14);
        self.quarter_round(3, 7, 11, 15);
        self.quarter_round(0, 5, 10, 15);
        self.quarter_round(1, 6, 11, 12);
        self.quarter_round(2, 7, 8, 13);
        self.quarter_round(3, 4, 9, 14);
    }

    pub fn serialize(self) -> [u8; 64] {
        let mut result = [0u8; 64];

        for (i, word) in self.data.iter().enumerate() {
            result[4 * i] = (word & 0xff) as u8;
            result[4 * i + 1] = ((word >> 8) & 0xff) as u8;
            result[4 * i + 2] = ((word >> 16) & 0xff) as u8;
            result[4 * i + 3] = ((word >> 24) & 0xff) as u8;
        }

        result
    }
}

impl std::ops::AddAssign for State {
    fn add_assign(&mut self, rhs: Self) {
        self.data.iter_mut().enumerate().for_each(|(i, e)| {
            *e = e.wrapping_add(rhs.data[i]);
        });
    }
}

/// RFC 7539 - Section 2.3 - The ChaCha20 Block Function
///
/// Pseudocode:
/// ```pseudocode
/// chacha20_block(key, counter, nonce):
///     state = constants | key | counter | nonce
///     working_state = state
///     for i=1 upto 10
///        inner_block(working_state)
///     end
///     state += working_state
///     return serialize(state)
/// end
/// ```
pub fn chacha20_block(key: [u8; 32], nonce: [u8; 12], counter: u32) -> [u8; 64] {
    #[rustfmt::skip]
    let mut state = State::new(key, nonce, counter);

    let mut working_state = state;

    (0..10).for_each(|_| {
        working_state.eight_quarter_rounds();
    });

    state += working_state;

    state.serialize()
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_new_state() {
        let key: [u8; 32] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
            0x1c, 0x1d, 0x1e, 0x1f,
        ];
        let nonce: [u8; 12] = [
            0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x4a, 0x00, 0x00, 0x00, 0x00,
        ];
        let block_count: u32 = 1;

        let state = State::new(key, nonce, block_count);

        let valid_state = State {
            data: [
                0x61707865, 0x3320646e, 0x79622d32, 0x6b206574,
                0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c,
                0x13121110, 0x17161514, 0x1b1a1918, 0x1f1e1d1c,
                0x00000001, 0x09000000, 0x4a000000, 0x00000000,
            ],
        };
        assert_eq!(state, valid_state);
    }

    #[test]
    #[rustfmt::skip]
    /// RFC 7539 - Section 2.2.1 - Test Vector for the Quarter Round on the ChaCha State
    fn test_quarter_round() {

        let mut state = State {
            data: [
                0x879531e0, 0xc5ecf37d, 0x516461b1, 0xc9a62f8a,
                0x44c20ef3, 0x3390af7f, 0xd9fc690b, 0x2a5f714c, 
                0x53372767, 0xb00a5631, 0x974c541a, 0x359e9963,
                0x5c971061, 0x3d631689, 0x2098d9d6, 0x91dbd320,
            ],
        };

        state.quarter_round(2, 7, 8, 13);

        let valid_state_after = State {
            data: [
                0x879531e0, 0xc5ecf37d, 0xbdb886dc, 0xc9a62f8a,
                0x44c20ef3, 0x3390af7f, 0xd9fc690b, 0xcfacafd2, 
                0xe46bea80, 0xb00a5631, 0x974c541a, 0x359e9963,
                0x5c971061, 0xccc07c79, 0x2098d9d6, 0x91dbd320,
            ],
        };

        assert_eq!(state, valid_state_after);
    }

    #[test]
    /// RFC 7539 - Section 2.3.2 - Test Vector for the ChaCha20 Block Function
    fn test_chacha20_block() {
        let key: [u8; 32] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
            0x1c, 0x1d, 0x1e, 0x1f,
        ];
        let nonce: [u8; 12] = [
            0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x4a, 0x00, 0x00, 0x00, 0x00,
        ];
        let block_count: u32 = 1;

        let result = chacha20_block(key, nonce, block_count);

        let valid_result: [u8; 64] = [
            0x10, 0xf1, 0xe7, 0xe4, 0xd1, 0x3b, 0x59, 0x15, 0x50, 0x0f, 0xdd, 0x1f, 0xa3, 0x20,
            0x71, 0xc4, 0xc7, 0xd1, 0xf4, 0xc7, 0x33, 0xc0, 0x68, 0x03, 0x04, 0x22, 0xaa, 0x9a,
            0xc3, 0xd4, 0x6c, 0x4e, 0xd2, 0x82, 0x64, 0x46, 0x07, 0x9f, 0xaa, 0x09, 0x14, 0xc2,
            0xd7, 0x05, 0xd9, 0x8b, 0x02, 0xa2, 0xb5, 0x12, 0x9c, 0xd1, 0xde, 0x16, 0x4e, 0xb9,
            0xcb, 0xd0, 0x83, 0xe8, 0xa2, 0x50, 0x3c, 0x4e,
        ];

        assert_eq!(result, valid_result);
    }
}
