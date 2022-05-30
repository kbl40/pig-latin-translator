use std::io;
//use std::io::Write;

fn main() {
    let mut s = String::new();

    println!("What's your word?");

    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line");

    let s = s.trim();

    let pre = &s[0..1];

    // Determine which function to run depending on first letter (either vowel or consanent)
    match pre {
        "a" => vowel(&s),
        "e" => vowel(&s),
        "i" => vowel(&s),
        "o" => vowel(&s),
        "u" => vowel(&s),
        _ => consanent(&s),
    }

    println!("End of test");
}

// Prints pig latin translation for words that begin with a vowel
fn vowel(string: &str) {
    let append = "hay";
    //let pl = format!("{}-{}", string, append);
    println!("{}-{}", string, append);
}

// Prints pig latin translation for words that begin with a consanent
fn consanent(string: &str) {
    let root = &string[1..];
    let prefix = &string[0..1];
    println!("{}-{}ay", root, prefix);
}
