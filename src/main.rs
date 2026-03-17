use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;

#[derive(Deserialize)]
struct Project {
    name: String,
    url: String,
    description: String,
    category: String,
}

fn main() {
    let raw_data = fs::read_to_string("data.json").expect("Failed to read data.json");

    let data: Vec<Project> = serde_json::from_str(&raw_data).expect("Failed to parse data.json");

    let mut categories: BTreeMap<String, Vec<&Project>> = BTreeMap::new();
    for project in &data {
        categories
            .entry(project.category.clone())
            .or_default()
            .push(project);
    }

    let mut output = String::from("## Active Projects\n");

    for (category, projects) in &categories {
        output.push_str(&format!("\n### {}\n\n", category));

        for project in projects {
            output.push_str(&format!(
                "- [{}]({}): {}\n",
                project.name, project.url, project.description
            ));
        }
    }

    fs::write("README.md", &output).expect("Failed to write README.md");
}
