<<<<<<< HEAD

pub fn magic(ireq: crate::middleware::Context) -> realm::Result {
=======
pub fn magic(ireq: crate::in_::In) -> realm::Result {
>>>>>>> 8fe523b4da4067ef58c5c4e4a00123791facfda0
    let req = ireq.realm_request;
    let input = realm::request_config::RequestConfig::new(req)?;
    match input.path.as_str() {
            url_ => crate::cms::layout(&input.req, crate::cms::get_context("cms"), url_),
        "/" => crate::routes::index::layout(&input.req,),
        _ => unimplemented!()
    }
}

