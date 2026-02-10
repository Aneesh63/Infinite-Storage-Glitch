use std::env;

const ONE_COLOR: &str = "Forest Green";
const ZERO_COLOR: &str = "Olive Green";

fn byte_to_colors(byte: u8) -> Vec<&'static str> {
    (0..8)
        .rev()
        .map(|i| {
            if (byte >> i) & 1 == 1 {
                ONE_COLOR
            } else {
                ZERO_COLOR
            }
        })
        .collect()
}

fn encode_text(input: &str) -> Vec<String> {
    input
        .bytes()
        .map(|b| byte_to_colors(b).join(" | "))
        .collect()
}

fn decode_bits(bits: &str) -> Result<String, String> {
    let cleaned: String = bits.chars().filter(|c| *c == '0' || *c == '1').collect();
    if cleaned.is_empty() || cleaned.len() % 8 != 0 {
        return Err("Bit input must contain 0/1 characters and be divisible by 8.".to_string());
    }

    let mut out = String::new();
    for chunk in cleaned.as_bytes().chunks(8) {
        let chunk_str = std::str::from_utf8(chunk).map_err(|_| "Invalid UTF-8 in bit chunk")?;
        let value = u8::from_str_radix(chunk_str, 2)
            .map_err(|_| format!("Could not parse bit chunk: {chunk_str}"))?;
        out.push(value as char);
    }

    Ok(out)
}

fn colors_to_bits(input: &str) -> String {
    let lower = input.to_lowercase();
    let normalized = lower
        .replace("forest green", "1")
        .replace("olive green", "0")
        .replace(|c: char| !matches!(c, '0' | '1'), " ");

    normalized.split_whitespace().collect::<Vec<_>>().join("")
}

fn print_usage(bin_name: &str) {
    println!("Rain Light encoder/decoder");
    println!("Mapping: {ONE_COLOR} = 1, {ZERO_COLOR} = 0");
    println!();
    println!("Usage:");
    println!("  {bin_name} encode <text>");
    println!("  {bin_name} decode-bits <bits>");
    println!("  {bin_name} decode-colors <color words>");
    println!();
    println!("Examples:");
    println!("  {bin_name} encode Hello");
    println!("  {bin_name} decode-bits 0100100001101001");
    println!("  {bin_name} decode-colors \"Forest Green Olive Green ...\"");
}

fn main() {
    let mut args = env::args();
    let bin = args.next().unwrap_or_else(|| "rain_light".to_string());

    let command = match args.next() {
        Some(cmd) => cmd,
        None => {
            print_usage(&bin);
            return;
        }
    };

    match command.as_str() {
        "encode" => {
            let text = args.collect::<Vec<_>>().join(" ");
            if text.is_empty() {
                eprintln!("Please provide text to encode.");
                print_usage(&bin);
                return;
            }

            println!("Input text: {text}");
            println!("Encoded color lines (one line per character):");
            for (index, line) in encode_text(&text).iter().enumerate() {
                println!("{:>3}: {}", index + 1, line);
            }
        }
        "decode-bits" => {
            let bit_input = args.collect::<Vec<_>>().join("");
            match decode_bits(&bit_input) {
                Ok(text) => println!("Decoded text: {text}"),
                Err(err) => eprintln!("Decode error: {err}"),
            }
        }
        "decode-colors" => {
            let color_words = args.collect::<Vec<_>>().join(" ");
            if color_words.is_empty() {
                eprintln!("Please provide color words to decode.");
                return;
            }

            let bits = colors_to_bits(&color_words);
            match decode_bits(&bits) {
                Ok(text) => {
                    println!("Detected bits: {bits}");
                    println!("Decoded text: {text}");
                }
                Err(err) => eprintln!("Decode error: {err}"),
            }
        }
        _ => {
            eprintln!("Unknown command: {command}");
            print_usage(&bin);
        }
    }
}
