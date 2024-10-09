use url::Url;

pub fn get_url_root(url_str: &str) -> Option<String> {
    let url = Url::parse(url_str).ok()?;
    let scheme = url.scheme().to_string();
    let host = url.host().unwrap().to_owned();
    Some(scheme + "://" + &host.to_string())
}