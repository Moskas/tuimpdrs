use std::io::stdin;
pub fn user_prompt() -> String {
    let mut choice: String = String::new();
    stdin().read_line(&mut choice).unwrap();
    println!("{}", choice);
    choice = match &choice[..] {
        ":q" => "Quitting".to_string(),
        _ => "Yes".to_string(),
    };
    return choice;
}
