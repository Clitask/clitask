use clap::{App, Arg};

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
        println!("title: {}. component: {}", matches.value_of("title").unwrap(), matches.value_of("component").unwrap());
    }
}
