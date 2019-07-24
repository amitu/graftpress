#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate failure;

extern crate url;

#[macro_use]
extern crate serde_json;
extern crate itertools;
extern crate chrono;


pub mod forward;
mod cms;
mod reverse;




pub type Result<T> = std::result::Result<T, failure::Error>;
