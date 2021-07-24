use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(version = "1.0", author = "Ran Z. <rantzvi@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    /// The address to connect to
    #[clap(short, long, default_value="127.0.0.1")]
    pub ip_address: String,

    /// The file name to open an ssh into
    #[clap(short, long)]
    pub file_path: String,

    /// The username to log into
    #[clap(short , long)]
    pub user_name: String,

    /// Password to use, leave empty for agent authentication, fill for password authentication
    #[clap(short, long)]
    pub password: Option<String>
}