use std::fs::File;
use std::io::{Error, Read, BufReader, BufRead};
use std::path::{Path, PathBuf};
use ssh2::Channel;
use crate::utils::client::Client;
use crate::utils::read_stdin;

pub struct SshFile {
    file_path: PathBuf,
    client: Client
}

impl SshFile {
    pub fn new(file_name: &str, client: Client) -> Self {
        let file_path = PathBuf::from(file_name);
        SshFile::create_file(&file_path).unwrap();

        SshFile {
            file_path,
            client
        }
    }

    pub fn run(&mut self) -> Result<i32, Error> {
        let f = File::open(&self.file_path)?;
        let reader = BufReader::new(f);
        self.read_and_execute(reader)?;
        Ok(0)
    }

    fn read_and_execute(&mut self, buffer: BufReader<File>) -> Result<(), Error> {
        let outputs: Vec<Result<String, Error>> = self.get_lines_from_buffer(buffer)
        .iter()
        .map(|l| {
            let mut channel = self.client.get_channel()?;
            let out = self.execute_command(&l, &mut channel);
            SshFile::kill_channel(&mut channel);
            out
        })
        .collect();

        for output in outputs.into_iter() {
            let output = output.unwrap();
            print!("{}", output);
        }
        self.delete_file_contents()?;
        Ok(())
    }

    fn get_lines_from_buffer(&self, buffer: BufReader<File>) -> Vec<String> {
        buffer
        .lines()
        .filter_map(|l| match l.unwrap().trim() {
                "" => None,
                x => Some(x.into())
            }
        )
        .collect()
    }

    fn delete_file_contents(&self) -> Result<(), Error> {
        File::create(&self.file_path)?;
        Ok(())
    }

    fn create_file(file_path: &Path) -> std::io::Result<File> {
        if file_path.exists() {
            let err_message = format!("File {f_name:?} already exists", f_name=file_path.to_str().unwrap());
            let user_input = read_stdin(&format!("{} {}", &err_message, "Override? y/n"))?;
            if &user_input != "y" {
                panic!("{}", err_message);
            }
        }
        Ok(File::create(file_path)?)
    }

    fn execute_command(&mut self, command: &str, channel: &mut Channel) -> Result<String, Error> {
        channel.exec(command)?;
        let mut output = String::new();
        channel.read_to_string(&mut output)?;
        Ok(output)
    }

    fn kill_channel(channel: &mut Channel) -> () {
        channel.wait_close().unwrap();
    }
}

impl std::ops::Drop for SshFile {
    fn drop(&mut self) {
        match std::fs::remove_file(&self.file_path) {
            Ok(_) => (),
            Err(err) => panic!("{}", err)
        }
    }
}