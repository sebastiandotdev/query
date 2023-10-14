use std::fs;

pub fn config(config: String) {
  let json_config = r#"{
    "name": "minify",
    "urlBase": "",
    "method": []
  }"#;

  if config == "init" {
    fs::write("minify.json", json_config).expect("Unable to write file");
  }
}
