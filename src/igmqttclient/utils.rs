use bytes::{BytesMut, BufMut};
use tokio::{io::{ReadHalf, AsyncReadExt}, net::TcpStream};
use tokio_rustls::client::TlsStream;

/// MQTT 3.1.1 Spec string byte encoding
pub fn write_str(value: &str, writer: &mut BytesMut) {
    let length: u16 = value.len() as u16;
    writer.put_u16(length);
    writer.put_slice(value.as_bytes());
}

/// MQTT 3.1.1 Spec Variable Length byte encoding
pub fn write_variable_length_encoding(value: usize, writer: &mut BytesMut) {
    assert!(
        value <= 268_435_455,
        "value exceeds maximum allowed: 268435455"
    );

    let mut x = value;
    while {
        let mut encoded_byte: u8 = (x % 128) as u8;
        x /= 128;

        if x > 0 {
            encoded_byte |= 128;
        }

        writer.put_u8(encoded_byte);

        x > 0
    } {}
}

/// MQTT 3.1.1 Spec Variable Length decoding
pub async fn read_variable_length_encoding(
    stream: &mut ReadHalf<TlsStream<TcpStream>>,
) -> Result<usize, std::io::Error> {
    let mut multipler: usize = 1;
    let mut value: usize = 0;

    while {
        let encoded_byte: u8 = stream.read_u8().await?;
        value += Into::<usize>::into(encoded_byte & 127) * multipler;
        multipler *= 128;

        if multipler > 128 * 128 * 128 {
            panic!("Expected nonmalformed variable length encoding!");
        }

        (encoded_byte & 128) != 0
    } {}

    Ok(value)
}
