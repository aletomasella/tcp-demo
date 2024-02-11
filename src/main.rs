extern crate tun_tap;
use std::io;
fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;

    let mut buf = [0u8; 1504];
    loop {
        let nbytes = nic.recv(&mut buf[..])?;

        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(header) => {
                let src = header.source_addr();
                let dst = header.destination_addr();
                let protocol = header.protocol();

                eprint!("Protocol: {}", protocol.0);

                eprint!("src: {} dst: {}", src, dst);
            }
            Err(e) => {
                eprint!("error parsing ipv4 header: {:?}", e);
            }
        }

        let mut response = [0u8; 1504];
        response[0] = 0x45;

        let response_len = nic.send(&response[..nbytes])?;

        eprint!("wrote {} bytes", response_len);
    }
}
