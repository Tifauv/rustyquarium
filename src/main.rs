mod aquarium;

use legion::*;
use crate::aquarium::*;


fn main() {
    let mut world = World::default();

	// Load the entities
    world.extend(vec![
        (Algue {}, ),
        (Algue {}, ),
        (Algue {}, ),
        (Algue {}, ),
        (Algue {}, )
    ]);
    world.extend(vec![
        (Poisson { nom: "Pierre Tramo"      , sexe: Sexe::Male    }, ),
        (Poisson { nom: "Jean Sérien"       , sexe: Sexe::Male    }, ),
        (Poisson { nom: "Aline Néhat"       , sexe: Sexe::Femelle }, ),
        (Poisson { nom: "Hildegarde Laporte", sexe: Sexe::Femelle }, )
    ]);

    // Query the world
    let mut algues_query = <&Algue>::query();
    println!("Algues :{:?}", algues_query.iter(&world).count());

    println!("Poissons :");
    let mut poissons_query = <&Poisson>::query();
    for poisson in poissons_query.iter(&world) {
        println!("  - {:?} {:?}", poisson.nom, poisson.sexe);
    }
}
