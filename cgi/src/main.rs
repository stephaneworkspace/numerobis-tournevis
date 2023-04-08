extern crate cgi;
extern crate numerobis_tournevis;

use numerobis_tournevis::numerologie::numerologie::CycleAdjacentType::Formatif;
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

    let re = format!("{:?}", params);
    let core = NumerologieCore {
        year: 1984,
        month: 4,
        day: 1,
        bsFirstName: "John".to_string(),
        bsSecondName: "".to_string(),
        bsThirdName: "".to_string(),
        bsLastName1: "Doe".to_string(),
        bsLastName2: "".to_string(),
        bsLastName3: "".to_string(),
        bsTel: "".to_string(),
        bsMobile: "".to_string()
    };
    let cycles_adjacents = core.cycles_adjacents();
    let data = serde_json::to_string(&cycles_adjacents).unwrap(); // TODO
    cgi::binary_response(200, "application/javascript", Vec::from(data.as_bytes()))
})}