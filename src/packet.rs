use super::error::{RenetError, Result};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

pub const FRAGMENT_MAX_COUNT: usize = 256;
pub const FRAGMENT_MAX_SIZE: usize = 1024;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PacketType {
    Packet = 0,
    Fragment = 1,
}

pub trait HeaderParser {
    type Header;

    fn parse(reader: &[u8]) -> Result<Self::Header>;
    fn write(&self, writer: &mut [u8]) -> Result<()>;

    /// Header size in bytes
    fn size() -> usize;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PacketHeader {
    // protocol_id: u16,
    // crc32: u32, // append protocol_id when calculating crc32
    pub sequence: u16,
}

impl HeaderParser for PacketHeader {
    type Header = Self;

    fn size() -> usize {
        3
    }

    fn write(&self, mut buffer: &mut [u8]) -> Result<()> {
        buffer.write_u8(PacketType::Packet as u8)?;
        buffer.write_u16::<BigEndian>(self.sequence)?;
        Ok(())
    }

    fn parse(mut reader: &[u8]) -> Result<Self> {
        let packet_type = reader.read_u8()?;
        if packet_type != PacketType::Packet as u8 {
            return Err(RenetError::InvalidHeaderType);
        }
        let sequence = reader.read_u16::<BigEndian>()?;

        let header = PacketHeader { sequence };

        Ok(header)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FragmentHeader {
    // crc32: u32,
    pub sequence: u16,
    pub fragment_id: u8,
    pub num_fragments: u8,
}

impl HeaderParser for FragmentHeader {
    type Header = Self;

    fn size() -> usize {
        5
    }

    fn write(&self, mut writer: &mut [u8]) -> Result<()> {
        writer.write_u8(PacketType::Fragment as u8)?;
        writer.write_u16::<BigEndian>(self.sequence)?;
        writer.write_u8(self.fragment_id)?;
        writer.write_u8(self.num_fragments)?;
        Ok(())
    }

    fn parse(mut reader: &[u8]) -> Result<Self> {
        let packet_type = reader.read_u8()?;
        if packet_type != PacketType::Fragment as u8 {
            return Err(RenetError::InvalidHeaderType);
        }
        let sequence = reader.read_u16::<BigEndian>()?;
        let fragment_id = reader.read_u8()?;
        let num_fragments = reader.read_u8()?;

        let header = FragmentHeader {
            sequence,
            fragment_id,
            num_fragments,
        };

        Ok(header)
    }
}

mod tests {
    use super::*;

    #[test]
    fn fragment_header_read_write() {
        let fragment_header = FragmentHeader {
            sequence: 42,
            fragment_id: 3,
            num_fragments: 5,
        };

        let mut buffer = vec![0u8; FragmentHeader::size()];

        fragment_header.write(&mut buffer).unwrap();

        let parsed_fragment_header = FragmentHeader::parse(&mut buffer).unwrap();
        assert_eq!(fragment_header, parsed_fragment_header);
    }

    #[test]
    fn packet_header_read_write() {
        let header = PacketHeader { sequence: 42 };

        let mut buffer = vec![0u8; PacketHeader::size()];

        header.write(&mut buffer).unwrap();

        let parsed_header = PacketHeader::parse(&mut buffer).unwrap();
        assert_eq!(header, parsed_header);
    }
}