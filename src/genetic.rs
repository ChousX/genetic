use crate::dna::DNA;

pub trait Genetic {
    fn to_dna(&self) -> DNA;
    fn from_dna(dna: DNA) -> Self;
}

pub trait Asexual: Genetic {
    fn bread(&self) -> DNA;
}

pub trait Sexual: Genetic {
    fn bread(&self, other: &Self) -> DNA;
}
