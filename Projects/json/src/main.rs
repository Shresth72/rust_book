use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}

fn main() {
    let json = r#"
    {
        "article": "Rust",
        "author": "John Doe",
        "paragraph": [
            {
                "name": "Rust is a multi-paradigm programming language"
            },
            {
                "name": "Rust is blazingly fast and memory-efficient"
            },
            {
                "name": "Rust is memory safe"
            }
        ]
    }"#;

    let parsed: Article = read_json_typed(json);

    // ask user to enter the index of the paragraph
    println!("Enter the index of the paragraph: ");
    let mut index = String::new();
    std::io::stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Please type a number!");

    // print the paragraph
    println!("\nThe paragraph is: {}", parsed.paragraph[index].name);
}

fn read_json_typed(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    parsed
}