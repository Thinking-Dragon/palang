use std::io;
use std::io::Write;

pub fn ask(question: &str) -> Result<String, String> {
    print!("{}: ", question);
    io::stdout().flush().unwrap();
    let mut answer: String = String::new();
    io::stdin().read_line(&mut answer).map_err(|e| e.to_string())?;

    answer.trim().parse::<String>().map_err(|e| e.to_string())
}
