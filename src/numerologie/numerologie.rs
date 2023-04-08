use serde::{Serialize};

/// NumerologieCore
#[derive(Debug, Clone)]
pub struct NumerologieCore {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub bsFirstName: String,
    pub bsSecondName: String,
    pub bsThirdName: String,
    pub bsLastName1: String,
    pub bsLastName2: String,
    pub bsLastName3: String,
    pub bsTel: String,
    pub bsMobile: String
}

impl NumerologieCore {
    pub fn cycles_adjacents(&self) -> Vec<CycleAdjacent> {
        use CycleAdjacentType::*;
        let mut vec: Vec<CycleAdjacent> = Vec::new();
        let ca = NumerologieCore::cycle_adjacent(self, Formatif);
        vec.push(ca);
        let ca = NumerologieCore::cycle_adjacent(self, Productif);
        vec.push(ca);
        let ca = NumerologieCore::cycle_adjacent(self, Moisson);
        vec.push(ca);
        vec
    }

    pub fn cycle_adjacent(&self, cycle_type: CycleAdjacentType) -> CycleAdjacent {
        use CycleAdjacentType::*;
        let calcul = match cycle_type {
            Formatif => {
                self.month
            },
            Productif => {
                self.day
            },
            Moisson => {
                self.year
            }
        };
        let calcul_string = match cycle_type {
            Formatif => {
                "Mois"
            },
            Productif => {
                "Jour"
            },
            Moisson => {
                "Année"
            }
        };
        CycleAdjacent {
            cycle: cycle_type,
            calcul: calcul_string.to_string(),
            nombre: NumerologieCore::reduction(calcul)
        }
    }

    fn reduction(nombre: i32) -> Vec<i32> {
        let mut ai_res : Vec<i32> = Vec::new();
        let mut i_temp: i32 = nombre;
        let mut b_done: bool = false;
        while !b_done {
            ai_res.push(i_temp);
            if i_temp.to_string().chars().count() == 1 {
                b_done = true;
            }
            let mut i_temp_2: i32 = 0;
            let t = i_temp.to_string();
            for (i, _) in t.chars().enumerate() {
                let s_temp: String = i_temp.to_string();
                let c_temp = s_temp.chars().nth(i).unwrap();
                let c_temp_string: String = String::from(c_temp);
                i_temp_2 += c_temp_string.parse().unwrap_or(0);
            }
            i_temp = i_temp_2
        }
        return ai_res
    }
}

#[derive(Serialize)]
pub struct CycleAdjacent {
    pub cycle: CycleAdjacentType,
    pub calcul: String,
    pub nombre: Vec<i32>
}

#[derive(Serialize)]
pub enum CycleAdjacentType {
    Formatif,
    Productif,
    Moisson
}

pub enum Colonne {
    Gauche,
    Droite,
    Nombre,
    Rien,
}

pub fn colonne(lettre: &str) -> Colonne {
    use Colonne::*;
    let l: &str = &lettre.to_uppercase();
    match l {
        "A" =>
            Droite,
        "À" =>
            Droite,
        "Á" =>
            Droite,
        "Â" =>
            Droite,
        "Ã" =>
            Droite,
        "B" =>
            Gauche,
        "C" =>
            Gauche,
        "D" =>
            Gauche,
        "E" =>
            Droite,
        "È" =>
            Droite,
        "É" =>
            Droite,
        "Ê" =>
            Droite,
        "Ë" =>
            Droite,
        "F" =>
            Gauche,
        "G" =>
            Gauche,
        "H" =>
            Gauche,
        "I" =>
            Droite,
        "Í" =>
            Droite,
        "Ì" =>
            Droite,
        "Î" =>
            Droite,
        "Ï" =>
            Droite,
        "J" =>
            Gauche,
        "K" =>
            Gauche,
        "L" =>
            Gauche,
        "M" =>
            Gauche,
        "N" =>
            Gauche,
        "Ñ" =>
            Gauche,
        "O" =>
            Droite,
        "Ó" =>
            Droite,
        "Ò" =>
            Droite,
        "Ô" =>
            Droite,
        "Ö" =>
            Droite,
        "Ø" =>
            Droite,
        "Õ" =>
            Droite,
        "P" =>
            Gauche,
        "Q" =>
            Gauche,
        "R" =>
            Gauche,
        "S" =>
            Gauche,
        "T" =>
            Gauche,
        "U" =>
            Droite,
        "Ú" =>
            Droite,
        "Ù" =>
            Droite,
        "Û" =>
            Droite,
        "Ü" =>
            Droite,
        "V" =>
            Gauche,
        "W" =>
            Gauche,
        "X" =>
            Gauche,
        "Y" =>
            Droite,
        "Ý" =>
            Droite,
        "Z" =>
            Gauche,
        "0" =>
            Rien,
        "1" =>
            Nombre,
        "2" =>
            Nombre,
        "3" =>
            Nombre,
        "4" =>
            Nombre,
        "5" =>
            Nombre,
        "6" =>
            Nombre,
        "7" =>
            Nombre,
        "8" =>
            Nombre,
        "9" =>
            Nombre,
        _ =>
            Rien,
    }
}

pub fn lettre_simple(lettre: &str) -> i32 {
    let l: &str = &lettre.to_uppercase();
    match l {
        "A" =>
            1,
        "À" =>
            1,
        "Á" =>
            1,
        "Â" =>
            1,
        "Ã" =>
            1,
        "B" =>
            2,
        "C" =>
            3,
        "D" =>
            4,
        "E" =>
            5,
        "È" =>
            5,
        "É" =>
            5,
        "Ê" =>
            5,
        "Ë" =>
            5,
        "F" =>
            6,
        "G" =>
            7,
        "H" =>
            8,
        "I" =>
            9,
        "Í" =>
            9,
        "Ì" =>
            9,
        "Î" =>
            9,
        "Ï" =>
            9,
        "J" =>
            10,
        "K" =>
            11,
        "L" =>
            12,
        "M" =>
            13,
        "N" =>
            14,
        "Ñ" =>
            14,
        "O" =>
            15,
        "Ó" =>
            15,
        "Ò" =>
            15,
        "Ô" =>
            15,
        "Ö" =>
            15,
        "Ø" =>
            15,
        "Õ" =>
            15,
        "P" =>
            16,
        "Q" =>
            17,
        "R" =>
            18,
        "S" =>
            19,
        "T" =>
            20,
        "U" =>
            21,
        "Ú" =>
            21,
        "Ù" =>
            21,
        "Û" =>
            21,
        "Ü" =>
            21,
        "V" =>
            22,
        "W" =>
            23,
        "X" =>
            24,
        "Y" =>
            25,
        "Ý" =>
            25,
        "Z" =>
            26 ,
        "0" =>
            0,
        "1" =>
            1,
        "2" =>
            2,
        "3" =>
            3,
        "4" =>
            4,
        "5" =>
            5,
        "6" =>
            6,
        "7" =>
            7,
        "8" =>
            8,
        "9" =>
            9,
        _ =>
            0
    }
}

pub fn lettre_colonne(lettre: &str) -> (i32, Colonne) {
    let colonne: Colonne = colonne(&lettre);
    let nombre: i32 = lettre_simple(&lettre);
    (nombre, colonne)
}

pub fn str_vers_nombre(str: &str) -> i32 {
    let mut nombre = 0;
    for s in str.split_whitespace() {
        let n = lettre_simple(s);
        nombre += n;
    }
    nombre
}
