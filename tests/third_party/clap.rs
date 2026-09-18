use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    input: PathBuf,

    #[arg(short, long, default_value_t = 1)]
    count: u32,

    #[arg(short, long)]
    verbose: bool,
}

fn run(cli: &Args) {
    if cli.verbose {
        eprintln!("running with count={}", cli.count);
    }
    println!("file: {:?}", cli.input);
}

#[test]
fn basic() {
    let args = Args::try_parse_from(["myprog", "test.txt"]).unwrap();
    run(&args);
    assert_eq!(args.input, PathBuf::from("test.txt"));
    assert_eq!(args.count, 1); // 默认值
    assert!(!args.verbose);
}

#[test]
fn shot() {
    let args = Args::try_parse_from(["myprog", "data.log", "-c", "5", "-v"]).unwrap();
    run(&args);
    assert_eq!(args.input, PathBuf::from("data.log"));
    assert_eq!(args.count, 5);
    assert!(args.verbose);
}

#[test]
#[should_panic]
fn missing_input() {
    let _args = Args::try_parse_from(["myprog"]).unwrap();
}
