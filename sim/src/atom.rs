pub struct Atom {
    /// Struct representing a single atom in 3D space.
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub element: String,
}

impl Atom {
    /// Instantiate a new atom into the 3D space
    pub fn new(x: f64, y: f64, z: f64, element: &str) -> Atom {
        Atom {
            x,
            y,
            z,
            element: String::from(element),
        }
    }

    /// method instantiating the atom to origin
    pub fn init_def_coords(element: &str) -> Atom {
        Atom {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            element: String::from(element),
        }
    }
}
