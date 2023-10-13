use std::fs;

pub fn config(config: String) {
  if config == "init" {
    fs::write(
      "clippy.config.json",
      r#"{
            "urlBase": ""
        }"#,
    )
    .expect("Unable to write file");
  }
}
