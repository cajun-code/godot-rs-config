use std::{fs, path::Path};
use reqwest::{self, StatusCode};
use convert_case::{Casing,Case};
use anyhow::{Result, Ok};
use tera::Tera;
const CLASS_URL: &str = "https://raw.githubusercontent.com/cajun-code/godot-rs-template/main/src/class.rs";
pub async fn generate_class(class_name: &String, parent_name: &String) -> Result<()>{
    if check_if_project(){
        let resposne = reqwest::get(CLASS_URL).await.unwrap();
        if resposne.status() == StatusCode::OK{
            let text = resposne.text().await.unwrap();
            let mut tera = Tera::new("templates/**/*").unwrap();
            let _ = tera.add_raw_template("class", text.as_str()).unwrap();
            let mut context = tera::Context::new();
            let name = class_name.to_case(Case::Pascal);
            context.insert("class_name", name.as_str());
            let parent = parent_name.to_case(Case::Pascal);
            context.insert("parent_name", parent.as_str());
            let file_name = class_name.to_case(Case::Snake);
            let rendered = tera.render("class", &context).unwrap();
            let file_path = format!("./src/{}.rs", file_name);
            fs::write(file_path, rendered.as_bytes()).unwrap();
        }
    }
    Ok(())
}

fn check_if_project() -> bool{
    Path::new("./src").is_dir()
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_check_if_project(){
        assert!(check_if_project(), "Not in a project directory");
    }

    #[tokio::test]
    async fn test_generate_class(){
        let class_name = String::from("Player");
        let parent_name = String::from("Area2D");
        generate_class(&class_name, &parent_name).await.expect("Could not create file");
        let data = fs::read_to_string("./src/player.rs").expect("Unable to read file");
        let golden = fs::read_to_string("./tests/examples/player.rs").expect("Unable to read example file");
        assert_eq!(data, golden, "Files are not the same");
        fs::remove_file("./src/player.rs").unwrap();
    }

}