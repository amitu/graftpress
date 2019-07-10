use failure::{self};
use graft::{self, Context, DirContext};
use serde_json;
use std::collections::HashMap;
use std::{env, fs::{File, canonicalize}, panic, path::PathBuf};
use std::io;
use std::io::Read;


fn try_cms(path: &str) -> crate::Result<String> {
    let mut proj_dir = env::current_dir().expect("could not find current dir");

    let src = proj_dir.join(path);
    println!("try_cms: {:?}", &src);
    let src = canonicalize(src)?;
    if !src.starts_with(&proj_dir) {
        return Err(failure::err_msg("error"));
    }
    let mut src = File::open(&src)?;
    let mut content = String::new();
    src.read_to_string(&mut content)?;
    Ok(content)
}


pub fn cms_content(path: &str) -> crate::Result<String> {
    let path: String = if path.ends_with("/") {
        path.chars().skip(1).take(path.len() - 2).collect()
    } else {
        path.to_owned()
    };
    match try_cms(&format!("cms/{}.graft", &path).as_str()) {
        Ok(content) => Ok(content),
        Err(_) => try_cms(&format!("cms/{}/index.graft", path).as_str()),
    }
}


#[derive(Debug, Deserialize, Clone)]
struct C {
    pub title: String,
    pub permission: String,
    pub widget: serde_json::Value,
}

pub fn serve( req: &realm::Request, url: &str) -> realm::Result {
    let content = match cms_content(url) {
        Ok(content) => content,
        Err(e) => {
            //return Ok(AckoResponse::Http404(url.to_owned()));
            return Err(e);
        }
    };

    //println!("content {:?}", content);
    let mut proj_dir = env::current_dir().expect("could not find current dir");
    let context =
        DirContext::new(proj_dir.join("cms/includes"));
    let c: C = serde_json::from_value(graft::convert(&content, &context)?)?;
    println!("c: {:#?}", &c);
    let spec = serde_json::from_value(c.widget)?;

    realm::HTML::new().title("index").render_to_response(spec)
}