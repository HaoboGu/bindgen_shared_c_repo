mod bindings {
    #![allow(
        clippy::fn_to_numeric_cast,
        clippy::missing_safety_doc,
        clippy::redundant_static_lifetimes,
        clippy::useless_transmute
    )]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;