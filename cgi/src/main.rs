extern crate cgi;

fn main() { cgi::handle(|request: cgi::Request| -> cgi::Response {
    let request_str = format!("{:?}", &request);
    let mut r: String = "".to_string();
    let mut sw_found = false;
    for x in request_str.split(" ") {
        if sw_found {
            r = x.to_string();
            break;
        }
        if "\"x-cgi-query-string\":" == x {
            sw_found = true;
        }
    }
    r.pop();      // remove last
    r.pop();
    if r.len() > 0 {
        r.remove(0);  // remove first
    }
    let mut params: Vec<(String, String)> = Vec::new();
    for x in r.split("&") {
        let mut a: String = "".to_string();
        let mut b: String = "".to_string();
        for (i, y) in x.split("=").enumerate() {
            if i == 0 {
                a = y.to_string();
            } else if i == 1 {
                b = y.to_string();
            }
        }
        params.push((a, b));
    }

    let re = format!("{:?}", params);
    // cgi::text_response(200, re)
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
    cgi::binary_response(200, "application/javascript", Vec::from(data.as_bytes()))
})}