//use serde::{Deserialize, Serialize};
extern crate yaml_rust;
//use std::collections::HashMap;
use yaml_rust::Yaml;
/*
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct DateOfBirth {
    date: u32,
    month: u32,
    year: u32,
}
impl DateOfBirth {
    fn new(date: u32, month: u32, year: u32) -> DateOfBirth {
        DateOfBirth {
            date: date,
            month: month,
            year: year,
        }
    }
}*/
use std::env;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct ServerConfig {
    workers: u64,
    ignore: bool,
    auth_server: Option<String>,
}
fn Serialdeserial() {
    let config = ServerConfig {
        workers: 100,
        ignore: false,
        auth_server: Some(String::from("auth.server.io")),
    };

    println!("");
    println!("");

    {
        println!("To and from Yaml");
        let serialized = serde_yaml::to_string(&config).unwrap();
        println!("serialized: {}", serialized);

        println!("");
        let deserialized: ServerConfig = serde_yaml::from_str(&serialized).unwrap();
        println!("deserialized: {:#?}", deserialized);
    }
}
const DOC: &str = r#"namespaces:
  production:

helmRepos:
  stable: "https://kubernetes-charts.storage.googleapis.com"

context: monitoring

apps:
  datadog:
    enabled: true
    namespace: production
    chart: stable/datadog
    version: "1.38.7"
    valuesFile: "values.yaml"
"#;

fn fun() {
    let mut cur_config_dir = std::env::current_dir().unwrap_or(PathBuf::from("."));
    cur_config_dir.push("src/config.yml");

    let mut value: serde_yaml::Value = serde_yaml::from_str(DOC).unwrap();
    *value
        .get_mut("apps")
        .unwrap()
        .get_mut("datadog")
        .unwrap()
        .get_mut("version")
        .unwrap() = "1.38.0".into();

    std::fs::write(
        cur_config_dir,
        serde_yaml::to_string_pretty(&value).unwrap(),
    ).expect("error");
    // serde_yaml::to_writer(std::io::stdout(), &value).unwrap();
}
#[derive(Debug, serde::Deserialize, PartialEq)]
struct Build {
    details: Vec<Foo>,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
#[serde(untagged)]
enum Foo {
    Step(Step),
}
#[derive(Debug, serde::Deserialize, PartialEq)]
struct Step {
    name: String,
    date: u32,
}

fn main() -> Result<(), serde_yaml::Error> {
    let doc = std::fs::read_to_string("src/config.yml").expect("error while reading");
    println!("{:#?}\n", doc);
    let data: Build = serde_yaml::from_str(&doc)?;
    println!("{:?}\n", data);
    //let hash = &data[0];
    //println!("{:?}\n", hash);

    //let map = hash.as_hash().unwrap();
    //println!("{:?}\n", map);

    /*
    for hash in data.iter() {
        //let hash = &data[0];
        //println!("{:?}\n", hash);

        let map = hash.as_hash().unwrap();
        //println!("{:?}\n", map);

        for (k, v) in map.iter() {
            println!("{}: ", k.as_str().unwrap());
            unwrap_value(v);
            println!("\n");
        }
    }*/
    //let newdate = DateOfBirth::new(08, 17, 1996);
    //println!("{:?}", newdate);
    fun();
    Serialdeserial();
    Ok(())
}
fn _unwrap_value(v: &Yaml) {
    match v {
        Yaml::Hash(v) => {
            for (k, vv) in v.iter() {
                match vv {
                    Yaml::Hash(vv) => {
                        println!("{:?}", vv)
                    }
                    Yaml::String(vv) => {
                        print!("{} =  {}\n", k.as_str().unwrap(), vv.as_str());
                    }
                    Yaml::Integer(vv) => {
                        print!("{} =  {}\n", k.as_str().unwrap(), vv);
                    }

                    Yaml::Array(vv) => {
                        for h in vv.iter() {
                            _unwrap_value(h);
                        }
                    }
                    _ => {
                        print!("\t empty {}\n", k.as_str().unwrap());
                    }
                }
            }
        }
        _ => todo!(),
    }
}
