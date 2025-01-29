# (dot) Properties Extended
A [`.properties`][SPEC] parser with extended bracket notation for grouping keys.

## Getting Started
This crate provides a `.properties` two static methods:
  1. `pub fn parse_file(file: &File) -> HashMap<String, String>` 
     1. Call to parse by file
  2. `pub fn parse_string(body: &str) -> HashMap<String, String>`
     1. Call to parse by string

#### Extended Bracket Notation
While `.properties` format has no official RFC, the general description
for this format is too simple and at times there's a need to group
similar keys together. 

For example:
```ini
# English translations for these labels used in the console app.
console.en.welcome = Welcome to the exciting console application!
console.en.get_started = To get started click on the "Run" button!
console.en.quit.label = Quit
console.en.quit.ask = Are you sure you want to quit?
```

As this file grows with all the languages that the console application will
support, this becomes tediously repetitive and inefficient.

The extended syntax allows scoping with the `{` and `}` brace characters
and automatically **concatenates** the sub keys together when parsing.

With the extended feature this becomes:
```ini
# English translations for these labels used in the console app.
console. {
    en. {
        welcome = Welcome to the exciting console application!
        get_started = To get started click on the "Run" button!
        quit. {
            label = Quit
            ask = Are you sure you want to quit?
        }
    }
}
```

The parsed keys will look identical to the first example in code.

> [!IMPORTANT]
> Remember this is combining the outer key and inner keys together!
> Therefore you must add the dot (.) postfix to the outer keys or use
> some other separator, otherwise parsed keys will be hard to read.

### Simple Start
Here is a very simple example to show you how to get started:

```rs
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
```

> [!TIP]
> See more in the tests defined in [`lib.rs`](./src/lib.rs).

## License
This project is licensed under the [Apache License, Version 2.0][LEGAL].

[LEGAL]: https://github.com/TheMaverickProgrammer/rust_propertiesext/LICENSE-APACHE
[SPEC]: https://en.wikipedia.org/wiki/.properties