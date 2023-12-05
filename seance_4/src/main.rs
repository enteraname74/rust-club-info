use std::io;
use std::io::Write;

#[derive(Debug)]
enum Importance {
    Faible,
    Moyenne,
    Élevée,
}

#[derive(Debug)]
struct Tache {
    id: u32,
    description: String,
    importance: Importance,
}

fn afficher_menu() {
    println!("1. Ajouter une tâche");
    println!("2. Afficher les tâches");
    println!("3. Supprimer une tâche");
    println!("4. Quitter");
}

fn ajouter_tache(taches: &mut Vec<Tache>, id: &mut u32) {
    print!("Entrez la description de la tâche : ");

    if let Err(_) = io::stdout().flush() {
        println!("Erreur lors de la lecture de l'entrée utilisateur");
        return;
    }

    let mut description = String::new();

    if let Err(_) = io::stdin().read_line(&mut description) {
        println!("Erreur lors de la lecture de l'entrée utilisateur");
        return;
    }

    let description = description.trim().to_string();

    println!("Choisissez l'importance (1: Faible, 2: Moyenne, 3: Élevée) : ");

    if let Err(_) = io::stdout().flush() {
        println!("Erreur lors de la lecture de l'entrée utilisateur");
        return;
    }

    let mut importance = String::new();

    if let Err(_) = io::stdin().read_line(&mut importance) {
        println!("Erreur lors de la lecture de l'entrée utilisateur");
        return;
    }

    let importance = importance.trim().parse();

    let importance = match importance {
        Ok(importance) => importance,
        Err(_) => {
            println!("Importance invalide");
            return;
        }
    };

    let importance = match importance {
        1 => Importance::Faible,
        2 => Importance::Moyenne,
        3 => Importance::Élevée,
        _ => {
            println!("Importance invalide");
            return;
        }
    };

    taches.push(Tache {
        id: *id,
        description,
        importance,
    });

    *id += 1;

    println!("Tâche ajoutée avec succès !");
}

fn afficher_taches(taches: &Vec<Tache>) {
    println!("Liste des tâches :");
    for tache in taches {
        println!(
            "{}. {} | (Importance : {:?})",
            tache.id, tache.description, tache.importance
        );
    }
}

fn suppression_tache(taches: &mut Vec<Tache>) {
    print!("Entrez l'identifiant de la tâche à supprimer : ");

    let result = io::stdout().flush();

    if let Err(_) = result {
        println!("Erreur lors de la lecture de l'entrée utilisateur");
        return;
    }

    let mut id = String::new();

    if let Err(_) = io::stdin().read_line(&mut id) {
        println!("Erreur lors de la lecture de l'entrée utilisateur");
        return;
    }

    let id = id.trim().parse();

    let id = match id {
        Ok(id) => id,
        Err(_) => {
            println!("Identifiant invalide");
            return;
        }
    };

    let mut index = None;
    for (i, tache) in taches.iter().enumerate() {
        if tache.id == id {
            index = Some(i);
            break;
        }
    }

    match index {
        Some(i) => {
            taches.remove(i);
            println!("Tâche supprimée avec succès !");
        }
        None => println!("Tâche inexistante"),
    }
}

fn main() {
    let mut taches: Vec<Tache> = Vec::new();
    let mut id: u32 = 0;

    loop {
        afficher_menu();

        print!("Choisissez une option : ");

        let result = io::stdout().flush();

        if let Err(_) = result {
            println!("Erreur lors de la lecture de l'entrée utilisateur");
            continue;
        }

        let mut choix = String::new();

        if let Err(_) = io::stdin().read_line(&mut choix) {
            println!("Erreur lors de la lecture de l'entrée utilisateur");
            continue;
        }

        let choix = choix.trim().parse();

        let choix = match choix {
            Ok(choix) => choix,
            Err(_) => {
                println!("Option invalide");
                continue;
            }
        };

        match choix {
            1 => {
                ajouter_tache(&mut taches, &mut id);
            }
            2 => {
                afficher_taches(&taches);
            }
            3 => {
                suppression_tache(&mut taches);
            }
            4 => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Option invalide"),
        }
    }
}
