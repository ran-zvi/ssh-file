use ssh2::{Session, Channel, ErrorCode, Error};
use std::net::{TcpStream};

pub struct Client {
    session: Session,
    is_auth: bool
}

impl Client {
    pub fn new(address: &str) -> Self {
        let tcp = TcpStream::connect(format!("{addr}:22", addr=address)).unwrap();
        let mut session = Session::new().unwrap();
        session.set_tcp_stream(tcp);
        session.handshake().unwrap();

        Client {
            session,
            is_auth: false
        }
    }

    pub fn session(&self) -> &Session {
        &self.session
    }

    pub fn is_auth(&self) -> bool {
        self.is_auth
    }

    pub fn authenticate_agent(&mut self, username: &str) -> () {
        self.session.userauth_agent(username).unwrap();
        self.is_auth = true;
    }

    pub fn authenticate_password(&mut self, username: &str, password: &str) -> () {
        self.session.userauth_password(username, password).unwrap();
        self.is_auth = true;
    }

    pub fn get_channel(&mut self) -> Result<Channel, Error> {
        if !self.is_auth {
            return Err(Error::new(ErrorCode::Session(-18), "Must authenticate before requesting channel"));
        }
        self.session.channel_session()
    }
}