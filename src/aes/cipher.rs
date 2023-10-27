use super::{state::State, word::Word};

pub fn cipher(in_array: [u8; 16], nr: usize, w: Vec<Word>) -> [u8; 16] {
    /*
        state <-- in                                                          . See Sec. 3.4
        state <-- add_round_key(state, w[0..3])                               . See Sec. 5.1.4
        for round from 1 to Nr − 1 do
            state <-- sub_bytes(state)                                        . See Sec. 5.1.1
            state <-- shift_rows(state)                                       . See Sec. 5.1.2
            state <-- mix_columns(state)                                      . See Sec. 5.1.3
            state <-- add_round_key(state, w[4 ∗ round..4 ∗ round + 3])
        end for
        state <-- sub_bytes(state)
        state <-- shift_rows(state)
        state <-- add_round_key(state, w[4 ∗ Nr..4 ∗ Nr + 3])
        return state                                                          . See Sec. 3.4
    */

    let mut state = State::new(in_array);
    state.add_round_key([
        [w[0][&0], w[1][&0], w[2][&0], w[3][&0]],
        [w[0][&1], w[1][&1], w[2][&1], w[3][&1]],
        [w[0][&2], w[1][&2], w[2][&2], w[3][&2]],
        [w[0][&3], w[1][&3], w[2][&3], w[3][&3]],
    ]);

    for round in 1..nr {
        state.sub_bytes();
        state.shift_rows();
        state.mix_columns();
        state.add_round_key([
            [
                w[4 * round][&0],
                w[4 * round + 1][&0],
                w[4 * round + 2][&0],
                w[4 * round + 3][&0],
            ],
            [
                w[4 * round][&1],
                w[4 * round + 1][&1],
                w[4 * round + 2][&1],
                w[4 * round + 3][&1],
            ],
            [
                w[4 * round][&2],
                w[4 * round + 1][&2],
                w[4 * round + 2][&2],
                w[4 * round + 3][&2],
            ],
            [
                w[4 * round][&3],
                w[4 * round + 1][&3],
                w[4 * round + 2][&3],
                w[4 * round + 3][&3],
            ],
        ]);
    }
    state.sub_bytes();
    state.shift_rows();
    state.add_round_key([
        [
            w[4 * nr][&0],
            w[4 * nr + 1][&0],
            w[4 * nr + 2][&0],
            w[4 * nr + 3][&0],
        ],
        [
            w[4 * nr][&1],
            w[4 * nr + 1][&1],
            w[4 * nr + 2][&1],
            w[4 * nr + 3][&1],
        ],
        [
            w[4 * nr][&2],
            w[4 * nr + 1][&2],
            w[4 * nr + 2][&2],
            w[4 * nr + 3][&2],
        ],
        [
            w[4 * nr][&3],
            w[4 * nr + 1][&3],
            w[4 * nr + 2][&3],
            w[4 * nr + 3][&3],
        ],
    ]);
    state.get_current_state()
}

