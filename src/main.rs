use clap::{App, Arg};

struct Task {
    // id: u8,
    title: std::string::String,
    component: std::string::String,
    // description: String,
    // priority: u8,
    // tags: [String],
}

fn main() {
    let app = App::new("clitask").version("0.1.0").subcommand(
        App::new("add")
            .about("Adds tasks")
            .version("0.1.0")
            .arg(
                Arg::with_name("title")
                    .long("title")
                    .short("--t")
                    .takes_value(true)
                    .required(true)
            )
            .arg(
                Arg::with_name("component")
                    .long("component")
                    .short("--c")
                    .takes_value(true)
            ),
    )
    .get_matches();

    if let Some(matches) = app.subcommand_matches("add") {
        let task = Task { title: matches.value_of("title").unwrap().to_string(), component: matches.value_of("component").unwrap().to_string()};
        println!("title: {}. component: {}", task.title, task.component);
    }
}
