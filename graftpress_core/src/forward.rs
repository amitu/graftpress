
pub fn magic(req: realm::Request) -> realm::Result {
    let input = realm::request_config::RequestConfig::new(req)?;
    match input.path.as_str() {
        url_ => crate::cms::layout(&input.req, crate::cms::get_context("cms"), url_),
    }
}

