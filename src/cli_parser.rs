/// Parse the command line parameters with help of clap.
pub fn parse_cli_parameters<'p>(program_name: &str, program_version: &str) -> clap::ArgMatches<'p> {
    let arg_matches = clap::App::new(program_name)
        .version(program_version)
        .author("Sven Putze <github@hardcode.de>")
        .about("Wallbox control service.")
        .arg(
            clap::Arg::with_name("configfile")
                .short("-conf")
                .long("--config-file")
                .value_name("json configuration file")
                .help("json file with the configuration of the webservice")
                .takes_value(true)
                .required(true),
        ).after_help(
r##"Here is an example of a working configuration file:

{
    "web_bind_address": "127.0.0.1::8844",
    "ssl_private_key_file": "conv.dev/wb-selfsigned.key",
    "ssl_certificate_chain_file": "conv.dev/wb-selfsigned-cert.pem",
    "get_relay_state_command": "conv.dev/bin/get-relay-state.sh",
    "set_relay_state_command": "conv.dev/bin/set-relay-state.sh",
    "user_id": "mobilephone",
    "password": "secret"
}"##)
        .get_matches();
        arg_matches
}
