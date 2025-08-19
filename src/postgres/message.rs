use std::{
    io::{Read, Write},
    net::TcpStream,
    num::ParseIntError,
};

const PROTOCOL: &'static str = "196608";

#[derive(Debug, PartialEq)]
pub struct Message {
    size: u32,
    protocol: u32,
    message: String,
}

impl Message {
    pub fn as_string(&self) -> String {
        let mut message: String = String::new();

        message.push_str(&self.size.to_string());
        message.push_str(&self.protocol.to_string());
        message.push_str(&self.message);

        message
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut message_bytes: Vec<u8> = Vec::new();

        message_bytes.extend(self.size.to_be_bytes());
        message_bytes.extend(self.protocol.to_be_bytes());
        message_bytes.extend(self.message.as_bytes());

        message_bytes
    }
}

#[derive(Debug, PartialEq)]
pub struct StartupMessage {
    protocol: String,
    username: String,
    database: Option<String>,
    message_size: Option<u32>,
}

impl StartupMessage {
    pub fn new(username: &str, protocol: &str) -> Self {
        Self {
            protocol: protocol.to_string(),
            username: username.to_string(),
            database: None,
            message_size: None,
        }
    }

    pub fn set_database(&mut self, db: &str) {
        self.database = Some(db.to_string());
    }

    pub fn mount_message(&mut self) -> Result<Message, ParseIntError> {
        let body_message: String = self.define_body_messsage();
        self.set_size(&body_message);

        let message: Message = Message {
            size: self.message_total_size(),
            protocol: self.protocol.parse()?,
            message: body_message,
        };

        Ok(message)
    }

    fn define_body_messsage(&mut self) -> String {
        let body_message: String = match self.database {
            Some(ref db) => format!("user\0{}\0database\0{}\0\0", self.username, db),
            None => format!("user\0{}\0database\0{}\0\0", self.username, self.username),
        };

        body_message
    }

    fn set_size(&mut self, message: &String) {
        self.message_size = Some(message.len() as u32);
    }

    fn message_total_size(&self) -> u32 {
        let total_size: u32 = self.message_size.unwrap() + 8;
        total_size
    }
}

pub fn send_start_up_message(username: &str, database: Option<&str>) {
    let mut star_up_message: StartupMessage = StartupMessage::new(username, PROTOCOL);

    match database {
        Some(db) => star_up_message.set_database(db),
        None => (),
    };

    let message = star_up_message.mount_message().unwrap().as_bytes();

    let mut stream = TcpStream::connect("localhost:5432").expect("error");
    stream.write_all(&message).unwrap();

    let mut buffer = [0u8; 1024];
    stream.read(&mut buffer).unwrap();

    println!("{:?}", buffer);
}

#[cfg(test)]
mod test_message_startup {
    use super::*;

    #[test]
    fn verify_if_the_start_up_message_start_like_expected() {
        let username = "loures";

        let start_up_message: StartupMessage = StartupMessage::new(username, PROTOCOL);
        let expected_start_up_message: StartupMessage = StartupMessage {
            protocol: PROTOCOL.to_string(),
            username: username.to_string(),
            database: None,
            message_size: None,
        };

        assert_eq!(start_up_message, expected_start_up_message);
    }

    #[test]
    fn verify_if_set_database_is_works() {
        let database = "db-postgres";
        let mut start_up_message: StartupMessage = StartupMessage::new("loures", PROTOCOL);
        start_up_message.set_database(database);

        let database: String = start_up_message.database.unwrap();
        let expected_database: String = database.to_string();

        assert_eq!(database, expected_database);
    }

    #[test]
    fn verify_if_define_body_message_is_works_when_database_isnt_defines() {
        let mut start_up_message: StartupMessage = StartupMessage::new("loures", PROTOCOL);

        let body_message: String = start_up_message.define_body_messsage();
        let expected_body_message: String = "user\0loures\0database\0loures\0\0".to_string();

        assert_eq!(body_message, expected_body_message);
    }

    #[test]
    fn verify_if_define_body_message_is_works_when_database_is_defines() {
        let mut start_up_message: StartupMessage = StartupMessage::new("loures", PROTOCOL);
        start_up_message.set_database("db-postgres");

        let body_message: String = start_up_message.define_body_messsage();
        let expected_body_message: String = "user\0loures\0database\0db-postgres\0\0".to_string();

        assert_eq!(body_message, expected_body_message);
    }

    #[test]
    fn verify_if_set_size_is_works_when_database_isnt_defines() {
        let mut start_up_message: StartupMessage = StartupMessage::new("loures", PROTOCOL);
        let message: String = "user\0loures\0database\0db-postgres\0\0".to_string();
        start_up_message.set_size(&message);

        let size: u32 = start_up_message.message_size.unwrap();
        let expected_size: u32 = message.len() as u32;

        assert_eq!(size, expected_size);
    }

    #[test]
    fn verify_if_mount_message_is_works() {
        let mut start_up_message: StartupMessage = StartupMessage::new("loures", PROTOCOL);

        let mount_message: Message = start_up_message.mount_message().unwrap();
        let expected_mount_message: Message = Message {
            size: 37,
            protocol: PROTOCOL.parse().unwrap(),
            message: "user\0loures\0database\0loures\0\0".to_string(),
        };

        assert_eq!(mount_message, expected_mount_message);
    }

    #[test]
    fn verify_if_message_as_string_is_works() {
        let protocol: u32 = PROTOCOL.parse().unwrap();

        let message: Message = Message {
            size: 37,
            protocol: protocol,
            message: "user\0loures\0database\0loures\0\0".to_string(),
        };

        let message_as_string: String = message.as_string();
        let expected_message_as_string: String =
            "37196608user\0loures\0database\0loures\0\0".to_string();

        assert_eq!(message_as_string, expected_message_as_string);
    }

    #[test]
    fn verify_if_message_as_bytes_is_works() {
        let protocol: u32 = PROTOCOL.parse().unwrap();

        let message: Message = Message {
            size: 37,
            protocol: protocol,
            message: "user\0loures\0database\0loures\0\0".to_string(),
        };

        let message_as_bytes: Vec<u8> = message.as_bytes();
        let expected_message_as_bytes: Vec<u8> = Vec::from([
            0, 0, 0, 37, 0, 3, 0, 0, 117, 115, 101, 114, 0, 108, 111, 117, 114, 101, 115, 0, 100,
            97, 116, 97, 98, 97, 115, 101, 0, 108, 111, 117, 114, 101, 115, 0, 0,
        ]);

        assert_eq!(message_as_bytes, expected_message_as_bytes);
    }
}
