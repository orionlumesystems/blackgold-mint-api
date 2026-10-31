pub mod consts;
pub mod error;
pub mod instruction;
pub mod sdk;
pub mod state;

pub mod prelude {
    pub use crate::consts::*;
    pub use crate::error::*;
    pub use crate::instruction::*;
    pub use crate::sdk::*;
    pub use crate::state::*;
}

use steel::*;

//* Program ID
declare_id!("mnt6SKo4zUvnG7ExPuh77EYxrVF65Stud4tPBH6dsk7");
