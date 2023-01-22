use std::fmt::Display;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Nucleotide {
    #[default]
    Adenine = 0,
    Cytosine = 1,
    Guanine = 2,
    Thymine = 3,
}

pub use {
    Nucleotide::Adenine as A, Nucleotide::Cytosine as C, Nucleotide::Guanine as G,
    Nucleotide::Thymine as T,
};

impl Into<char> for Nucleotide {
    fn into(self) -> char {
        match self {
            A => 'A',
            C => 'C',
            G => 'G',
            T => 'T',
        }
    }
}

impl Into<char> for &Nucleotide {
    fn into(self) -> char {
        match self {
            &A => 'A',
            &C => 'C',
            &G => 'G',
            &T => 'T',
        }
    }
}

impl TryFrom<char> for Nucleotide {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a' | 'A' => Ok(A),
            'c' | 'C' => Ok(C),
            'g' | 'G' => Ok(G),
            't' | 'T' => Ok(T),
            _ => Err(()),
        }
    }
}

impl Display for Nucleotide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Nucleotide {
    /// Slice has to be exactly 4
    pub fn to_u8(item: &[Self]) -> u8 {
        debug_assert_eq!(item.len(), 4);
        let mut output = 0u8;
        for i in 0..3 {
            let n = item[i] as u8;
            output = output | n;
            output = output << 2;
        }
        // we don't bit shift the last one
        let n = item[3] as u8;
        output = output | n;
        output
    }

    pub fn from_u8(mut item: u8) -> [Nucleotide; 4] {
        //mask for the last 2 bits
        const MASK: u8 = 0b00000011;
        let mut output = [Nucleotide::default(); 4];
        for i in 0..4 {
            let n = match item & MASK {
                0 => A,
                1 => C,
                2 => G,
                3 => T,
                _ => unreachable!(),
            };
            output[3 - i] = n;
            item = item >> 2;
        }
        output
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const T_ZERO: [Nucleotide; 4] = [A, A, A, A];
    const T_ONE: [Nucleotide; 4] = [A, A, A, C];
    #[test]
    fn nucleotide_u8_convertion() {
        //0 case
        assert_eq!(Nucleotide::to_u8(&T_ZERO), 0u8);
        assert_eq!(Nucleotide::from_u8(0), T_ZERO);

        //1 case
        assert_eq!(Nucleotide::to_u8(&T_ONE), 1u8);
        assert_eq!(Nucleotide::from_u8(1), T_ONE);
    }
}
