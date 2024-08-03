use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn setup(_args: String) -> FnResult<String> {
    let stdout = helpers::setup_capistrano()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn deploy(args: String) -> FnResult<String> {
    helpers::setup_capistrano()?;
    let args = args.split_whitespace().collect::<Vec<&str>>();
    let environment = args.get(0).unwrap_or(&"production");
    let options = args[1..].join(" ");

    let stdout = dag()
        .flox()?
        .with_exec(vec!["cap", environment, "deploy", &options])?
        .stdout()?;

    Ok(stdout)
}
