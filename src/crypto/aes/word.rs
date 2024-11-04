use std::ops::{Index, IndexMut};

use super::utils::{inv_sbox, sbox};

#[derive(Clone, Copy, Debug)]
pub struct Word(pub [u8; 4]);

impl Word {
    pub fn rot(&self) -> Self {
        Word([self.0[1], self.0[2], self.0[3], self.0[0]])
    }

    pub fn inv_rot(&self) -> Self {
        Word([self.0[3], self.0[0], self.0[1], self.0[2]])
    }

    pub fn sub(&self) -> Self {
        Word([
            sbox(self.0[0]),
            sbox(self.0[1]),
            sbox(self.0[2]),
            sbox(self.0[3]),
        ])
    }

    pub fn inv_sub(&self) -> Self {
        Word([
            inv_sbox(self.0[0]),
            inv_sbox(self.0[1]),
            inv_sbox(self.0[2]),
            inv_sbox(self.0[3]),
        ])
    }

    pub fn get(&self, i: usize) -> Option<u8> {
        match i {
            0..=3 => Some(self.0[i]),
            _ => None,
        }
    }
}

impl std::ops::BitXor for Word {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Word([
            self.0[0] ^ rhs.0[0],
            self.0[1] ^ rhs.0[1],
            self.0[2] ^ rhs.0[2],
            self.0[3] ^ rhs.0[3],
        ])
    }
}

impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..4 {
            if self.0[i] != other.0[i] {
                return false;
            }
        }
        true
    }
}

impl Index<&'_ usize> for Word {
    type Output = u8;

    fn index(&self, s: &usize) -> &u8 {
        match s {
            0..=3 => &self.0[*s],
            _ => panic!("unknown field: {}", s),
        }
    }
}

impl IndexMut<&'_ usize> for Word {
    fn index_mut(&mut self, s: &usize) -> &mut u8 {
        match s {
            0..=3 => &mut self.0[*s],
            _ => panic!("unknown field: {}", s),
        }
    }
}
