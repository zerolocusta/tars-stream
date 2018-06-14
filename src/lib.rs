#![feature(try_from)]
#![feature(core_intrinsics)]
#![feature(extern_prelude)]
#![feature(specialization)]
extern crate bytes;

#[macro_use]
extern crate quick_error;

pub mod errors;

pub mod tars_type;

pub mod tars_decoder;
pub mod tars_encoder;

pub mod tup_decoder;
pub mod tup_encoder;

pub mod prelude {
    pub use errors::*;
    pub use tars_decoder::*;
    pub use tars_encoder::{EncodeIntoTars, TarsEncoder, TarsEncoderTrait};
    pub use tars_type::*;
}
