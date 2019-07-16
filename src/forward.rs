pub fn magic(req: realm::Request) -> realm::Result {
    let req_conf = realm::RequestConfig::new(req)?;
    match realm::utils::get_slash_complete_path(req.uri().path()).as_ref() {
        "/" => crate::routes::index::layout(&req_conf.req),
        url_ => crate::cms::layout(&req_conf.req, crate::cms::get_context("cms"), url_),
        _ => unimplemented!()
    }
}

