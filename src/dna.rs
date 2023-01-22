use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use crate::{nucleotide, Nucleotide};

use rand::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DNA(Vec<Nucleotide>);

impl Deref for DNA {
    type Target = Vec<Nucleotide>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DNA {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Into<String> for &DNA {
    fn into(self) -> String {
        let mut string = String::with_capacity(self.capacity());
        for nucleotide in self.iter() {
            string.push(nucleotide.into())
        }
        string
    }
}

impl Into<String> for DNA {
    fn into(self) -> String {
        let mut string = String::with_capacity(self.capacity());
        for nucleotide in self.iter() {
            string.push(nucleotide.into())
        }
        string
    }
}

impl Display for DNA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.into();
        write!(f, "{}", s)
    }
}

impl From<Chromosome> for DNA {
    fn from(value: Chromosome) -> Self {
        // getting the amount of uncompressed Nucleotides
        let remaner = {
            let mut output = 0;
            for i in 0..3 {
                if value.remaner[i].is_some() {
                    output += 1;
                } else {
                    break;
                }
            }
            output
        };
        // alocate space
        let mut output = Vec::with_capacity(value.data.len() * 4 + remaner);

        //convert and add to output
        for u in value.data.into_iter() {
            let nucleotides = Nucleotide::from_u8(u);
            output.extend_from_slice(&nucleotides);
        }

        //move the remaner to the end of output
        for i in 0..remaner {
            output.push(value.remaner[i].unwrap())
        }

        Self(output)
    }
}

impl From<&[Nucleotide]> for DNA {
    fn from(value: &[Nucleotide]) -> Self {
        let mut output = Vec::with_capacity(value.len());
        for n in value {
            output.push(*n);
        }
        DNA(output)
    }
}

impl DNA {
    pub fn insertion(&mut self, segment: &[Nucleotide], mut pos: usize) {
        if pos >= self.len() {
            panic!("Out of bounds Error")
        }

        for n in segment {
            self.insert(pos, *n);
            pos += 1;
        }
    }

    pub fn deletion(&mut self, pos: usize, len: usize) {
        for _ in 0..len {
            self.remove(pos);
        }
    }

    pub fn point(&mut self, pos: usize, nucleotide: Nucleotide) {
        self[pos] = nucleotide;
    }

    pub fn inversion(&mut self, pos: usize, len: usize) {
        if pos + len >= self.len() {
            panic!("Out of bounds error");
        }
        let mut segment: DNA = self[pos..(pos + len)].into();
        segment.reverse();
        for i in 0..len {
            self[i + pos] = segment[i];
        }
    }

    pub fn crossover(&self, other: &Self) -> Self {
        let mut rng = thread_rng();
        let size = self.len();
        //for now I am assuming that they are the same size
        let mut new = Vec::with_capacity(size);
        let mut index = 0;
        for segment in segment_rand(size, 5, 2, &mut rng) {
            let me = rng.gen_bool(1.0 / 2.0);
            for i in 0..segment {
                new.push(if me { self } else { other }[index + i]);
            }
            index += segment;
        }
        DNA(new)
    }
}

