use std::env;
use std::convert::TryFrom;

fn main() {
    let args: Vec<String> = env::args().collect();

    let control_word = match args.get(1) {
        None => { println!("no arguments given. --help for help."); String::new() },
        Some(x) => { x.to_string() }
    };
        if control_word.starts_with(&String::from("--help")) {
            help();
        }
        else if control_word.eq(&String::from("-e")) {
            let text = args.get(2).unwrap_or(&String::new()).to_string();
            let key = args.get(3).unwrap_or(&String::new()).to_string();
            initialise_encode(&text, &key)
        }
        else if control_word.eq(&String::from("-d")) {
            let text = args.get(2).unwrap_or(&String::new()).to_string();
            let key = args.get(3).unwrap_or(&String::new()).to_string();
            initialise_decode(&text, &key)
        }
        else {
            println!("unknown arguments --help for help ");
        }
}

    fn help(){
        println!("-e [text] [key]        : encode the text must be given without spaces.");
        println!("-d [cipher-text] [key] : decode the text must be given without spaces.");
        println!("--help                 : help.");
    }

fn initialise_encode(text : &String, key : &String) {

    let controll = &String::new();
    if !text.eq(controll) && !key.eq(controll) {
        encode(text,key);
    } else {
        println!("not enough arguments given")
    }
}

fn encode(text: &String, key: &String) {
    let text_bytes = text.to_ascii_lowercase().as_bytes().to_vec();
    let key_bytes = key.to_ascii_lowercase().as_bytes().to_vec();
    let mut cipher = Vec::new();
    for i in 0..text_bytes.len(){
        let key_index = i % key_bytes.len();
        let new_char = char::from((((text_bytes.get(i).unwrap() - 97) + (key_bytes.get(key_index).unwrap() -97)) % 26) + 97).to_ascii_lowercase();
        cipher.push(new_char);
    }
    let c : String = cipher.iter().collect();
    println!("encode : {}", c);
}
fn initialise_decode(cipher : &String, key : &String) {
    let controll = &String::new();
    if cipher != controll && key != controll {
        decode(cipher,key);
    } else {
        println!("not enough arguments given")
    }
}

fn decode(cipher: &String, key: &String) {
    let cipher_bytes = cipher.to_ascii_lowercase().as_bytes().to_vec();
    let key_bytes = key.to_ascii_lowercase().as_bytes().to_vec();
    let mut text = Vec::new();
    for i in 0..cipher_bytes.len() {
        let key_index = i % key_bytes.len();
        let u = i32::from(cipher_bytes.get(i).unwrap() - 97) - i32::from(key_bytes.get(key_index).unwrap() - 97);
        let cipher_number = u8::try_from( if (u < 0) {123 + u} else {u + 97}).unwrap();
        let new_char = char::from(cipher_number).to_ascii_lowercase();
        text.push(new_char);
    }
    let t: String = text.iter().collect();
    println!("decoded : {}", t);
}