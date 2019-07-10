
use crate::cms;
pub fn get_slash_complete_path(path: &str) -> String {
    if path.ends_with("/") {
        path.to_string()
    } else {
        format!("{}/", path)
    }
}

pub fn magic(req: &realm::Request) -> realm::Result {
    let url_ = req.uri();
    println!("uri {}", url_);
    let path = get_slash_complete_path(url_.path());
    match path.as_ref() {
        "/" => crate::routes::index::layout(req, 0),
        url_ => cms::serve(req,url_),
    }
}

