use tera::Result;
use tera::Value;
use tera::Function;
use tera;
use std::collections::HashMap;
// use base64::{Engine as _, engine::{self, general_purpose}, alphabet};
use base64::{Engine as _, engine::general_purpose};


// functions to implement: current_time, current_path, env_var

pub fn current_time() -> impl Function {
    let current_time = chrono::Utc::now();
    Box::new(move |_: &HashMap<String, Value>| -> Result<Value> {
        Ok(tera::to_value(current_time).unwrap())
    })
}

pub fn env_var() -> impl Function {
    Box::new(move |args: &HashMap<String, Value>| -> Result<Value> {
        match args.get("name") {
            Some(val) => match tera::from_value::<String>(val.clone()) {
                Ok(v) => match std::env::var(v) {
                    Ok(val) => Ok(tera::to_value(val).unwrap()),
                    Err(_) => Err("oops".into()),
                },
                Err(_) => Err("oops".into()),
            },
            None => Err("oops".into()),
        }
    })
}

// pub fn make_url_for(url: String) -> impl Function {
//     Box::new(move |args: &HashMap<String, Value>| -> Result<Value> {
//         match args.get("name") {
//             Some(val) => match tera::from_value::<String>(val.clone()) {
//                 Ok(val) =>  Ok(tera::to_value(url.to_string()).unwrap()),
//                 Err(_) => Err("oops".into()),
//             },
//             None => Err("oops".into()),
//         }
//     })
// }


pub fn filter1(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    Ok(value.clone())
}

pub fn filter2(value: &Value, args: &HashMap<String, Value>) -> Result<Value> {
    let current_str = tera::try_get_value!("filter2", "value", String, value);
    let name = match args.get("name") {
        Some(val) => tera::try_get_value!("filter2", "name", String, val),
        None => "common".to_string(),
    };
    Ok(tera::to_value(format!("{}-{}", current_str, name)).unwrap())
}

pub fn as_yaml(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let current_object = tera::try_get_value!("as_yaml", "value", tera::Value, value);
    let yaml_str = serde_yaml::to_string(&current_object).unwrap();
    Ok(tera::to_value(yaml_str).unwrap())
}


pub fn as_json(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let current_object = tera::try_get_value!("as_json", "value", tera::Value, value);
    let json_str = serde_json::to_string(&current_object).unwrap();
    // transform json_str to a replace " with '
    let yaml_str = json_str.replace("\"", "'");
    Ok(tera::to_value(yaml_str).unwrap())
}


pub fn as_base64(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let current_object = tera::try_get_value!("as_json", "value", tera::Value, value);
    let json_str = serde_json::to_string(&current_object).unwrap();
    // transform to a base64 string
    let b64 = general_purpose::STANDARD.encode(json_str);
    //println!("{}", b64);
    Ok(tera::to_value(b64).unwrap())
}