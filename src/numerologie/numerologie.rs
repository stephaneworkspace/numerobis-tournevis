use serde::{Serialize};
use chrono::Datelike;

/// NumerologieCore
#[derive(Debug, Clone)]
pub struct NumerologieCore {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub first_name: String,
    pub second_name: String,
    pub third_name: String,
    pub last_name_1: String,
    pub last_name_2: String,
    pub last_name_3: String,
    pub tel: String,
    pub mobile: String
}

impl NumerologieCore {
    pub fn calcul(&self) -> Calcul {
        Calcul {
            chemin_de_vie: self.chemin_de_vie(),
            annee_personelle: self.annee_personelle(),
            cycles_adjacents: self.cycles_adjacents(),
            cycles_realisations: self.cycles_realisations()
        }
    }

    fn chemin_de_vie(&self) -> Vec<i32> {
        NumerologieCore::reduction(self.year + self.month + self.day)
    }

    fn annee_personelle(&self) -> AnneePersonnelle {
        let current_date = chrono::Utc::now().date_naive();
        let annee = current_date.year();
        let temp = annee + self.month + self.day;
        let reduction = NumerologieCore::reduction(temp);
        match reduction.last() {
            Some(n) => {
                AnneePersonnelle {
                    annee,
                    nombre: *n
                }
            },
            None => {
                AnneePersonnelle {
                    annee,
                    nombre: 0
                }
            }
        }
    }

    fn cycles_adjacents(&self) -> Vec<CycleAdjacent> {
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

    fn cycle_adjacent(&self, cycle_type: CycleAdjacentType) -> CycleAdjacent {
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

    fn cycles_realisations(&self) -> Vec<CycleRealisation> {
        use CycleRealisationType::*;
        let mut vec: Vec<CycleRealisation> = Vec::new();
        let ca = NumerologieCore::cycle_realisation(self, JourPlusMois);
        vec.push(ca);
        let ca = NumerologieCore::cycle_realisation(self, JourPlusAnnee);
        vec.push(ca);
        let ca = NumerologieCore::cycle_realisation(self, UnPlusDeux);
        vec.push(ca);
        let ca = NumerologieCore::cycle_realisation(self, MoisPlusAnnee);
        vec.push(ca);
        vec
    }

    fn cycle_realisation(&self, cycle_type: CycleRealisationType) -> CycleRealisation {
        use CycleRealisationType::*;
        let calcul = match cycle_type {
            JourPlusMois => {
                self.day + self.month
            },
            JourPlusAnnee => {
                self.day + self.year
            },
            UnPlusDeux => {
                (self.day + self.month) + (self.day + self.year)
            },
            MoisPlusAnnee => {
                self.month + self.year
            }
        };
        let calcul_string = match cycle_type {
            JourPlusMois => {
                "Jour plus mois"
            },
            JourPlusAnnee => {
                "Jour plus année"
            },
            UnPlusDeux => {
                "Un plus deux"
            },
            MoisPlusAnnee => {
                "Mois plus année"
            }
        };
        let chemin_de_vie = NumerologieCore::reduction(self.day + self.month + self.year);
        let age = match chemin_de_vie.last() {
            Some(cdv) => {
                match cdv {
                    1 => {
                        match &cycle_type {
                            JourPlusMois => {
                                "0 à 35 ans"
                            }
                            JourPlusAnnee => {
                                "35 à 44 ans"
                            }
                            UnPlusDeux => {
                                "44 à 53 ans"
                            }
                            MoisPlusAnnee => {
                                "dès 54 ans"
                            }
                        }
                    },
                    2 => {
                        match &cycle_type {
                            JourPlusMois => {
                                "0 à 34 ans"
                            }
                            JourPlusAnnee => {
                                "34 à 43 ans"
                            }
                            UnPlusDeux => {
                                "43 à 52 ans"
                            }
                            MoisPlusAnnee => {
                                "dès 53 ans"
                            }
                        }
                    },
                    3 => {
                        match &cycle_type {
                            JourPlusMois => {
                                "0 à 33 ans"
                            }
                            JourPlusAnnee => {
                                "33 à 42 ans"
                            }
                            UnPlusDeux => {
                                "42 à 51 ans"
                            }
                            MoisPlusAnnee => {
                                "dès 52 ans"
                            }
                        }
                    },
                    4 => {
                        match &cycle_type {
                            JourPlusMois => {
                                "0 à 32 ans"
                            }
                            JourPlusAnnee => {
                                "32 à 41 ans"
                            }
                            UnPlusDeux => {
                                "41 à 50 ans"
                            }
                            MoisPlusAnnee => {
                                "dès 51 ans"
                            }
                        }
                    },
                    5 => {
                        match &cycle_type {
                            JourPlusMois => {
                                "0 à 31 ans"
                            }
                            JourPlusAnnee => {
                                "31 à 40 ans"
                            }
                            UnPlusDeux => {
                                "40 à 49 ans"
                            }
                            MoisPlusAnnee => {
                                "dès 50 ans"
                            }
                        }
                    },
                    6 => {
                        match &cycle_type {
                            JourPlusMois => {
                                "0 à 30 ans"
                            }
                            JourPlusAnnee => {
                                "30 à 39 ans"
                            }
                            UnPlusDeux => {
                                "39 à 48 ans"
                            }
                            MoisPlusAnnee => {
                                "dès 49 ans"
                            }
                        }
                    },
                    7 => {
                        match &cycle_type {
                            JourPlusMois => {
                                "0 à 29 ans"
                            }
                            JourPlusAnnee => {
                                "29 à 38 ans"
                            }
                            UnPlusDeux => {
                                "38 à 47 ans"
                            }
                            MoisPlusAnnee => {
                                "dès 48 ans"
                            }
                        }
                    },
                    8 => {
                        match &cycle_type {
                            JourPlusMois => {
                                "0 à 28 ans"
                            }
                            JourPlusAnnee => {
                                "28 à 37 ans"
                            }
                            UnPlusDeux => {
                                "37 à 46 ans"
                            }
                            MoisPlusAnnee => {
                                "dès 47 ans"
                            }
                        }
                    },
                    9 => {
                        match &cycle_type {
                            JourPlusMois => {
                                "0 à 27 ans"
                            }
                            JourPlusAnnee => {
                                "27 à 36 ans"
                            }
                            UnPlusDeux => {
                                "36 à 45 ans"
                            }
                            MoisPlusAnnee => {
                                "dès 46 ans"
                            }
                        }
                    },
                    _ => {
                        "?"
                    }
                }
            },
            None => {
                "?"
            }
        };
        CycleRealisation {
            cycle: cycle_type,
            calcul: calcul_string.to_string(),
            age: age.to_string(),
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
pub struct Calcul {
    pub chemin_de_vie: Vec<i32>,
    pub annee_personelle: AnneePersonnelle,
    pub cycles_adjacents: Vec<CycleAdjacent>,
    pub cycles_realisations: Vec<CycleRealisation>
}

#[derive(Serialize)]
pub struct AnneePersonnelle {
    pub annee: i32,
    pub nombre: i32,
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

#[derive(Serialize)]
pub struct CycleRealisation {
    pub cycle: CycleRealisationType,
    pub age: String,
    pub calcul: String,
    pub nombre: Vec<i32>
}

#[derive(Serialize)]
pub enum CycleRealisationType {
    JourPlusMois,
    JourPlusAnnee,
    UnPlusDeux,
    MoisPlusAnnee,
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
