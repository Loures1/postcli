use super::frontend;
use crate::postgres::buffer::MutBytes;

#[test]
fn startup_message_its_works() {
    let mut buf: MutBytes = MutBytes::new();
    let expected_buf = vec![
        0, 0, 0, 37, 0, 3, 0, 0, 117, 115, 101, 114, 0, 108, 111, 117, 114, 101, 115, 0, 100, 97,
        116, 97, 98, 97, 115, 101, 0, 108, 111, 117, 114, 101, 115, 0, 0,
    ];

    let params = [("user", "loures"), ("database", "loures")];

    frontend::startup_message(params, &mut buf).expect("error startup_message");

    assert_eq!(buf.as_slice(), expected_buf);
}

#[test]
#[should_panic(expected = "string contains embedded null")]
fn startup_message_cant_receive_params_with_null() {
    let mut buf: MutBytes = MutBytes::new();
    let params = [("user", "loures"), ("database", "ag\0ro")];

    frontend::startup_message(params, &mut buf).expect("error startup_message");
}
