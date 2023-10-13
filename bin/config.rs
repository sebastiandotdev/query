use std::fs;

pub fn config(config: String) {
  if config == "init" {
    fs::write(
      "minify.config.json",
      r#"{
            "urlBase": ""
        }"#,
    )
    .expect("Unable to write file");
  }
}
