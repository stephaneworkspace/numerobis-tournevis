/// NumerologieCore
#[derive(Debug, Clone)]
pub struct NumerologieCore {
    pub ws: i32,
}

pub enum Colonne {
    Gauche,
    Droite,
    Nombre,
    Rien,
}

pub fn colonne(lettre: &str) -> Colonne {
    use Colonne::*;
    let l: &str = lettre.to_uppercase();
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
    use Colonne::*;
    let l: &str = lettre.to_uppercase();
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
/*pub fn charsToInt(chars: &str) -> i32 {
    let nbr = 0;
    for l in chars {
        let n: Int = self::NumerologieCore::
        nombre = n + nombre;
    }
    nbr
}*/