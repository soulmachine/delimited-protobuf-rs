use protobuf::Message;
use simple_error::SimpleError;

/// Read one length-delimited message from the given stream.
pub fn read<MSG: Message>(reader: &mut dyn std::io::Read) -> Result<MSG, SimpleError> {
    let size = read_u32(reader).map_err(SimpleError::from)?;
    if size == 0_u32 {
        return Err(SimpleError::new("empty message"));
    }
    let mut buf = vec![0_u8; size as usize];
    reader.read_exact(&mut buf).unwrap();

    MSG::parse_from_bytes(&buf).map_err(SimpleError::from)
}

/// Write one length-delimited message to the given stream.
pub fn write<MSG: Message>(msg: &MSG, writer: &mut dyn std::io::Write) -> protobuf::Result<()> {
    msg.write_length_delimited_to_writer(writer)
}

// forked from https://github.com/amandasaurus/vartyint/blob/main/src/lib.rs#L94
fn read_u32(reader: &mut dyn std::io::Read) -> Result<u32, SimpleError> {
    let mut buf = [0_u8; 1];
    let mut num_bits_read = 0;
    let mut val: u32 = 0;
    let mut is_last: bool;
    let mut byte: u32;

    loop {
        reader.read_exact(&mut buf).map_err(SimpleError::from)?;
        byte = buf[0] as u32;

        is_last = byte >> 7 == 0;
        byte &= 0b0111_1111;

        byte = match byte.checked_shl(num_bits_read) {
            None => {
                return Err(SimpleError::new("too many bytes for u32"));
            }
            Some(v) => v,
        };
        val |= byte;
        num_bits_read += 7;
        if is_last {
            // last byte
            break;
        }
    }

    Ok(val)
}
