extern crate numerobis_tournevis;

use numerobis_tournevis::numerologie::NumerologieCore;

fn main() {
    let core = NumerologieCore {
        year: 1984,
        month: 4,
        day: 1,
        first_name: "John".to_string(),
        second_name: "".to_string(),
        third_name: "".to_string(),
        last_name_1: "Doe".to_string(),
        last_name_2: "".to_string(),
        last_name_3: "".to_string(),
        tel: "".to_string(),
        mobile: "".to_string()
    };
    let calcul = core.calcul();
    let data = serde_json::to_string(&calcul).unwrap(); // TODO
    print!("{}",data);
}