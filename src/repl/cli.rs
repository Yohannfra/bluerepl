use clap::{arg, Arg, Command};

pub fn cli() -> Command<'static> {
    // strip out usage
    const PARSER_TEMPLATE: &str = "\
        {all-args}
    ";
    // strip out name/version
    const COMMAND_TEMPLATE: &str = "\
        {about-with-newline}\n\
        {usage-heading}\n    {usage}\n\
        \n\
        {all-args}{after-help}\
    ";

    Command::new("repl")
        .multicall(true)
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand_value_name("COMMAND")
        .subcommand_help_heading("COMMANDS")
        .help_template(PARSER_TEMPLATE)

        // clear
        .subcommand(
            Command::new("clear")
            .about("Clear the terminal")
            .help_template(COMMAND_TEMPLATE))

        // quit
        .subcommand(
            Command::new("quit")
            .alias("exit")
            .about("Quit the REPL")
            .help_template(COMMAND_TEMPLATE))

        // write
        .subcommand(
            Command::new("write")
            .about("Write a value to a characteristic")
            .args(&[
                arg!(-n --noresp ... "Write no response (default write is write with response)"),
                Arg::new("service").help("The service that contains the characteristic to write").required(true),
                Arg::new("characteristic").help("The characteristic to write").required(true),
                Arg::new("payload").help("The payload to write").required(true)
            ]).help_template(COMMAND_TEMPLATE))

        // read
        .subcommand(
            Command::new("read")
            .about("read the value of a characteristic")
            .args(&[
                Arg::new("service").help("The service that contains the characteristic to write").required(true),
                Arg::new("characteristic").help("The characteristic to write").required(true),
            ]).help_template(COMMAND_TEMPLATE))

        // scan
        .subcommand(
            Command::new("scan")
            .about("Search for BLE devices around")
            .args(&[
                Arg::new("timeout").help("Time to scan in seconds").required(false).default_value("5"),
                arg!(-a --all ... "Show unnamed peripheral"),
                arg!(-l --list ... "Show last scan list (doesn't run a new scan)"),
            ]).help_template(COMMAND_TEMPLATE))

        // info
        .subcommand(
            Command::new("info")
            .about("print informations about topic")
            .args(&[
                Arg::new("adapter").help("Print informations about BLE adapter").required(true).exclusive(true),
                Arg::new("service").help("Print informations about connected peripheral").required(true).exclusive(true),
                Arg::new("preset").help("Print informations about loaded preset").required(true).exclusive(true),
            ]).help_template(COMMAND_TEMPLATE))

        // connect
        .subcommand(
            Command::new("connect")
            .about("Connect to a BLE peripheral")
            .args(&[
                arg!(-n --name ... "Connection using the name of the peripheral").takes_value(true).exclusive(true).required(true),
                arg!(-m --mac ... "Connection using the mac address of the peripheral").takes_value(true).exclusive(true).required(true),
                arg!(-i --id ... "Connection using the id of the peripheral in the scan list").takes_value(true).exclusive(true).required(true),
            ]).help_template(COMMAND_TEMPLATE))

        // disconnect
        .subcommand(
            Command::new("disconnect")
            .about("Disconnect from BLE peripheral")
            .help_template(COMMAND_TEMPLATE))

        // indicate
        .subcommand(
            Command::new("indicate")
            .about("Subscribe to a characteristic indications and print it's value when it gets updated")
            .args(&[
                Arg::new("service").help("The service that contains the characteristic to subscribe to").required(true),
                Arg::new("characteristic").help("The characteristic to subscribe to").required(true),
            ]).help_template(COMMAND_TEMPLATE))

        // notify
        .subcommand(
            Command::new("notify")
            .about("Subscribe to a characteristic notifications and print it's value when it gets updated")
            .args(&[
                Arg::new("service").help("The service that contains the characteristic to subscribe to").required(true),
                Arg::new("characteristic").help("The characteristic to subscribe to").required(true),
            ]).help_template(COMMAND_TEMPLATE))

        // unsubscribe
        .subcommand(
            Command::new("unsubscribe")
            .about("Unsubscribe from the notifications or indications of a characteristic")
            .args(&[
                Arg::new("service").help("The service that contains the characteristic to unsubscribe from").required(true),
                Arg::new("characteristic").help("The characteristic to unsubscribe from").required(true),
            ]).help_template(COMMAND_TEMPLATE))
}
