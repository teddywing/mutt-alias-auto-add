use std::io::{self, BufRead};
// use std::io::Read;

fn main() {
    // let mut input = String::new();
    // io::stdin().read_to_end(&mut input).unwrap();
        // .expect("Failed to read STDIN");

    // match io::stdin().lock() {
    //     Ok(s) => s.read_to_string(&mut input),
    //     Err(e) => panic!("Error: {}", e),
    // }

    // let stdin = io::stdin();
    // for line in stdin.lock().lines() {
    //     input.push_str(&line.unwrap());
    // }

    // let stdin = io::stdin();
    // loop {
    //     stdin.read_line(&mut input).expect("asdf");
    // }

    let stdin = io::stdin();
    let input: Vec<String> = stdin.lock().lines().map(|line| line.unwrap()).collect();

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
