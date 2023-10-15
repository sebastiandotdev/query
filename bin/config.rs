use std::fs;

pub struct CreateConfig;

impl CreateConfig {
  pub fn new() -> CreateConfig {
    CreateConfig
  }
  pub fn config(&self, config: String) {
    let json_config = r#"{
      "name": "minify",
      "urlBase": "",
      "method": []
    }"#;

    if config == "init" {
      fs::write("minify.json", json_config).expect("Unable to write file");
    }
  }
}
