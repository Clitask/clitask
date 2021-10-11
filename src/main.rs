use clap::{Arg, App};

fn main() {
    
let app = App::new("rustask")
    .version("1.0");
    // Define the name command line option
    let title = Arg::with_name("title")
        .long("title") // allow --name
        .short("--t")
        .takes_value(true)
        .required(true);

    // now add in the argument we want to parse
    let app = app.arg(title);

    let component = Arg::with_name("component")
        .long("component") // allow --name
        .short("--c")
        .takes_value(true)
        .required(true);

    // now add in the argument we want to parse
    let app = app.arg(component);

    // extract the matches
    let matches = app.get_matches();

    // Extract the actual name
    let title_name = matches.value_of("title")
        .expect("This can't be None, we said it was required");
    
    let component_name = matches.value_of("component")
        .expect("This can't be None, we said it was required");

    println!("title: {}. component: {}", title_name, component_name);

}
