use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph{
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article{
    article: String,
    author: String,
    paragraphs: Vec<Paragraph>,
}

fn read_json(json: &str) -> Article{
    let parsed: Article = serde_json::from_str(json).unwrap();
    parsed
}



fn main() {
    let json = r#"
    {
        "article": "Json in rust",
        "author": "Ashraf",
        "paragraphs": [
            {
                "name": "Introduction"
            },
            {
                "name": "Body"
            },
            {
                "name": "Conclusion"
            }
        ]
    }"#;

    let parsed  = read_json(json);

    println!("{:?}", parsed.paragraphs[0].name);

    println!("Hello, world!");
}
