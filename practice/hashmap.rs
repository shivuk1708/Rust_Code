use std::collections::HashMap;

#[warn(unused_imports)]
use std::{
    fs::File,
    io::{Write},
};

#[repr(u8)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum SampleType {
    CuboidList,
    Egomotion,
}
#[derive(Hash, Eq, PartialEq, Debug)]
struct PubSubData {
    publish: String,
    subscribe: Vec<SubscribeData>,
}

impl PubSubData {
    /// Creates a new componets.
    fn new(publisher: &str, subscriber: Vec<SubscribeData>) -> PubSubData {
        PubSubData {
            publish: publisher.to_string(),
            subscribe: subscriber,
        }
    }
}
#[derive(Hash, Eq, PartialEq, Debug)]
struct SubscribeData {
    topic: String,
    dtype: Option<SampleType>,
}
impl SubscribeData {
    /// Creates a new componets.
    fn new(topic: &str, dtype: Option<SampleType>) -> SubscribeData {
        SubscribeData {
            topic: topic.to_string(),
            dtype: dtype,
        }
    }
}
fn main() {
    //let mut componets = HashMap::new();

    let componets = HashMap::from([
        (
            "vse",
            PubSubData::new(
                "egomotion",
                vec![SubscribeData::new("egomotion", Some(SampleType::Egomotion))],
            ),
        ),
        (
            "e2e_preproc",
            PubSubData::new(
                "network_input",
                vec![SubscribeData::new("egomotion", Some(SampleType::Egomotion))],
            ),
        ),
        (
            "e2e-tracker",
            PubSubData::new(
                "tracker",
                vec![
                    SubscribeData::new("e2e", Some(SampleType::CuboidList)),
                    SubscribeData::new("egomotion", Some(SampleType::Egomotion)),
                ],
            ),
        ),
        (
            "aip",
            PubSubData::new(
                "aipilot",
                vec![SubscribeData::new("tracker", Some(SampleType::CuboidList))],
            ),
        ),
        (
            "harp",
            PubSubData::new(
                "harp",
                vec![
                    SubscribeData::new("network_input", None),
                    SubscribeData::new("network_output", None),
                    SubscribeData::new("egomotion", None),
                ],
            ),
        ),
    ]);

    let w = File::create("/home/shivakumar/Rust/practice/test.txt").unwrap();

    // Iterae over everything.
    for (compo, val) in &componets {
        println!("{} \"{:?}\"", compo, val);
        writeln!(&w, "components of {} {:?}", compo, val).expect("error while writing");
    }

    
}
