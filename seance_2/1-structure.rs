#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("\nUser information : {:?}", user1);

    user1.email = String::from("anotheremail@example.com");

    println!("\nUser information after modifications : {:?}", user1);

    let built_user = build_user("gigachad@hotmail.com".to_string(), "Noah".to_string());

    println!("\nFirst built user : {:?}", built_user);

    let second_built_user = build_user_v2("beta@hotmail.com".to_string(), "Tommy".to_string());

    println!("\nSecond built user : {:?}", second_built_user);

    // On peut construire un utilisateur en complétant avec les champs d'un autre utilisateur.
    let mixedUser = User {
        email: String::from("another@example.com"),
        ..built_user
    };

    println!("\nMixed built user : {:?}", mixedUser);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn build_user_v2(email: String, username: String) -> User {
    User {
        active: true,
        username, // Doit avoir le même nom que le paramètre de la structure !
        email,    // Doit avoir le même nom que le paramètre de la structure !
        sign_in_count: 1,
    }
}
