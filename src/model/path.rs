use url::Url;
use percent_encoding::percent_decode;

#[derive(Debug)]
pub struct Path(String);

impl Path {
    pub fn from_url(url: &Url) -> Path {
        let path = get_url_without_origin(&url);
        let path = percent_decode(path.as_bytes()).decode_utf8_lossy();
        if path.is_empty() {
            return Path("/".into());
        } else {
            return Path(path.into());
        }
    }
    pub fn as_str(&self) -> &str {
        return &self.0;
    }
}

fn get_url_without_origin(url: &Url) -> &str {
    let origin = url.origin();
    let url = url.as_str();
    let unicode_origin = origin.unicode_serialization();
    let ascii_origin = origin.ascii_serialization();
    if url.starts_with(&unicode_origin) && unicode_origin.len() >= 1 {
        return &url[unicode_origin.len()..];
    }
    if url.starts_with(&ascii_origin) && ascii_origin.len() >= 1 {
        return &url[ascii_origin.len()..];
    }
    // Must never be executed.
    panic!("Unable to get path from url");
}
