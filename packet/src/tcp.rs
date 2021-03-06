

#[derive(Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Flag {
    NS,
    CWR,
    ECE,
    URG,
    ACK,
    PSH,
    RST,
    SYN,
    FIN
}

#[derive(Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Operation {
    LISTEN,
    SYN_SENT,
    SYN_RECEIVED,
    ESTABLISHED,
    FIN_WAIT_1,
    FIN_WAIT_2,
    CLOSE_WAIT,
    CLOSING,
    LAST_ACK,
    TIME_WAIT,
    CLOSED
}

/// Connection establishment
/// 
/// * SYN
/// * SYN-ACK
/// * ACK
/// 
/// https://en.wikipedia.org/wiki/Transmission_Control_Protocol#TCP_segment_structure
#[derive(Debug, PartialEq, Eq)]
pub struct Packet<'a> {
    src_port: u16,              // 16 bits
    dst_port: u16,              // 16 bits
    sequence_number: u32,       // 32 bits
    acknowledgment_number: u32, // 32 bits , if ACK set
    data_offset: u8,            //  4 bits
    reserved: u8,               //  3 bits
    flags   : u16,              //  9 bits, NS/CWR/ECE/URG/ACK/PSH/RST/SYN/FIN
    window_size: u16,           // 16 bits
    checksum: u16,              // 16 bits
    urgent_pointer: u16,        // 16 bits , if URG set
    options: Option<Vec<u8>>,   // .. bits , if data offset > 5. Padded at the end with "0" bytes if necessary
    payload: &'a [u8],          // .. bits
}

impl <'a>Packet<'a> {
    #[allow(unused_variables)]
    pub fn from_bytes(payload: &[u8]) -> Result<Self, ::std::io::Error> {
        unimplemented!();
    }
    
    pub fn as_bytes(&self) -> &[u8] {
        unimplemented!();
    }
    
    pub fn checksum(&self) -> bool {
        unimplemented!();
    }
}

