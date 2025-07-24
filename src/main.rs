use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    function: String,

    #[arg(short, long)]
    source: String,

    #[arg(short, long)]
    pattern: String,
}

fn main() {
    let args = Args::parse();
    match args.function {
        // for each match run function
        // if missing param, send err
        _ => todo!(),
    }
    // let matches = App::new("cli_utils").get_matches();
}

fn echo(text: &str) {
    assert!(text.len() > 0);
    println!("{text}");
}

fn echo_from_vec(text: Vec<&str>) {
    for t in text {
        println!("{t}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_echo_test() {
        echo("Hello, World!");
    }
    #[test]
    fn long_echo_test() {
        echo("Hello, World! 1edfc4f8-717f-4207-bc09-4c8c71dac8e2 24c16597-cb3e-474b-8983-0dbd1a453ff6 c03f0c49-b5fb-4f6b-8a98-1ad64cc52180 0b05130f-c164-46d1-a9ad-0075b69a3571 c10a2589-f5b6-4928-bbe0-0d455459f106 0de8191c-e775-46d5-9a71-4bf4b25caa01");
    }
    #[test]
    fn string_vec_echo_test() {
        echo_from_vec(vec!["1", "2", "3"]);
    }
    #[test]
    fn five_consecutive_echo_test() {
        for num in 1..5 {
            echo(format!("Hello, World! {}", num).as_str());
        }
    }
}
