use std::io::{self, BufRead, BufReader};
use std::fs::File;

#[cfg(test)]
mod tests;

struct Alias {
    alias: String,
    name: String,
    email: String,
}

impl Alias {
    fn new(email: &str) -> Alias {
        let mut split: Vec<&str> = email.split_whitespace().collect();

        // Remove "From: "
        split.remove(0);

        let mut alias = String::new();
        let mut name = String::new();
        let mut email = String::new();

        if split.len() == 1 {
            alias = split[0].to_lowercase().to_string();
            email = split[0].to_string();
        } else if split.len() == 2 {
            alias = split[0].to_lowercase().to_string();
            name = split[0].to_string();
            email = split[1].to_string();
        } else if split.len() > 2 {
            alias = format!("{}-{}", split[split.len() - 2], split[0]).to_lowercase().to_string();
            name = split[0..(split.len() - 1)].join(" ");
            email = split[split.len() - 1].to_string();
        }

        alias = alias.replace(',', "");
        alias = alias.replace('\'', "");
        alias = alias.replace('"', "");

        Alias { alias: alias, name: name, email: email }
    }

    fn to_string(&self) -> String {
        if self.name.is_empty() {
            format!("alias {} {}", self.alias, self.email)
        } else {
            format!("alias {} {} {}", self.alias, self.name, self.email)
        }
    }
}

fn handle_alias(s: &str) {
    let alias = build_alias(s);
}

fn build_alias(s: &str) -> String {
    let mut split: Vec<&str> = s.split_whitespace().collect();

    // Remove "From: "
    split.remove(0);

    let mut alias_line = String::from("alias ");
    let mut alias = String::new();

    if split.len() == 1 {
        alias = format!("{} ", split[0].to_lowercase());
    } else if split.len() == 2 {
        alias = format!("{} ", split[0].to_lowercase());
    } else if split.len() > 2 {
        alias = format!("{}-{} ", split[split.len() - 2], split[0]).to_lowercase();
    }

    alias = alias.replace(',', "");
    alias = alias.replace('\'', "");
    alias = alias.replace('"', "");

    alias_line.push_str(&alias);
    alias_line.push_str(&split.join(" "));

    alias_line
}

fn is_alias_in_file(alias: &Alias, file: &str) -> Result<(), io::Error> {
    let f = try!(File::open(file));
    let file = BufReader::new(&f);
    for line in file.lines() {
        let line = line.unwrap();
        let split: Vec<&str> = line.split_whitespace().collect();

        // if email is in alias file
        // return true

        if split[1].starts_with(&alias.alias) {
            println!("booya");
        }
    }

    Ok(())
}

fn main() {
    let stdin = io::stdin();
    let input: Vec<String> = stdin.lock().lines().map(|line| line.unwrap()).collect();

    for line in &input {
        if line.starts_with("From: ") {
            println!("!!!!!!!! {}", line);
            // run matcher function
            handle_alias(line);
        }
    }

    for l in &input {
        println!("{}", l);
    }

// if alias not new
// 	do nothing
// if alias is new
// 	if real_alias is in alias file
// 		if email address is in alias file
// 			do nothing
// 		
// 		find all instances of real_alias
// 			get instance with greatest id number
// 			
// 			change real_alias to "#{real_alias}-#{id + 1}"
// 			
// 			if only one instance
// 				append "-2" to real_alias
// 	
// 	add alias to alias file
}
