use crate::{base_url, error::CustomError, ReadConfigQuery};
use colored::Colorize;
use std::{any::Any, io, process};

pub struct Method {
  pub get: String,
  pub post: String,
  pub put: String,
  pub delete: String,
}

impl Method {
  pub fn new(args: Method) -> Self {
    Self {
      get: args.get,
      post: args.post,
      put: args.put,
      delete: args.delete,
    }
  }
  pub async fn method_get(
    &self,
    option: &String,
    endpoint: &String,
  ) -> Result<(), CustomError> {
    if &self.get != option {
      let err = CustomError::new("Invalid command see the --help option");
      return Err(err);
    }

    let read_config = ReadConfigQuery::new().unwrap();

    let json: base_url::ConfigJSON =
      serde_json::from_str(&read_config.json).unwrap();

    let client = reqwest::Client::new();

    let res = client
      .get(format!("{}/{}", json.base_url, endpoint))
      .send()
      .await
      .unwrap_or_else(|open_err| {
        eprintln!("{}", open_err.to_string().red());
        process::exit(1);
      });

    let mut content = String::new();
    if let Ok(value) = res.text().await {
      content = value;
    }
    let parsed_json: serde_json::Value = serde_json::from_str(&content)
      .unwrap_or_else(|err| {
        eprintln!("{}", err.to_string().red());
        process::exit(1);
      });
    match parsed_json {
      serde_json::Value::Array(items) => {
        for item in items {
          let json =
            serde_json::to_string_pretty(&item).unwrap_or_else(|err| {
              eprintln!("{}", err.to_string().red());
              process::exit(1);
            });
          println!("{}", json.to_string().italic().green());
        }
      }
      _ => {
        println!("The response is not an array")
      }
    }

    Ok(())
  }
  pub async fn method_post(
    &self,
    option: &String,
    endpoint: &String,
    _data: Box<dyn Any>,
  ) -> Result<(), CustomError> {
    if &self.post != option {
      let err = CustomError::new("Invalid command see the --help option");
      return Err(err);
    }

    let json_config: ReadConfigQuery = ReadConfigQuery::new().unwrap();
    let json: base_url::ConfigJSON =
      serde_json::from_str(&json_config.json).unwrap();

    println!("Hola!; Que dato enviaras procede a escribirlos: ");
    println!("NOTA: Por favor escribe el JSON sin presionar enter");
    println!("HELP: Si deseas cancelar el prompt escribe la palabra *exit*");

    let mut value = String::new();
    loop {
      let mut input = String::new();
      io::stdin().read_line(&mut input).expect("Error read");

      if input.trim() == "exit" {
        break;
      }
      value.push_str(&input);
    }

    println!("{}", value);
    let client = reqwest::Client::new();
    let res = client
      .post(format!("{}/{}", json.base_url, endpoint))
      .body(value)
      .send()
      .await
      .unwrap_or_else(|open_err| {
        eprintln!("{}", open_err.to_string().red());
        process::exit(1);
      });

    if res.status().is_success() {
      println!("Response server: {}", res.text().await.expect("err"));
      println!("Message: {}", "created succesfully".green());
    } else {
      println!("Status: {}", res.status());
      println!("Message: {}", "Oh, something error is server".red());
    }
    Ok(())
  }

  pub fn method_delete(&self, option: &String, endpoint: &String) -> Result<(), CustomError> {
    if &self.delete != option {
      let err = CustomError::new("Invalid command see the --help option");
      return Err(err);
    }
    println!("DELETE");
    println!("{}", endpoint);
    Ok(())
  }
}
