use std::io::{self, BufRead};

#[cfg(test)]
mod tests;

fn handle_alias(s: &str) {
    let alias = build_alias(s);
}

fn build_alias(s: &str) -> String {
    String::from("String")
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
