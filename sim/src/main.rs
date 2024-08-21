mod atom;
mod bond;
mod molecule;

use atom::Atom;
use bond::Bond;
use molecule::Molecule;

fn main() {
    // molecule collection
    let mut molecule = Molecule::new();

    // atoms for water
    let oxygen = Atom::init_def_coords("0");
    let hydrogen1 = Atom::new(1.0, 0.0, 0.0, "H");
    let hydrogen2 = Atom::new(-1.0, 0.0, 0.0, "H");

    // 'create' the H20  by pushing to the molecule
    molecule.add_atom(oxygen);
    molecule.add_atom(hydrogen1);
    molecule.add_atom(hydrogen2);

    // define the bonds between the atoms
    let bond1 = Bond::new(0, 1, 1.0);
    let bond2 = Bond::new(0, 2, 1.0);

    // add bonds to the molecules
    molecule.add_bond(bond1);
    molecule.add_bond(bond2);

    println!(
        "Molecule has {} atoms and {} bonds",
        molecule.atoms.len(),
        molecule.bonds.len()
    );
}
