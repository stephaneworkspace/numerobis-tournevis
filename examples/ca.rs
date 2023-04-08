extern crate numerobis_tournevis;

use numerobis_tournevis::numerologie::numerologie::CycleAdjacentType::Formatif;
use numerobis_tournevis::numerologie::NumerologieCore;

fn main() {
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
    print!("{}",data);
}