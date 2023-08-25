use std::io::{self, BufRead};

fn main() {
    println!("Bienvenue! Je suis un chatbot. Posez-moi des questions ou dites 'bye' pour quitter.");

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Impossible de lire l'entrée de l'utilisateur");

        let response = chatbot_response(&input);
        println!("Bot : {}", response);

        if input.trim().eq_ignore_ascii_case("bye") {
            println!("Au revoir!");
            break;
        }
    }
}

fn chatbot_response(input: &str) -> String {
    if input.contains("Comment ça va?") {
        return String::from("Je vais bien, merci!");
    }

    if input.contains("Quel est ton nom?") {
        return String::from("Je suis Chatbot, enchanté!");
    }

    // Autres réponses possibles avec des conditions

    return String::from("Désolé, je ne comprends pas. Pouvez-vous reformuler votre question?");
}
