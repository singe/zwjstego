use std::str::from_utf8;
use clap::{Arg, ArgAction, Command};
use std::io::{self, Read};

fn byte_to_variation_selector(byte: u8) -> char {
    if byte < 16 {
        char::from_u32(0xFE00 + byte as u32).unwrap()
    } else {
        char::from_u32(0xE0100 + (byte - 16) as u32).unwrap()
    }
}

fn encode(base: char, bytes: &[u8]) -> String {
    let mut result = String::new();
    result.push(base);
    for byte in bytes {
        result.push(byte_to_variation_selector(*byte));
    }
    result
}
fn variation_selector_to_byte(variation_selector: char) -> Option<u8> {
    let variation_selector = variation_selector as u32;
    if (0xFE00..=0xFE0F).contains(&variation_selector) {
        Some((variation_selector - 0xFE00) as u8)
    } else if (0xE0100..=0xE01EF).contains(&variation_selector) {
        Some((variation_selector - 0xE0100 + 16) as u8)
    } else {
        None
    }
}

fn decode(variation_selectors: &str) -> Vec<u8> {
    let mut result = Vec::new();
    
    for variation_selector in variation_selectors.chars() {
        if let Some(byte) = variation_selector_to_byte(variation_selector) {
            result.push(byte);
        } else if !result.is_empty() {
            return result;
        }
        // note: we ignore non-variation selectors until we have
        // encountered the first one, as a way of skipping the "base
        // character".
    }

    result
}

fn main() {
    let matches = Command::new("hidden-message")
        .version("1.0")
        .about("Encodes or decodes hidden messages")
        .arg(
            Arg::new("decode")
                .short('d')
                .long("decode")
                .help("Decode the hidden message")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("emoji")
                .short('e')
                .long("emoji")
                .help("The emoji to use when encoding (defaults to 😊)")
                .num_args(1),
        )
        .arg(
            Arg::new("message")
                .help("The message to encode/decode. If omitted, reads from STDIN")
                .index(1)
                .required(false),
        )
        .get_matches();

    // Determine the message: either from the positional argument or from STDIN.
    let message = if let Some(msg) = matches.get_one::<String>("message") {
        msg.clone()
    } else {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .expect("Failed to read from STDIN");
        buffer.trim().to_string()
    };

    // Check which mode to use: decoding or encoding.
    if matches.get_flag("decode") {
        let result = &decode(&message);
        println!("{:?}", from_utf8(result).unwrap()); // "hello"
    } else {
        // For encoding, use the provided emoji or default to 😊.
        let emoji = matches
            .get_one::<String>("emoji")
            .and_then(|s| s.chars().next())
            .unwrap_or('😊');

         if !message.is_ascii() {
             eprintln!("Error: The message must contain only ASCII characters.");
             std::process::exit(1);
         }

        let result = encode(emoji, message.as_bytes());
        println!("{}", result);
        //println!("Encoded debug view: {:?}", result);
    }
}
