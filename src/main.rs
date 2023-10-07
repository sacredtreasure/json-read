extern crate serde_json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Monotype {
    title: String,
    version: String,
    packet_wt: String,
}

#[derive(Serialize, Deserialize)]
struct Prototype {
    creator: String,
    #[serde(rename = "version control")]
    version_control: String,
    class: String,
    monotype: Vec<Monotype>,
}

fn main() {
    let json = r#"
    {
        "creator": "Steven Spielberg",
        "version control": "1.1",
        "class": "S-class",
        "monotype": [
            {
                "title": "Jurassic Park",
                "version": "1993",
                "packet_wt": "4"
            },
            {
                "title": "The Adventures of Tintin",
                "version": "2011",
                "packet_wt": "3"
            },
            {
                "title": "The BFG",
                "version": "2016",
                "packet_wt": "4"
            },
            {
                "title": "Ready Player One",
                "version": "2018",
                "packet_wt": "5"
            },
            {
                "title": "The Fabelmans",
                "version": "2022",
                "packet_wt": "2"
            }
        ]
    }"#;

    let screened = read_json_typed(json);
    println!(
        "\n\n Steven Spielberg's most prominent work in the year {} was: {}",
        screened.monotype[0].version,
        screened.monotype[0].title
    );
}

fn read_json_typed(raw_json: &str) -> Prototype {
    let screened = serde_json::from_str(raw_json).unwrap();
    return screened;
}
