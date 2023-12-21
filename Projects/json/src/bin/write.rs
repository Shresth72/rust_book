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
    let article: Article = Article{
        article: String::from("Rust"),
        author: String::from("John Doe"),
        paragraph: vec![
            Paragraph{
                name: String::from("Rust is a multi-paradigm programming language")
            },
            Paragraph{
                name: String::from("Rust is blazingly fast and memory-efficient")
            },
            Paragraph{
                name: String::from("Rust is memory safe")
            }
        ]
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("{}", json);
}