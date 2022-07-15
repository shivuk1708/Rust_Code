use std::env;
pub fn cli_fun() {
    let args: Vec<String> = env::args().collect();

    let command = args[1].clone();
    println!("{:?}", args);
    println!("command {}", command);
    let name = "Shivakumar";
    let status = "100%";
    if command == "hello" {
        println!("Hi {} How are you?", name)
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("Invalind command");
    }
}
fn wrtie_components(components: &Vec<String>, adas247_configuration: &Config) {
    for element in components.iter() {
        println!("{} ", *element);
        let str1: String = String::from(element);
        println!(
            "{:?}",
            adas247_configuration
                .components
                .as_ref()
                .unwrap()
                .get(&str1)
        );
    }
    let w = File::create("/home/shivakumar/AIcore/adas2472./component_graph.txt").unwrap();

    // Iterae over everything.
    for element in components.iter() {
        let str1: String = String::from(element);
        writeln!(&w, "components = {} ", element);

        //writeln!(&w, "components of {} are {:?}", element,  adas247_configuration.components.as_ref().unwrap().get(&str1)).expect("error while writing");
        let pubstruct = adas247_configuration
            .components
            .as_ref()
            .unwrap()
            .get(&str1);
        if pubstruct.is_some() {
            let pubdata = &pubstruct.unwrap().publish;
            for element in pubdata {
                for (identifier, topic) in element {
                    writeln!(&w, " publish = {} topic {}", identifier, topic);
                }
            }
            let subdata = &pubstruct.unwrap().subscribe;
            let publish_data = subdata.unwrap().publish;
            let subscribe_data = subdata.unwrap().subscribe;
        } else {
            println!("*****************************************");
        }
    }
}
