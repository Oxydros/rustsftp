
use std::io::{self, Read, Write};
use std::fs::File;

mod packet;

fn handle_init(stdin : &mut std::io::Stdin, out : &mut File) -> io::Result<()> {
    write!(out, "Handling INIT !\n");

    let mut version = [0; 4];
    stdin.read_exact(&mut version)?;
    let version_number = u32::from_be_bytes(version);

    write!(out, "Got SSH FTP Procotol v{}\n", version_number);

    // Create answer packet
    let answer = [5, packet::SSH_FXP_VERSION];

    write!(out, "Sending answer packet {:X?}\n", answer);
    let w = std::io::stdout().write(&answer)?;
    write!(out, "Wrote {} bytes\n", w);
    let w = std::io::stdout().write(&version)?;
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
            _ => write!(out, "Unknown packet ID\n")
        }?;
    }
}
