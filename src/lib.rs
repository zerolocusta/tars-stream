#![feature(try_from)]
#![feature(core_intrinsics)]
#![feature(extern_prelude)]
#![feature(specialization)]
extern crate bytes;

#[macro_use]
extern crate quick_error;

#[macro_use]
extern crate assert_approx_eq;

pub mod errors;
pub mod tars_decoder;
pub mod tars_encoder;
pub mod tars_type;

pub mod prelude {
    pub use errors::*;
    pub use tars_decoder::{DecodeFrom, TarsDecoder};
    pub use tars_encoder::{write_struct, EncodeTo};
}
