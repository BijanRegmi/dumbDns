#[derive(Debug)]
pub struct Header {
    pub id: u16,
    // Flags Start: u16
    /// Query or Response
    pub qr: u8, // 1 bit
    /// kind of query in this message
    pub opcode: u8, // 4 bits
    /// Authoritative Answer
    pub aa: u8, // 1 bit
    /// TrunCation
    pub tc: u8, // 1 bit
    /// Recursion Desired
    pub rd: u8, // 1 bit
    /// Recursion Available
    pub ra: u8, // 1 bit
    /// Padding
    pub z: u8, // 3 bits
    /// Response Code
    pub rcode: u8, // 4 bits
    // Flags End
    /// Number of entries in Question Section
    pub qdcount: u16,
    /// Number of RRs in Answer Section
    pub ancount: u16,
    /// Number of name server RRs in Authority records Section
    pub nscount: u16,
    /// Number of RRs in Additional records Section
    pub arcount: u16,
}

impl Header {
    pub fn from_buffer(buf: &Vec<u8>, offset: usize) -> Self {
        Header {
            id: ((buf[offset + 0] as u16) << 8) | buf[offset + 1] as u16,
            qr: (buf[offset + 2] & 0b1000000) >> 7,
            opcode: (buf[offset + 2] & 0b01111000) >> 3,
            aa: (buf[offset + 2] & 0b00000100) >> 2,
            tc: (buf[offset + 2] & 0b00000010) >> 1,
            rd: (buf[offset + 2] & 0b00000001),
            ra: (buf[offset + 3] & 0b10000000) >> 7,
            z: (buf[offset + 3] & 0b01110000) >> 4,
            rcode: buf[offset + 3] & 0b00001111,
            qdcount: ((buf[offset + 4] as u16) << 8) | buf[offset + 5] as u16,
            ancount: ((buf[offset + 6] as u16) << 8) | buf[offset + 7] as u16,
            nscount: ((buf[offset + 8] as u16) << 8) | buf[offset + 9] as u16,
            arcount: ((buf[offset + 10] as u16) << 8) | buf[offset + 11] as u16,
        }
    }

    pub fn to_buffer(&self) -> Vec<u8> {
        vec![
            (self.id >> 8) as u8,
            self.id as u8,
            (self.qr << 7) | (self.opcode << 3) | (self.aa << 2) | (self.tc << 1) | self.rd,
            (self.ra << 7) | (self.z << 4) | (self.rcode),
            (self.qdcount >> 8) as u8,
            self.qdcount as u8,
            (self.ancount >> 8) as u8,
            self.ancount as u8,
            (self.nscount >> 8) as u8,
            self.nscount as u8,
            (self.arcount >> 8) as u8,
            self.arcount as u8,
        ]
    }
}

#[derive(Debug)]
pub struct Query {
    pub qtype: u16,
    pub qclass: u16,
    pub qname: Vec<u8>,
}

impl Query {
    pub fn from_buffer(buf: &Vec<u8>, offset: usize) -> Self {
        let mut end: usize = 0;
        for i in offset..buf.len() {
            if buf[i] == 0 {
                end = i;
                break;
            }
        }
        Query {
            qname: buf[offset..end + 1].to_vec(),
            qtype: (buf[end + 1] as u16) << 8 | buf[end + 2] as u16,
            qclass: (buf[end + 3] as u16) << 8 | buf[end + 4] as u16,
        }
    }

    pub fn get_name(&self) -> String {
        let mut result = String::new();
        let mut count = 0;
        for &c in &self.qname {
            if count == 0 {
                count = c;
            } else {
                result.push(c as char);
                count -= 1;
                if count == 0 {
                    result.push('.');
                }
            }
        }
        return result;
    }

    pub fn to_buffer(&self) -> Vec<u8> {
        let mut result = vec![];
        result.extend_from_slice(&self.qname);
        result.extend_from_slice(&[
            (self.qtype >> 8) as u8,
            self.qtype as u8,
            (self.qclass >> 8) as u8,
            self.qclass as u8,
        ]);

        result
    }
}

#[derive(Debug)]
pub struct ResourceRecord {
    pub name: Vec<u8>,
    pub r#type: u16,
    pub class: u16,
    pub ttl: u32,
    pub rdlength: u16,
    pub rdata: Vec<u8>,
}

impl ResourceRecord {
    pub fn new(ttl: u32, rdata: &str) -> Self {
        let parts = rdata
            .split('.')
            .collect::<Vec<_>>()
            .iter()
            .map(|x| x.parse::<u8>().unwrap())
            .collect::<Vec<_>>();
        ResourceRecord {
            name: vec![0xc0, 0x0c],
            r#type: 1,
            class: 1,
            ttl,
            rdlength: parts.len() as u16,
            rdata: parts,
        }
    }

    pub fn to_buffer(&self) -> Vec<u8> {
        let mut result = vec![];
        result.extend_from_slice(&self.name);
        result.extend_from_slice(&[
            (self.r#type >> 8) as u8,
            self.r#type as u8,
            (self.class >> 8) as u8,
            self.class as u8,
            (self.ttl >> 24) as u8,
            (self.ttl >> 16) as u8,
            (self.ttl >> 8) as u8,
            self.ttl as u8,
            (self.rdlength >> 8) as u8,
            self.rdlength as u8,
        ]);
        result.extend_from_slice(&self.rdata);
        return result;
    }
}
