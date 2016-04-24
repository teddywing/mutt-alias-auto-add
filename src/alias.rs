use std::error::{self, Error};
use std::fs::{File, OpenOptions};
use std::fmt;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

#[derive(Clone)]
pub struct Alias {
    pub alias: String,
    pub name: String,
    pub email: String,
}

impl Alias {
    pub fn new(email: &str) -> Alias {
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

    pub fn to_string(&self) -> String {
        if self.name.is_empty() {
            format!("alias {} {}", self.alias, self.email)
        } else {
            format!("alias {} {} {}", self.alias, self.name, self.email)
        }
    }

    pub fn find_in_file<P: AsRef<Path>>(&self, file: P) -> Result<Vec<String>, AliasSearchError> {
        let mut matches = Vec::new();
        let f = try!(File::open(file));
        let file = BufReader::new(&f);
        for line in file.lines() {
            let line = try!(line);
            let split: Vec<&str> = line.split_whitespace().collect();

            if line.contains(&self.email) {
                return Err(AliasSearchError::EmailExists)
            }

            if split[1].starts_with(&self.alias) {
                matches.push(split[1].to_owned());
            }
        }

        if matches.is_empty() {
            Err(AliasSearchError::NotFound)
        } else {
            Ok(matches)
        }
    }

    pub fn write_to_file<P: AsRef<Path>>(&mut self, file: P) -> Result<(), AliasSearchError> {
        match self.find_in_file(&file) {
            Ok(similar_aliases) => self.update_alias_id(similar_aliases),
            Err(AliasSearchError::NotFound) => {},
            Err(e) => return Err(e),
        };

        let mut f = try!(OpenOptions::new().append(true).open(file));
        try!(f.write_all(format!("{}\n", self.to_string()).as_bytes()));

        Ok(())
    }

    pub fn update_alias_id(&mut self, similar_aliases: Vec<String>) {
        if !similar_aliases.is_empty() {
            self.alias = format!("{}-{}", self.alias, similar_aliases.len() + 1);
        }
    }
}

#[derive(Debug)]
pub enum AliasSearchError {
    NotFound,
    EmailExists,
    Io(io::Error),
}

impl fmt::Display for AliasSearchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AliasSearchError::NotFound => writeln!(f, "{}", self.description()),
            AliasSearchError::EmailExists => writeln!(f, "{}", self.description()),
            AliasSearchError::Io(ref err) => writeln!(f, "IO error: {}", err),
        }
    }
}

impl error::Error for AliasSearchError {
    fn description(&self) -> &str {
        match *self {
            AliasSearchError::NotFound => "Alias could not be found in aliases file",
            AliasSearchError::EmailExists => "Email already exists in aliases file",
            AliasSearchError::Io(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            AliasSearchError::Io(ref err) => Some(err),
            _ => None,
        }
    }
}

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
