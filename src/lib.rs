use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

use enums::{Glyphs, DOUBLE_BACKSLASH};
use utils::StringUtils;

pub mod enums;
pub mod utils;

type MapType = HashMap<String, String>;

struct Parser {
    results: MapType,
    multiline: bool,
    trailing_append: bool,
    key: String,
    val: String,
    prefix: String,
    prefix_stack: Vec<String>,
}

impl Parser {
    fn new() -> Self {
        Self {
            results: MapType::new(),
            multiline: false,
            trailing_append: false,
            key: String::new(),
            val: String::new(),
            prefix: String::new(),
            prefix_stack: Vec::new(),
        }
    }

    fn parse_line(&mut self, mut line: &str) {
        line = &mut line.trim();

        if line.is_empty() {
            return;
        }

        if self.multiline {
            if line.ends_with(Glyphs::Backslash.as_str()) {
                self.val += &line[..line.len() - 1];
                self.val += "\n";
                return;
            } else {
                self.val += line;
            }

            self.multiline = false;
            self.results.insert(self.key.clone(), self.val.clone());
            return;
        }

        if self.trailing_append && line.starts_with(Glyphs::CurlyRight.as_str()) {
            self.prefix_stack.pop();

            self.trailing_append = !self.prefix_stack.is_empty();
            self.prefix.clear();
            for p in &self.prefix_stack {
                self.prefix.push_str(&p);
            }
            return;
        }

        // Lines starting with # or ! are comments.
        if line.starts_with([Glyphs::Hash.as_char(), Glyphs::Bang.as_char()]) {
            return;
        }

        // .properties files prioritize = and : over space terminators.
        let mut line_iter = line.as_bytes().iter();

        let mut pos =
            line_iter.position(|x| *x == Glyphs::Equal.value() || *x == Glyphs::Colon.value());

        if pos == None {
            // If those prioritized failed, then seek space terminators.
            pos = line_iter.position(|x| *x == Glyphs::Space.value());
        }

        if let Some(p) = pos {
            self.key = (&line[..p]).to_string();
            self.val = (&line[(p + 1)..]).to_string();
            self.key.trim();
            self.val.trim();

            if self.trailing_append {
                self.key = self.prefix.clone() + &self.key;
            }

            if self.val.is_empty() {
                // If none of these were found, then the whole line must be a key...
                // Note: here the line is already stored in `key`.
                if self.key.ends_with(Glyphs::CurlyLeft.as_char()) {
                    self.trailing_append = true;
                    let next_key = self.key[..self.key.len() - 1].to_owned();
                    self.prefix_stack.push(next_key.clone());
                    self.prefix += &next_key;
                    return;
                }

                // Edge case: key with no value
                self.results.insert(line.to_string(), "".to_string());
            }

            // If the value ends with a backslash, we have a multiline key.
            // We must check to see if this backslash is not delimited.
            if self.val.len() > 1
                && self.val.ends_with(Glyphs::Backslash.as_str())
                && !self.val.ends_with(DOUBLE_BACKSLASH)
            {
                self.val.remove(self.val.len() - 1);
                self.val += "\n";
                self.multiline = true;
                return;
            }

            // Insert the parsed key value pair
            self.results.insert(self.key.clone(), self.val.clone());
        } else {
            // If none of these were found, then the whole line must be a key...
            if line.ends_with(Glyphs::CurlyLeft.as_char()) {
                self.trailing_append = true;
                let next_key = line[..line.len() - 1].to_owned();
                self.prefix_stack.push(next_key.clone());
                self.prefix += &next_key;
                return;
            }

            // Intentionally set key with no value
            if self.trailing_append {
                self.results
                    .insert(self.prefix.clone() + line, "".to_string());
            } else {
                self.results.insert(line.to_string(), "".to_string());
            }
            return;
        }
    }
}

pub fn parse_string(body: &str) -> MapType {
    let mut parser = Parser::new();

    for mut line in body.split('\n') {
        parser.parse_line(&mut line);
    }

    parser.results
}

pub fn parse_file(file: &File) -> MapType {
    let mut parser = Parser::new();

    let lines = io::BufReader::new(file).lines();

    // Consumes the iterator, returns an (Optional) String
    for mut line in lines.map_while(Result::ok) {
        parser.parse_line(&mut line);
    }

    parser.results
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_parse_string() {
        let body = "my-app.{
            service-in-use=\"{}\"\\
                is in use.
            ask-delete=You want to delete \"{}\"?
            ask-restart={} already has a running service.\\
                Do you want to restart it?
        }

        mail-service.{
            default=Which mail do you want to read?
            empty= You have no mail
        }

        # General YES/NO labels
        *.{
            yes=YES
            no=NO
        }";

        let hash = super::parse_string(body);

        for (key, val) in &hash {
            println!("key={}, val={}", key, val);
        }

        assert_eq!(hash.len(), 7);
        assert_eq!(
            hash.get("my-app.service-in-use").unwrap(),
            "\"{}\"\nis in use."
        );
        assert_eq!(
            hash.get("my-app.ask-delete").unwrap(),
            "You want to delete \"{}\"?"
        );
        assert_eq!(
            hash.get("my-app.ask-restart").unwrap(),
            "{} already has a running service.\nDo you want to restart it?"
        );
        assert_eq!(
            hash.get("mail-service.default").unwrap(),
            "Which mail do you want to read?"
        );
        assert_eq!(hash.get("mail-service.empty").unwrap(), "You have no mail");
        assert_eq!(hash.get("*.yes").unwrap(), "YES");
        assert_eq!(hash.get("*.no").unwrap(), "NO");
    }

    #[test]
    fn colon_test() {
        let body = "my-app.{
            service-in-use:\"{}\"\\
                is in use.
            ask-delete:   You want to delete \"{}\"?
            ask-restart  :  {} already has a running service.\\
                Do you want to restart it?
        }

        mail-service.{
            default        :Which mail do you want to read?
            empty: You have no mail
        }

        # General YES/NO labels
        *.{
            yes : YES
            no      :      NO
        }";

        let hash = super::parse_string(body);

        for (key, val) in &hash {
            println!("key={}, val={}", key, val);
        }

        assert_eq!(hash.len(), 7);
        assert_eq!(
            hash.get("my-app.service-in-use").unwrap(),
            "\"{}\"\nis in use."
        );
        assert_eq!(
            hash.get("my-app.ask-delete").unwrap(),
            "You want to delete \"{}\"?"
        );
        assert_eq!(
            hash.get("my-app.ask-restart").unwrap(),
            "{} already has a running service.\nDo you want to restart it?"
        );
        assert_eq!(
            hash.get("mail-service.default").unwrap(),
            "Which mail do you want to read?"
        );
        assert_eq!(hash.get("mail-service.empty").unwrap(), "You have no mail");
        assert_eq!(hash.get("*.yes").unwrap(), "YES");
        assert_eq!(hash.get("*.no").unwrap(), "NO");
    }

    #[test]
    fn no_multiline() {
        let body = "hello=world
            weather: good
            # this is a comment
            ! this is also a comment
            wumbology: the study of wumbo
            ";

        let hash = super::parse_string(body);

        for (key, val) in &hash {
            println!("key={}, val={}", key, val);
        }

        assert_eq!(hash.len(), 3);
        assert_eq!(hash.get("hello").unwrap(), "world");
        assert_eq!(hash.get("weather").unwrap(), "good");
        assert_eq!(hash.get("wumbology").unwrap(), "the study of wumbo");
    }
}
