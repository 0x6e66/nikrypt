pub struct W<T>(pub T);

impl<const N: usize> std::ops::BitXor<W<[u8; N]>> for W<[u8; N]> {
    type Output = W<[u8; N]>;

    fn bitxor(mut self, rhs: W<[u8; N]>) -> Self::Output {
        for i in 0..N {
            self.0[i] ^= rhs.0[i];
        }

        self
    }
}
