use crate::dna::DNA;

pub trait Genetic {
    fn to_dna(&self) -> DNA;
    fn from_dna(dna: DNA) -> Self;
}

pub trait Sexual: Genetic {
    fn bread(&self, other: &Self) -> DNA{
        let dna0 = self.to_dna();
        let dna1 = other.to_dna();
        dna0.crossover(&dna1)
    }
}