pub fn inv_cipher(in_array: [u8; 16], nr: usize, w: Vec<Word>) -> [u8; 16] {
    /*
       state <-- in                                                             . See Sec. 3.4
       state <-- add_round_key(state, w[4 ∗ Nr..4 ∗ Nr + 3])                   . See Sec. 5.1.4
       for round from Nr − 1 downto 1 do
           state <-- inv_shift_rows(state)                                      . See Sec. 5.3.1
           state <-- inv_sub_bytes(state)                                       . See Sec. 5.3.2
           state <-- add_round_key(state, w[4 ∗ round..4 ∗ round + 3])
           state <-- inv_mix_columns(state)                                     . See Sec. 5.3.3
       end for
       state <-- inv_shift_rows(state)
       state <-- inv_sub_bytes(state)
       state <-- add_round_key(state, w[0..3])
       return state
    */
    let mut state = State::new(in_array);
    state.add_round_key([
        [
            w[4 * nr][&0],
            w[4 * nr + 1][&0],
            w[4 * nr + 2][&0],
            w[4 * nr + 3][&0],
        ],
        [
            w[4 * nr][&1],
            w[4 * nr + 1][&1],
            w[4 * nr + 2][&1],
            w[4 * nr + 3][&1],
        ],
        [
            w[4 * nr][&2],
            w[4 * nr + 1][&2],
            w[4 * nr + 2][&2],
            w[4 * nr + 3][&2],
        ],
        [
            w[4 * nr][&3],
            w[4 * nr + 1][&3],
            w[4 * nr + 2][&3],
            w[4 * nr + 3][&3],
        ],
    ]);

    for round in (1..nr).rev() {
        println!("{}", round);
        state.inv_shift_rows();
        state.inv_sub_bytes();
        state.add_round_key([
            [
                w[4 * round][&0],
                w[4 * round + 1][&0],
                w[4 * round + 2][&0],
                w[4 * round + 3][&0],
            ],
            [
                w[4 * round][&1],
                w[4 * round + 1][&1],
                w[4 * round + 2][&1],
                w[4 * round + 3][&1],
            ],
            [
                w[4 * round][&2],
                w[4 * round + 1][&2],
                w[4 * round + 2][&2],
                w[4 * round + 3][&2],
            ],
            [
                w[4 * round][&3],
                w[4 * round + 1][&3],
                w[4 * round + 2][&3],
                w[4 * round + 3][&3],
            ],
        ]);
        state.inv_mix_columns();
    }
    state.inv_shift_rows();
    state.inv_sub_bytes();
    state.add_round_key([
        [w[0][&0], w[1][&0], w[2][&0], w[3][&0]],
        [w[0][&1], w[1][&1], w[2][&1], w[3][&1]],
        [w[0][&2], w[1][&2], w[2][&2], w[3][&2]],
        [w[0][&3], w[1][&3], w[2][&3], w[3][&3]],
    ]);
    state.get_current_state()
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use crate::aes::key::Key;

    #[test]
    fn cipher() {
        let key = Key::from([
            0x54, 0x68, 0x61, 0x74, 0x73, 0x20, 0x6D, 0x79, 0x20, 0x4B, 0x75, 0x6E, 0x67, 0x20,
            0x46, 0x75,
        ]);
        let w = key.get_round_keys().unwrap();

        let plaintext = [
            0x54, 0x77, 0x6F, 0x20, 0x4F, 0x6E, 0x65, 0x20, 0x4E, 0x69, 0x6E, 0x65, 0x20, 0x54,
            0x77, 0x6F,
        ];

        let correct_ciphertext = [
            0x29, 0xC3, 0x50, 0x5F, 0x57, 0x14, 0x20, 0xF6, 0x40, 0x22, 0x99, 0xB3, 0x1A, 0x02,
            0xD7, 0x3A,
        ];

        assert_eq!(super::cipher(plaintext, 10, w), correct_ciphertext);
    }

    #[test]
    fn inv_cipher() {
        let key = Key::from([
            0x54, 0x68, 0x61, 0x74, 0x73, 0x20, 0x6D, 0x79, 0x20, 0x4B, 0x75, 0x6E, 0x67, 0x20,
            0x46, 0x75,
        ]);
        let w = key.get_round_keys().unwrap();

        let ciphertext = [
            0x29, 0xC3, 0x50, 0x5F, 0x57, 0x14, 0x20, 0xF6, 0x40, 0x22, 0x99, 0xB3, 0x1A, 0x02,
            0xD7, 0x3A,
        ];

        let correct_plaintext = [
            0x54, 0x77, 0x6F, 0x20, 0x4F, 0x6E, 0x65, 0x20, 0x4E, 0x69, 0x6E, 0x65, 0x20, 0x54,
            0x77, 0x6F,
        ];

        assert_eq!(super::inv_cipher(ciphertext, 10, w), correct_plaintext);
    }
}
