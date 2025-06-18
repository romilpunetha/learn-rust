use thrift::protocol::{TBinaryInputProtocol, TBinaryOutputProtocol, TSerializable};
use thrift::transport::{TBufferChannel, TIoChannel};

pub fn serialize<T: TSerializable>(data: &T) -> thrift::Result<Vec<u8>> {
    let transport = TBufferChannel::with_capacity(0, 64);
    let (_, write) = transport.split().unwrap();
    let mut o_prot = TBinaryOutputProtocol::new(write, true);
    data.write_to_out_protocol(&mut o_prot)?;
    Ok(o_prot.transport.write_bytes())
}

pub fn deserialize<T: TSerializable>(bytes: &[u8]) -> thrift::Result<T> {
    let mut transport = TBufferChannel::with_capacity(bytes.len(), 0);
    transport.set_readable_bytes(bytes);
    let (read, _) = transport.split().unwrap();
    let mut i_prot = TBinaryInputProtocol::new(read, true);
    TSerializable::read_from_in_protocol(&mut i_prot)
}
