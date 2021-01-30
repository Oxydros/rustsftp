
use std::io::{self, Read, Write};
use std::fs::File;
use std::convert::TryInto;

use bytes::{BytesMut, BufMut};

mod packet;

// Server must answer a SSH_FXP_VERSION specifying the lowest version anf
// its set of capabilities
// https://tools.ietf.org/pdf/draft-ietf-secsh-filexfer-13.pdf#20
fn handle_init(stdin : &mut std::io::Stdin, out : &mut File) -> io::Result<()> {
    write!(out, "Handling INIT !\n");

    let mut version = [0; 4];

    stdin.read_exact(&mut version)?;
    let version_number = u32::from_be_bytes(version);

    write!(out, "Got SSH FTP Procotol v{}\n", version_number);

    let mut answer = bytes::BytesMut::new();
    answer.put_u8(packet::SSH_FXP_VERSION);
    answer.put_u32(version_number);

    // Add extensions
    // string "supported2"
    // string supported-structure
    //     uint32 supported-attribute-mask
    //     uint32 supported-attribute-bits
    //     uint32 supported-open-flags
    //     uint32 supported-access-mask
    //     uint32 max-read-size
    //     uint16 supported-open-block-vector
    //     uint16 supported-block-vector
    //     uint32 attrib-extension-count
    //     string attrib-extension-names[attrib_extension-count]
    //     uint32 extension-count
    //     string extension-names[extension-count]

    // answer.put(&b"supported2"[..]);
    // answer.put_u32(0);
    // answer.put_u32(0);
    // answer.put_u32(0);
    // answer.put_u32(0);
    // answer.put_u32(0);
    // answer.put_u16(0);
    // answer.put_u16(0);
    // answer.put(&b""[..]);
    // answer.put_u32(0);
    // answer.put(&b""[..]);
    // // let attribute_mask : u32 = 0;
    // // let attribute_bits : u32 = 0;
    // // let open_flags : u32 = 0;
    // // let access_mask : u32 = 0;
    // // let max_read_size : u32 = 0;
    // // let open_block_vector : u16 = 0;
    // // let attrib_extension_count : u16 = 0;
    // // let attrib_extension_names = "";
    // // let extension_count : u32 = 0;
    // // let extension_names = "";

    // // string "acl-supported"
    // // string supported-structure
    // //     uint32 capabilities
    // answer.put(&b"acl-supported"[..]);
    // answer.put_u32(0);

    // Convert tu 4 bytes
    let size : u32 = answer.len().try_into().unwrap();

    write!(out, "Sending answer packet {:X?} of size {}\n", answer, size);
    let w = std::io::stdout().write(&size.to_be_bytes())?;
    write!(out, "Wrote {} bytes\n", w);
    let w = std::io::stdout().write(&answer)?;
    write!(out, "Wrote {} bytes\n", w);
    Ok(())
}

fn handle_version(stdin : &mut std::io::Stdin, out : &mut File) -> io::Result<()> {
    write!(out, "Handling VERSION !\n");
    Ok(())
}

fn handle_open(stdin : &mut std::io::Stdin, out : &mut File) -> io::Result<()> {
    write!(out, "Handling OPEN !\n");
    Ok(())
}

fn handle_fsetstat(stdin : &mut std::io::Stdin, out : &mut File) -> io::Result<()> {
    write!(out, "Handling  SSH_FXP_FSETSTAT!\n");
    Ok(())
}

fn handle_realpath(stdin : &mut std::io::Stdin, out : &mut File, to_read : u32) -> io::Result<()> {
    write!(out, "Handling  SSH_FXP_REALPATH!\n");

    let mut req_id = [0; 4];
    stdin.read_exact(&mut req_id)?;
    let req_id_number = u32::from_be_bytes(req_id);

    write!(out, "Got REQ ID{}\n", req_id_number);

    let mut path = String::new();
    stdin.read_to_string(&mut path).unwrap();

    write!(out, "Got PATH {}\n", path);

    let string_size : u32 = path.len().try_into().unwrap();
    let to_read : u32 = to_read - 4 - string_size;

    write!(out, "Need to read {}\n", to_read);
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
        write!(out, "Got packet of size {}\n", packet_size);

        stdin.read_exact(&mut packet_type)?;
        write!(out, "Got packet type {:X?}\n", packet_type);

        // Dispatch
        match packet_type[0] {
            packet::SSH_FXP_INIT => handle_init(&mut stdin, &mut out),
            packet::SSH_FXP_VERSION => handle_version(&mut stdin, &mut out),
            packet::SSH_FXP_OPEN => handle_open(&mut stdin, &mut out),
            packet::SSH_FXP_FSETSTAT => handle_fsetstat(&mut stdin, &mut out),
            packet::SSH_FXP_REALPATH => handle_realpath(&mut stdin, &mut out, packet_size - 1),
            _ => write!(out, "Unknown packet ID {}\n", packet_type[0])
        }?;

        // Flush out
        std::io::stdout().flush().unwrap();
    }
}