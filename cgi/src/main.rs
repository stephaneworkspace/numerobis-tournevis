extern crate cgi;
extern crate numerobis_tournevis;

use numerobis_tournevis::numerologie::NumerologieCore;

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
    let mut year = 1984;
    let mut month = 4;
    let mut day = 1;
    let mut first_name: String = "".to_string();
    let mut second_name: String = "".to_string();
    let mut third_name: String = "".to_string();
    let mut last_name_1: String= "".to_string();
    let mut last_name_2: String= "".to_string();
    let mut last_name_3: String= "".to_string();
    let mut tel: String= "".to_string();
    let mut mobile: String= "".to_string();
    for x in params.iter() {
        match x.0.as_str() {
        "year" => {
           year = x.1.parse().unwrap_or(1984);
        }
        "month" => {
           month = x.1.parse().unwrap_or(4);
        },
        "day" => {
            day = x.1.parse().unwrap_or(1);
        },
        "first_name" => {
            let s = &html_escape::decode_html_entities(&x.1);
            first_name.push_str(&s);
        },
        "second_name" => {
            let s = &html_escape::decode_html_entities(&x.1);
            second_name.push_str(&s);
        },
        "third_name" => {
            let s = &html_escape::decode_html_entities(&x.1);
            third_name.push_str(&s);
        },
        "last_name_1" => {
            let s = &html_escape::decode_html_entities(&x.1);
            last_name_1.push_str(&s);
        },
        "last_name_2" => {
            let s = &html_escape::decode_html_entities(&x.1);
            last_name_2.push_str(&s);
        },
        "last_name_3" => {
            let s = &html_escape::decode_html_entities(&x.1);
            last_name_3.push_str(&s);
        },
        "tel" => {
            let s = &html_escape::decode_html_entities(&x.1);
            tel.push_str(&s);
        },
        "mobile" => {
            let s = &html_escape::decode_html_entities(&x.1);
            mobile.push_str(&s);
        },
        _ => {}
       }
    }
    let core = NumerologieCore {
        year,
        month,
        day,
        first_name: first_name.to_string(),
        second_name: second_name.to_string(),
        third_name: third_name.to_string(),
        last_name_1: last_name_1.to_string(),
        last_name_2: last_name_2.to_string(),
        last_name_3: last_name_3.to_string(),
        tel: "".to_string(),
        mobile: "".to_string()
    };
    let calcul = core.calcul();
    let data = serde_json::to_string(&calcul).unwrap(); // TODO
    cgi::binary_response(200, "application/javascript", Vec::from(data.as_bytes()))
})}