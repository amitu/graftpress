#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate failure;


pub mod forward;
mod pages;
mod reverse;
mod routes;
mod widgets;
mod cms;

pub type Result<T> = std::result::Result<T, failure::Error>;
