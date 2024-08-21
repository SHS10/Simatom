use crate::atom::Atom;
use crate::bond::Bond;

pub struct Molecule {
    pub atoms: Vec<Atom>,
    pub bonds: Vec<Bond>,
}

impl Molecule {
    pub fn new() -> Molecule {
        Molecule {
            atoms: Vec::new(),
            bonds: Vec::new(),
        }
    }

    pub fn add_atom(&mut self, atom: Atom) {
        self.atoms.push(atom);
    }

    pub fn add_bond(&mut self, bond: Bond) {
        self.bonds.push(bond);
    }
}
