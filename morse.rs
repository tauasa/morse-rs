/// Morse Code encode / decode logic.
///
/// Format:
///   `.`   – dot
///   `-`   – dash
///   ` `   – letter separator  (single space)
///   ` / ` – word separator    (space-slash-space)
use std::collections::HashMap;

// Build lookup tables at compile time via a static slice.
static TABLE: &[(&str, &str)] = &[
    // Letters
    ("A", ".-"),   ("B", "-..."), ("C", "-.-."), ("D", "-.."),
    ("E", "."),    ("F", "..-."), ("G", "--."),  ("H", "...."),
    ("I", ".."),   ("J", ".---"), ("K", "-.-"),  ("L", ".-.."),
    ("M", "--"),   ("N", "-."),   ("O", "---"),  ("P", ".--."),
    ("Q", "--.-"), ("R", ".-."),  ("S", "..."),  ("T", "-"),
    ("U", "..-"),  ("V", "...-"), ("W", ".--"),  ("X", "-..-"),
    ("Y", "-.--"), ("Z", "--.."),
    // Digits
    ("0", "-----"), ("1", ".----"), ("2", "..---"), ("3", "...--"),
    ("4", "....-"), ("5", "....."), ("6", "-...."), ("7", "--..."),
    ("8", "---.."), ("9", "----."),
    // Punctuation
    (".", ".-.-.-"), (",", "--..--"), ("?", "..--.."),
    ("!", "-.-.--"), ("-", "-....-"), ("/", "-..-."),
    ("@", ".--.-."), ("(", "-.--."), (")", "-.--.-"),
];

/// Lazily-built encode table: `char → &'static str`.
fn encode_map() -> &'static HashMap<char, &'static str> {
    use std::sync::OnceLock;
    static MAP: OnceLock<HashMap<char, &'static str>> = OnceLock::new();
    MAP.get_or_init(|| {
        TABLE
            .iter()
            .map(|(ch, code)| (ch.chars().next().unwrap(), *code))
            .collect()
    })
}

/// Lazily-built decode table: `&'static str → char`.
fn decode_map() -> &'static HashMap<&'static str, char> {
    use std::sync::OnceLock;
    static MAP: OnceLock<HashMap<&'static str, char>> = OnceLock::new();
    MAP.get_or_init(|| {
        TABLE
            .iter()
            .map(|(ch, code)| (*code, ch.chars().next().unwrap()))
            .collect()
    })
}

/// Encode plain text → Morse code.
///
/// Letters are separated by a single space; words by ` / `.
/// Input is folded to uppercase before encoding.
///
/// # Errors
/// Returns an error string describing the first unsupported character found.
pub fn encode(text: &str) -> Result<String, String> {
    if text.trim().is_empty() {
        return Err("Input text is empty.".into());
    }

    let map = encode_map();
    let mut words_out: Vec<String> = Vec::new();

    for word in text.split_whitespace() {
        let mut codes: Vec<&str> = Vec::with_capacity(word.len());
        for ch in word.chars() {
            let upper: char = ch.to_uppercase().next().unwrap();
            match map.get(&upper) {
                Some(code) => codes.push(code),
                None => return Err(format!("Unsupported character: '{ch}'")),
            }
        }
        words_out.push(codes.join(" "));
    }

    Ok(words_out.join(" / "))
}

/// Decode Morse code → plain text.
///
/// Expects letters separated by single spaces and words by ` / `.
///
/// # Errors
/// Returns an error string for any unrecognised Morse sequence.
pub fn decode(morse: &str) -> Result<String, String> {
    if morse.trim().is_empty() {
        return Err("Morse input is empty.".into());
    }

    let map = decode_map();

    // Collapse runs of multiple spaces so the split is robust.
    let normalised = morse
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    let mut result = String::new();

    for (wi, word) in normalised.split(" / ").enumerate() {
        if wi > 0 {
            result.push(' ');
        }
        for code in word.split(' ').filter(|s| !s.is_empty()) {
            match map.get(code) {
                Some(&ch) => result.push(ch),
                None => return Err(format!("Unknown Morse sequence: '{code}'")),
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Encoding ──────────────────────────────────────────────────────────────

    #[test]
    fn encode_single_letter() {
        assert_eq!(encode("A").unwrap(), ".-");
    }

    #[test]
    fn encode_word() {
        assert_eq!(encode("SOS").unwrap(), "... --- ...");
    }

    #[test]
    fn encode_multiple_words() {
        assert_eq!(encode("HI THERE").unwrap(), ".... .. / - .... . .-. .");
    }

    #[test]
    fn encode_lowercase_normalised() {
        assert_eq!(encode("a").unwrap(), ".-");
    }

    #[test]
    fn encode_digits() {
        assert_eq!(encode("123").unwrap(), ".---- ..--- ...--");
    }

    #[test]
    fn encode_punctuation() {
        assert_eq!(encode(".").unwrap(), ".-.-.-");
    }

    #[test]
    fn encode_unsupported_char() {
        assert!(encode("Hello #World").is_err());
    }

    #[test]
    fn encode_empty_input() {
        assert!(encode("   ").is_err());
    }

    // ── Decoding ──────────────────────────────────────────────────────────────

    #[test]
    fn decode_single_code() {
        assert_eq!(decode(".-").unwrap(), "A");
    }

    #[test]
    fn decode_word() {
        assert_eq!(decode("... --- ...").unwrap(), "SOS");
    }

    #[test]
    fn decode_multiple_words() {
        assert_eq!(decode(".... .. / - .... . .-. .").unwrap(), "HI THERE");
    }

    #[test]
    fn decode_digits() {
        assert_eq!(decode(".---- ..--- ...--").unwrap(), "123");
    }

    #[test]
    fn decode_unknown_sequence() {
        assert!(decode("..---.").is_err());
    }

    #[test]
    fn decode_empty_input() {
        assert!(decode("").is_err());
    }

    // ── Round-trip ────────────────────────────────────────────────────────────

    #[test]
    fn round_trip_simple() {
        let original = "HELLO WORLD";
        assert_eq!(decode(&encode(original).unwrap()).unwrap(), original);
    }

    #[test]
    fn round_trip_with_digits_and_punctuation() {
        let original = "MEETING AT 3PM";
        assert_eq!(decode(&encode(original).unwrap()).unwrap(), original);
    }
}
