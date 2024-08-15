use clap::{ArgMatches, Args, Command, FromArgMatches};

use common::{config::Config, errors::MegaResult};

use crate::server::https_server::{start_https, HttpsOptions};

pub fn cli() -> Command {
    HttpsOptions::augment_args_for_update(Command::new("https").about("Start Mega HTTPS server"))
}

pub(crate) async fn exec(config: Config, args: &ArgMatches) -> MegaResult {
    let server_matchers = HttpsOptions::from_arg_matches(args)
        .map_err(|err| err.exit())
        .unwrap();

    tracing::info!("{server_matchers:#?}");
    start_https(config, server_matchers).await;
    Ok(())
}

#[cfg(test)]
mod tests {}