/*  Marco Polo Game
*/

pub fn marco_polo(name: &str) -> String {
    if name == "Marco" {
        "Polo".to_string()
    }
    else {
        "What's your name?".to_string()
    }
}