use std::vec;

use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup_flox() -> Result<(), Error> {
    let os = dag().get_os()?;
    if os == "macos" {
        dag()
      .pipeline("setup-flox")?
      .with_exec(vec![r#"type brew > /dev/null 2> /dev/null || /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)""#])?
      .with_exec(vec!["type flox > /dev/null 2> /dev/null || brew install flox"])?
      .stdout()?;
    }
    Ok(())
}

pub fn setup_capistrano() -> Result<String, Error> {
    let home = dag().get_env("HOME")?;
    let path = dag().get_env("PATH")?;

    setup_flox()?;
    let stdout = dag()
        .pipeline("setup capistrano")?
        .flox()?
        .with_exec(vec!["flox install ruby"])?
        .with_exec(vec!["gem install capistrano"])?
        .with_exec(vec!["[ -d $HOME/.local/bin ] || mkdir -p $HOME/.local/bin"])?
        .with_exec(vec![
            "ln -s `flox activate -- gem environment gemhome`/bin/cap $HOME/.local/bin/cap || true",
        ])?
        .with_exec(vec!["PATH=$HOME/.local/bin:$PATH", "type", "cap"])?
        .with_exec(vec!["type", "ruby"])?
        .stdout()?;

    dag().set_envs(vec![(
        "PATH".into(),
        format!("{}/.local/bin:{}", home, path),
    )])?;

    Ok(stdout)
}
