
use std::io::{self, Read, Write};
use std::fs::File;
use std::convert::TryInto;
use std::string::String;
use std::path::Path;
use std::str;
use std::ffi::CString;

use std::fs;

use bytes::{BytesMut, BufMut, Buf};

mod packet;

// Server must answer a SSH_FXP_VERSION specifying the lowest version anf
// its set of capabilities
// https://tools.ietf.org/pdf/draft-ietf-secsh-filexfer-13.pdf#20
fn handle_init(stdin : &mut std::io::Stdin, out : &mut File, buf : &mut bytes::BytesMut) -> io::Result<()> {
    writeln!(out, "Handling INIT !");

    let version_number = buf.get_u32();

    writeln!(out, "Got SSH FTP Procotol v{}", version_number);

    let mut answer = bytes::BytesMut::new();
    answer.put_u8(packet::SSH_FXP_VERSION);
    answer.put_u32(3);

    // Convert tu 4 bytes
    let size : u32 = answer.len().try_into().unwrap();

    writeln!(out, "Sending answer packet {:X?} of size {}", answer, size);
    let w = std::io::stdout().write(&size.to_be_bytes())?;
    writeln!(out, "Wrote {} bytes", w);
    let w = std::io::stdout().write(&answer)?;
    writeln!(out, "Wrote {} bytes", w);
    Ok(())
}

fn handle_realpath(stdin : &mut std::io::Stdin, out : &mut File, buf : &mut bytes::BytesMut) -> io::Result<()> {
    writeln!(out, "Handling  SSH_FXP_REALPATH!");

    let req_id = buf.get_u32();

    let req_id_2 = buf.get_u32();

    writeln!(out, "Got REQ ID {}, 2 is {}", req_id, req_id_2);

    let rest : Vec<u8> = buf.to_vec();
    
    let original_path = str::from_utf8(&rest).unwrap();

    writeln!(out, "Path is {:?}", original_path);

    let path = std::path::Path::new(original_path);
    let realpath = fs::canonicalize(&path).unwrap();
    writeln!(out, "Real is {:?}", realpath);
    
    let mut answer = bytes::BytesMut::new();
    answer.put_u8(packet::SSH_FXP_NAME);
    answer.put_u32(req_id);
    answer.put_u32(1);
    let to_send_path = realpath.to_str().unwrap().as_bytes();
    answer.put_u32(to_send_path.len() as u32);
    answer.put(to_send_path);
    answer.put_u32(to_send_path.len() as u32);
    answer.put(to_send_path);
    answer.put_u32(0);

    // Convert tu 4 bytes
    let size : u32 = answer.len().try_into().unwrap();

    writeln!(out, "Sending answer packet {:X?} of size {}", answer, size);
    let w = std::io::stdout().write(&size.to_be_bytes())?;
    writeln!(out, "Wrote {:?} bytes", w);
    let w = std::io::stdout().write(&answer)?;
    writeln!(out, "Wrote {:?} bytes", w);

    Ok(())
}

fn main() -> io::Result<()> {
    let mut stdin = io::stdin();

    let out_file = "out_rustftp.debug";
    let mut out = File::create(out_file).expect("create failed");

    let mut length_packet_buffer = [0; 4];
    let mut packet_type = [0; 1];
    
    loop {
        stdin.read_exact(&mut length_packet_buffer)?;
        let packet_size = u32::from_be_bytes(length_packet_buffer);
        writeln!(out, "Got packet of size {}", packet_size);

        stdin.read_exact(&mut packet_type)?;
        writeln!(out, "Got packet type {:X?}", packet_type);

        let length_to_read : usize = (packet_size - 1) as usize;

        let mut buf = bytes::BytesMut::with_capacity(length_to_read);
        let mut read_buf = Vec::new();
        stdin
            .by_ref()
            .take(length_to_read as u64)
            .read_to_end(&mut read_buf).unwrap();

        buf.extend_from_slice(&read_buf);
        writeln!(out, "Packet content: {:?}", buf);

        // Dispatch
        match packet_type[0] {
            packet::SSH_FXP_INIT => handle_init(&mut stdin, &mut out, &mut buf),
            packet::SSH_FXP_REALPATH => handle_realpath(&mut stdin, &mut out, &mut buf),
            _ => writeln!(out, "Unknown packet ID {}", packet_type[0])
        }?;

        // Flush out
        std::io::stdout().flush().unwrap();
    }
}