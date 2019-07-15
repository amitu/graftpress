use crate::routes;
use realm::utils::{get_slash_complete_path, get, sub_string};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use url::Url;


pub fn magic(req: &realm::Request) -> realm::Result {
    let url = req.uri();
    let site_url = "127.0.0.1:3000/".to_string();
    let path = get_slash_complete_path(url.path());
    let url = Url::parse(&format!("{}{}", &site_url, req.uri()).as_str())?;
    let mut rest = sub_string(path.as_ref(), path.len(), None);
    let data_: JsonValue  = serde_json::from_slice(req.body().as_slice())?;
    let query_: HashMap<_, _> = url.query_pairs().into_owned().collect();

    match path.as_ref() {
        "/" => {
            let i = get("i", &query_, &data_, &mut rest, false)?;
            routes::index::layout(req, i)
        },
        _ => unimplemented!()
    }
}