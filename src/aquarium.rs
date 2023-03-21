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


