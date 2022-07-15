use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Stateinfo {
    pub state: String,
    pub count: u32,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
struct Runtime {
    pub path: PathBuf,
    pub stateinfo: Stateinfo,
}
impl Runtime {
    fn new(stateinfo: Stateinfo, path: PathBuf) -> Runtime {
        Runtime {
            stateinfo: stateinfo,
            path: path,
        }
    }
    fn read_json(mut self) -> Runtime {
        let jsonpath = self.path.as_path();
        let str1 = std::fs::read_to_string(jsonpath).unwrap();

        let state: Stateinfo = serde_json::from_str(&str1).unwrap();
        println!("{}    {}", state.count, state.state);

        self.stateinfo = state;
        self
    }
    fn write_json(&mut self) -> Result<()> {
        let jsonpath = self.path.as_path();

        self.stateinfo.count += 1;
        if (self.stateinfo.count % 2) == 0 {
            self.stateinfo.state = String::from("Even Runnig");
        } else {
            self.stateinfo.state = String::from("Odd Running");
        }
        let config_json = serde_json::to_string(&self.stateinfo)?;
        std::fs::write(jsonpath, &config_json).expect("error occured");
        Ok(())
    }
}
/*impl Drop for Runtime {
    fn drop(&mut self) {
        match self.write_json() {
            Err(e) => println!("{:?}", e),
            Ok(t) => println!("{:?}", t),
        }
    }
}*/
fn main() -> Result<()> {
    let path = PathBuf::from("file.json");
    let mut runtime = Runtime::new(
        Stateinfo {
            state: ("stopped".to_string()),
            count: (0),
        },
        path,
    );
    runtime = runtime.read_json();
    match runtime.write_json() {
        Err(e) => println!("{:?}", e),
        Ok(t) => println!("{:?}", t),
    }

    Ok(())
}
