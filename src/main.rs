mod aquarium;

use legion::*;
use legion::world::SubWorld;
use crate::aquarium::*;


#[system]
#[read_component(Algue)]
#[read_component(Poisson)]
fn show_aquarium(p_world: &SubWorld) {
    let mut algues_query = <&Algue>::query();
    println!("Algues : {:?}", algues_query.iter(p_world).count());

    println!("Poissons :");
    let mut poissons_query = <&Poisson>::query();
    for poisson in poissons_query.iter(p_world) {
        println!("  - {:?} {:?}", poisson.nom, poisson.sexe);
    }
}


/**
 * Création des entités et composants.
 */
fn load_entities(p_world: &mut World) {
    p_world.extend(vec![
        init_algue(),
        init_algue(),
        init_algue(),
        init_algue(),
        init_algue()
    ]);
    p_world.extend(vec![
        init_poisson("Pierre Tramo"      , Sexe::Male   ),
        init_poisson("Jean Sérien"       , Sexe::Male   ),
        init_poisson("Aline Néhat"       , Sexe::Femelle),
        init_poisson("Hildegarde Laporte", Sexe::Femelle)
    ]);
}


/**
 * Fonction principale.
 */
fn main() {
    let mut world = World::default();

	// Création des entités & composants
    load_entities(&mut world);

    // Création des ressources
    let mut resources = Resources::default();
    
    // Organisation des systèmes
    let mut scheduler = Schedule::builder()
        .add_system(show_aquarium_system())
        .build();
    
    // Boucle d'exécution
    for n in 1..11 {
        println!("===== Tour {:?} =====", n);
        scheduler.execute(&mut world, &mut resources);
    }
}
