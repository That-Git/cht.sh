use std::{io::Write, process::{Command, Stdio}};

fn prompt(prompt:&str) -> String {
    let mut line = String::new();
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");

    return line.trim().to_string()
}

fn main() /* -> &'static str */ {
    let topic: String = prompt("topic: ");
    let query: String = prompt("query: ");
    // println!("{}", (format!("https://cht.sh/{}~{}", topic.to_string(), query.to_string())).as_str());
    let output = Command::new("curl").arg((format!("https://cht.sh/{}/{}", topic.to_string(), query.to_string())).as_str())
        // Tell the OS to record the command's output
        .stdout(Stdio::piped())
        // execute the command, wait for it to complete, then capture the output
        .output()
        // Blow up if the OS was unable to start the program
        .unwrap();
    // extract the raw bytes that we captured and interpret them as a string
    let hypertext = String::from_utf8(output.stdout).unwrap();
    println!("{}", hypertext);
}
