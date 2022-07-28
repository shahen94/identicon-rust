use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
  #[structopt(long = "usename", short = "u")]
  pub username: String
}