fn segment_rand(
    mut len: usize,
    max_segment: usize,
    min_segment: usize,
    rng: &mut ThreadRng,
) -> Vec<usize> {
    let mut output = Vec::new();
    while len > 0 {
        let max = if max_segment < len { max_segment } else { len };

        let min = if len < min_segment { len } else { min_segment };

        let range = min..max;
        if range.is_empty() {
            output.push(1);
            len -= 1;
            continue;
        }

        let segment = rng.gen_range(range);
        len -= segment;
        output.push(segment);
    }
    output
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Chromosome {
    data: Vec<u8>,
    remaner: [Option<Nucleotide>; 3],
}

impl From<DNA> for Chromosome {
    fn from(value: DNA) -> Self {
        let mut output = Vec::with_capacity(value.len() / 4);
        let mut remaner: [Option<Nucleotide>; 3] = Default::default();
        let mod_len = value.len() % 4;
        if mod_len != 0 {
            let range = (value.len() - mod_len)..value.len();
            for (i, n) in value[range].into_iter().enumerate() {
                remaner[i] = Some(*n);
            }
        }
        let range = (0..(value.len() - 3)).step_by(4);
        for i in range {
            let slice = &value[(i..(i + 4))];
            output.push(Nucleotide::to_u8(slice));
        }
        Self {
            data: output,
            remaner,
        }
    }
}
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod test {
    use super::{
        super::{A, C, G, T},
        *,
    };

    const T_0_9: [Nucleotide; 9] = [A, A, A, A, A, A, A, A, A];
    const T_0_4: [Nucleotide; 4] = [A, A, A, A];
    const T_1_9: [Nucleotide; 9] = [A, A, A, A, A, A, A, A, C];
    const T_0_7: [Nucleotide; 7] = [A, A, A, A, A, A, A];
    const T_G_9: [Nucleotide; 9] = [G, G, G, G, G, G, G, G, G];
    const T_A_9: [Nucleotide; 9] = T_0_9;
    const T__0: [Nucleotide; 4] = [A, G, C, T];
    const T__1: [Nucleotide; 8] = [A, A, G, G, C, C, T, T];

    #[test]
    fn dna_chromosome_convertion_0_9() {
        let slise: &[Nucleotide] = &T_0_9[..];
        let dna: DNA = slise.into();
        let c: Chromosome = dna.clone().into();
        let c_dna: DNA = c.into();
        assert_eq!(c_dna, dna);
    }

    #[test]
    fn dna_chromosome_convertion_0_4() {
        let slise: &[Nucleotide] = &T_0_4[..];
        let dna: DNA = slise.into();
        let c: Chromosome = dna.clone().into();
        let c_dna: DNA = c.into();
        assert_eq!(c_dna, dna);
    }

    #[test]
    fn dna_chromosome_convertion_1_4() {
        let slise: &[Nucleotide] = &T_1_9[..];
        let dna: DNA = slise.into();
        let c: Chromosome = dna.clone().into();

        let c_dna: DNA = c.into();
        assert_eq!(c_dna, dna);
    }

    #[test]
    fn dna_chromosome_convertion_0_7() {
        let slise: &[Nucleotide] = &T_0_7[..];
        let dna: DNA = slise.into();
        let c: Chromosome = dna.clone().into();
        let c_dna: DNA = c.into();
        assert_eq!(c_dna, dna);
    }

    #[test]
    fn crossover() {
        let slise: &[Nucleotide] = &T_G_9[..];
        let dna0: DNA = slise.into();

        let slise: &[Nucleotide] = &T_A_9[..];
        let dna1: DNA = slise.into();

        let child = dna0.crossover(&dna1);

        assert_eq!(child.len(), 9)
    }

    #[test]
    fn inversion_0() {
        let slise: &[Nucleotide] = &T__0[..];
        let mut dna: DNA = slise.into();
        let save = dna.clone();
        dna.inversion(1, 2);
        // println!("{dna} != {save}");
        assert_ne!(dna, save);
        dna.inversion(1, 2);
        // println!("{dna} == {save}");

        assert_eq!(dna, save);
    }

    #[test]
    fn inversion_1() {
        let slise: &[Nucleotide] = &T__1[..];
        let mut dna: DNA = slise.into();
        let save = dna.clone();
        dna.inversion(1, 4);
        // println!("{dna} != {save}");
        assert_ne!(dna, save);
        dna.inversion(1, 4);
        // println!("{dna} == {save}");

        assert_eq!(dna, save);
    }

    #[test]
    fn insertion() {
        let slise: &[Nucleotide] = &T__1[..];
        let mut dna0: DNA = slise.into();

        let slise: &[Nucleotide] = &T__0[..];
        let dna1: DNA = slise.into();

        dna0.insertion(&dna1[..], 0);
        
    }
}
