use std::fmt;
use num::FromPrimitive;

use crate::utils::{bytes_to_name_offset, extract_name, byte_combine};

enum_from_primitive! {
#[derive(Debug, PartialEq)]
pub enum RRType {
    A = 1,  // RFC1035
    NS = 2, // RFC1035
    CNAME = 5,  // RFC1035
    SOA = 6,  // RFC1035 RFC2038
    PTR = 12,  // RFC1035
    MX = 15, // RFC1035 RFC7505
    TXT = 16, // RFC1035
}
}

enum_from_primitive! {
#[derive(Debug, PartialEq)]
pub enum Class {
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4
}
}


fn extract_ttl(bytes: &[u8], offset: usize) -> i32 {
    (16 * 16 * (byte_combine(bytes[offset], bytes[offset + 1]) as i32)
        + (byte_combine(bytes[offset + 2], bytes[offset + 3]) as i32))
}

pub struct RR {
    name: Vec<String>,
    rrtype: RRType,
    class: Class,
    ttl: i32,
    rdlength: u16,
    rdata: Vec<u8>,
}

impl RR {
    pub fn from_wire(buf: &[u8], mut offset: usize) -> (RR, usize) {
        let mut name = Vec::new();
        let name_type = buf[offset] >> 6;

        if name_type == 3 {
            let name_offset = bytes_to_name_offset(buf[offset], buf[offset + 1]);
            let (mut ref_name, _loffset) = extract_name(buf, name_offset as usize);
            name.append(&mut ref_name);
            offset += 2;
        } else if name_type == 0 {
            let (mut ref_name, new_offset) = extract_name(buf, offset);
            name.append(&mut ref_name);
            offset = new_offset;
        } else {
            panic!("Unimplemented name type: {:#b}", name_type);
        }

        // XXX: do we really want to unwrap here?!
        let rrtype = RRType::from_u16(byte_combine(buf[offset], buf[offset + 1])).unwrap();
        offset += 2;
        let class = Class::from_u16(byte_combine(buf[offset], buf[offset + 1])).unwrap();
        offset += 2;
        let ttl = extract_ttl(buf, offset);
        offset += 4;
        let rdlength = byte_combine(buf[offset], buf[offset + 1]) as usize;
        offset += 2;

        let rr = RR {
            name: name,
            rrtype: rrtype,
            class: class,
            ttl: ttl,
            rdlength: rdlength as u16,
            rdata: buf[offset .. offset+rdlength].to_owned(),
        };
        offset += rdlength;
        (rr, offset)
    }
}

fn format_rdata(rr: &RR) -> String {
    let mut rdata_fmt = String::new();
    match rr.rrtype {
        RRType::A => {
            let mut todo = rr.rdlength / 4; // 4 bytes per IPv4
            let mut idx = 0;
            while todo > 0 {
                let mut sep = "";
                for byte in rr.rdata[idx..idx+4].iter() {
                    rdata_fmt.push_str(sep);
                    rdata_fmt.push_str(&byte.to_string());
                    sep = ".";
                }
                rdata_fmt.push_str("\n");
                idx += 4;
                todo -= 1;
            }
        },
        _ => {
            for byte in rr.rdata.iter() {
                rdata_fmt.push_str(&byte.to_string());
            }
        }
    }
    rdata_fmt
}

impl fmt::Display for RR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rdata_fmt = format_rdata(&self);

        write!(
            f,
            "{}\t{:?}\t{:?}\tTTL: {:?}, RDLEN: {:?}\n{}",
            self.name.join("."),
            self.rrtype,
            self.class,
            self.ttl,
            self.rdlength,
            rdata_fmt
        )
    }
}