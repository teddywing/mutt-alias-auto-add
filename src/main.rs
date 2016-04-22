use std::io::{self, BufRead, BufReader, Write};
use std::fs::{File, OpenOptions};
use std::path::Path;

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

    fn write_to_file<P: AsRef<Path>>(&self, file: P) -> Result<(), io::Error> {
        let mut f = try!(OpenOptions::new().write(true).open(file));
        try!(f.write_all(self.to_string().as_bytes()));
        Ok(())
    }

    fn update_alias_id(&mut self, similar_aliases: Vec<String>) {
        if !similar_aliases.is_empty() {
            self.alias = format!("{}-{}", self.alias, similar_aliases.len() + 1);
        }
    }
}

fn handle_alias(s: &str) {
}

#[derive(Debug)]
enum AliasSearchError {
    NotFound,
    EmailExists,
    Io(io::Error),
}

// impl fmt::Display for AliasSearchError {}
// impl error::Error for AliasSearchError {}

impl From<io::Error> for AliasSearchError {
    fn from(err: io::Error) -> AliasSearchError {
        AliasSearchError::Io(err)
    }
}

#[cfg(test)]
impl PartialEq<AliasSearchError> for AliasSearchError {
    fn eq(&self, other: &AliasSearchError) -> bool {
        match *self {
            AliasSearchError::NotFound => match *other {
                AliasSearchError::NotFound => true,
                _ => false,
            },
            AliasSearchError::EmailExists => match *other {
                AliasSearchError::EmailExists => true,
                _ => false,
            },
            AliasSearchError::Io(_) => match *other {
                AliasSearchError::Io(_) => true,
                _ => false,
            },
        }
    }
}

fn find_alias_in_file(alias: &Alias, file: &str) -> Result<Vec<String>, AliasSearchError> {
    let mut matches = Vec::new();
    let f = try!(File::open(file));
    let file = BufReader::new(&f);
    for line in file.lines() {
        let line = try!(line);
        let split: Vec<&str> = line.split_whitespace().collect();

        if line.contains(&alias.email) {
            return Err(AliasSearchError::EmailExists)
        }

        if split[1].starts_with(&alias.alias) {
            matches.push(split[1].to_owned());
        }
    }

    if matches.is_empty() {
        Err(AliasSearchError::NotFound)
    } else {
        Ok(matches)
    }
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
