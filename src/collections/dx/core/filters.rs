use tera::Result;
use tera::Value;
use tera::Function;
use tera;
use std::collections::HashMap;


pub fn make_url_for(url: String) -> impl Function {
    Box::new(move |args: &HashMap<String, Value>| -> Result<Value> {
        match args.get("name") {
            Some(val) => match tera::from_value::<String>(val.clone()) {
                Ok(v) =>  Ok(tera::to_value(url.to_string()).unwrap()),
                Err(_) => Err("oops".into()),
            },
            None => Err("oops".into()),
        }
    })
}


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