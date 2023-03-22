// ===== Algues =====
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Algue;


// ===== Poissons =====
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Sexe {
    Male,
    Femelle
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Poisson {
    pub nom: &'static str,
    pub sexe: Sexe
}


pub fn init_algue() -> (Algue, ) {
    (Algue {}, )
}

pub fn init_poisson(p_nom: &'static str, p_sexe: Sexe) -> (Poisson, ) {
    (Poisson { nom: p_nom, sexe: p_sexe }, )
}
