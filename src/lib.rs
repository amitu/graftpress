#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate failure;

extern crate url;

#[macro_use]
extern crate serde_json;
extern crate itertools;
extern crate chrono;


pub mod middleware;
mod forward;
mod pages;
mod reverse;
mod routes;
mod widgets;
mod cms;
<<<<<<< HEAD
pub mod middleware;
=======
mod in_;

>>>>>>> 8fe523b4da4067ef58c5c4e4a00123791facfda0


pub type Result<T> = std::result::Result<T, failure::Error>;
