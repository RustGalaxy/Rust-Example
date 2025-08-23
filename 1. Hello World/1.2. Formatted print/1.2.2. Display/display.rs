// Display

// fmt::Debug hardly looks compact and clean, so it is often advantageous to customize the output appearance. This is done by manually implementing fmt::Display, which uses the {} print marker. Implementing it looks like this:

// Import (via `use`) the `fmt` module to mmake it available.
use std::fmt;

// Define a structure for which `fmt::Display` will be implemented.
struct Structure {
    field: i32
}

// Implement `fmt::Display` for `Structure`.
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Structure with field = {}", self.field)
    }
}

