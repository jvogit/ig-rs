pub fn get_set_cookie_value(response: &reqwest::Response, cookie_name: &str) -> Option<String> {
    response
        .headers()
        .get_all(reqwest::header::SET_COOKIE)
        .iter()
        .map(|hv| cookie::Cookie::parse(hv.to_str().unwrap()).unwrap())
        .find(|cookie| cookie.name() == cookie_name)
        .map(|cookie| cookie.value().to_string())
}
