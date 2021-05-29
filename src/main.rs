use std::fs::read_to_string;
use std::path::Path;

use clap::{App, Arg};
use serde_json::Value;

fn main() {
    let mut app = App::new("Serde Gen").
        version("1.0-SNAPSHOT").author("Wyatt Jacob Herkamp <wherkamp@kingtux.me>").about("Makes a list of data values from json").
        arg(Arg::with_name("file").short("f").long("file").value_name("fileName").help("File name").takes_value(true).required(true));
    let matches = app.get_matches();
    let path = Path::new(matches.value_of("file").unwrap());
    let result = read_to_string(path).unwrap();
    let value: Value = serde_json::from_str(result.as_str()).unwrap();
    let x = value.as_object().unwrap();
    my_loop(x);
}

fn my_loop(map: &serde_json::Map<String, Value>) {
    for x in map {
        if x.1.is_object(){
            my_loop(x.1.as_object().unwrap());
        }else if x.1.is_boolean(){
            println!("pub {}: Option<bool>,", x.0)
        }else if x.1.is_null(){
            println!("pub {}: Option<Value>,", x.0)
        }else if x.1.is_f64(){
            println!("pub {}: Option<f64>,", x.0)
        }else if x.1.is_i64(){
            println!("pub {}: Option<i64>,", x.0)
        }else if x.1.is_u64(){
            println!("pub {}: Option<u64>,", x.0)
        }else if x.1.is_string(){
            println!("pub {}: Option<String>,", x.0)
        }else if x.1.is_number(){
            println!("pub {}: Option<i64>,", x.0)
        }
    }
}
