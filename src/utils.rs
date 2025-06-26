use std::collections::BTreeSet;

pub fn gather_imports<'a>(imports: impl Iterator<Item = &'a str>) -> String {
    let mut import_set: BTreeSet<&'a str> = BTreeSet::from(["import dataclasses"]);

    for import in imports {
        import_set.insert(&import);
    }

    let mut output = String::new();

    for import in import_set {
        output += import;
        output += "\n";
    }

    return output;
}

pub fn to_pascal_case(s: &str) -> String {
    s.split(|c: char| !c.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => {
                    first.to_ascii_uppercase().to_string() + &chars.as_str().to_ascii_lowercase()
                }
                None => String::new(),
            }
        })
        .collect::<String>()
}

pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();

    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i != 0 {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }

    result
}

#[test]
fn test_to_snake_case() {
    assert_eq!("hello_world", to_snake_case("HelloWorld"))
}

#[test]
fn test_to_pascal_case() {
    assert_eq!("HelloWorld", to_pascal_case("hello_world"))
}
