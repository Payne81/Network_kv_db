extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};
use kvs::KvStore;
use std::process::exit;

fn cliprocess(kvstore: &mut KvStore) {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the value of a string key to a string")
                .version("0.1")
                .author("Payne Z. <3334248237@qq.com>")
                .arg(
                    Arg::with_name("KEY")
                        .help("Sets the string key")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("VALUE")
                        .help("Sets the string Value")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the string value of a given string key")
                .version("0.1")
                .author("Payne Z. <3334248237@qq.com>")
                .arg(
                    Arg::with_name("KEY")
                        .help("Get the string key")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove a given key")
                .version("0.1")
                .author("Payne Z. <3334248237@qq.com>")
                .arg(
                    Arg::with_name("KEY")
                        .help("Remove a given key")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("set") {
        let key = matches.value_of("KEY").unwrap().to_owned();
        let val = matches.value_of("VALUE").unwrap().to_owned();
        eprintln!("unimplemented");
        exit(1);
        kvstore.set(key, val);
    }
    if let Some(matches) = matches.subcommand_matches("get") {
        let key = matches.value_of("KEY").unwrap().to_owned();
        eprintln!("unimplemented");
        exit(1);
        match kvstore.get(key) {
            Some(val) => println!("{}", val),
            None => panic!(),
        }
    }
    if let Some(matches) = matches.subcommand_matches("rm") {
        let key = matches.value_of("KEY").unwrap().to_owned();
        eprintln!("unimplemented");
        exit(1);
        kvstore.remove(key);
    }
}

fn main() {
    let mut kvstore = KvStore::new();
    kvstore.set("key1".to_string(), "value1".to_string());
    cliprocess(&mut kvstore);
}
