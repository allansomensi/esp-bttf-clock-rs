use crate::error::AppError;
use std::{
    io,
    net::{Ipv4Addr, SocketAddrV4, UdpSocket},
    time::Duration,
};

/// A DNS responder that listens for DNS requests and responds with predefined
/// DNS responses.
pub struct DnsResponder {
    response_footer: [u8; 16],
    udp_socket: UdpSocket,
}

impl DnsResponder {
    /// Initializes a new [DnsResponder] with the provided IP address to bind
    /// the UDP socket.
    ///
    /// This function creates a [UdpSocket] bound to the given `ip_address` and
    /// sets a read timeout for the socket. It also sets up a predefined
    /// response footer, which includes the IP address to be used in DNS
    /// responses.
    ///
    /// ## Arguments
    /// - `ip_address` - The IPv4 address to bind the DNS server to.
    ///
    /// ## Returns
    /// Returns `Ok(Self)` if the socket is successfully created and
    /// initialized, or an [`AppError`] if an error occurs during
    /// initialization.
    ///
    /// ## Example
    /// ```rust
    /// let dns_responder = DnsResponder::init(Ipv4Addr::from_str("192.168.71.1").unwrap())?;
    /// ```
    pub fn init(ip_address: Ipv4Addr) -> Result<Self, AppError> {
        let udp_socket = UdpSocket::bind(SocketAddrV4::new(ip_address, 53))?;
        udp_socket.set_read_timeout(Some(Duration::from_millis(10)))?;

        let mut response_footer = [
            0xc0, 0x0c, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x04, 0x00, 0x00,
            0x00, 0x00,
        ];
        response_footer[12..].copy_from_slice(&ip_address.octets());

        Ok(Self {
            response_footer,
            udp_socket,
        })
    }

    /// Handles incoming DNS requests by reading the request and sending a
    /// predefined response.
    ///
    /// This function listens for DNS requests on the bound UDP socket,
    /// processes the requests, and sends a response back to the requesting
    /// client. The response includes the predefined IP address configured
    /// in the [`DnsResponder`] instance. If the packet size exceeds 100
    /// bytes, a warning is logged.
    ///
    /// ## Returns
    /// Returns `Ok(())` if the request is successfully processed and responded
    /// to, or an [`AppError`] if an error occurs while handling the request.
    ///
    /// ## Example
    /// ```rust
    /// dns_responder.handle_requests()?;
    /// ```
    pub fn handle_requests(&mut self) -> Result<(), AppError> {
        let mut buffer = [0; 128];
        match self.udp_socket.recv_from(&mut buffer) {
            Ok((length, client_addr)) => {
                if length > 100 {
                    log::warn!("Received DNS request with an invalid packet size: {length}");
                } else {
                    buffer[2] |= 0x80;
                    buffer[3] |= 0x80;
                    buffer[7] = 0x01;
                    let total_len = length + self.response_footer.len();
                    buffer[length..total_len].copy_from_slice(&self.response_footer);
                    self.udp_socket
                        .send_to(&buffer[0..total_len], client_addr)?;
                }
                Ok(())
            }
            Err(error) => match error.kind() {
                io::ErrorKind::TimedOut => Ok(()),
                _ => Err(AppError::StdIO(error)),
            },
        }
    }
}
