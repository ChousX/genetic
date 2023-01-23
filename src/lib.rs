mod dna;
mod genetic;
mod nucleotide;
pub use nucleotide::*;
pub mod prelude{
    pub use crate::nucleotide::*;
    pub use crate::genetic::*;
    pub use crate::dna::*;
}
