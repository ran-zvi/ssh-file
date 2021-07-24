use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use clap::Clap;

pub mod client;
pub mod file;
pub mod cli;

pub fn read_stdin(output: &str) -> Result<String, std::io::Error> {
    println!("{}", output);
    let mut s = String::new();
    match std::io::stdin().read_line(&mut s) {
        Ok(_) => Ok(s.trim().into()),
        Err(err) => panic!("{}", err)
    }
}

pub fn execute_program() -> Result<(), Box<dyn std::error::Error>> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let opts = cli::Opts::parse();

    let mut ssh_client = client::Client::new(&opts.ip_address);

    match opts.password {
        None => &ssh_client.authenticate_agent(&opts.user_name),
        Some(p) => &ssh_client.authenticate_password(&opts.user_name, &p)
    };
    let mut ssh_file = file::SshFile::new(&opts.file_path, ssh_client);

    while running.load(Ordering::SeqCst) {
        ssh_file.run()?;
    }
    println!("Exited!");
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_client() -> client::Client {
        client::Client::new("127.0.0.1")   
    }

    #[test]
    fn test_client() {
        let ssh_client = get_test_client();
        assert!(!ssh_client.is_auth());
    }

    #[test]
    #[should_panic]
    fn test_panic_get_channel_not_authenticated() {
        let mut ssh_client = get_test_client();
        ssh_client.get_channel().unwrap();
    }
}