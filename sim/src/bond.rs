/// Struct for defining atomic bonds

pub struct Bond {
    pub atom1: usize,
    pub atom2: usize,
    pub strength: f64,
}

impl Bond {
    pub fn new(atom1: usize, atom2: usize, strength: f64) -> Bond {
        Bond {
            atom1,
            atom2,
            strength,
        }
    }
}
