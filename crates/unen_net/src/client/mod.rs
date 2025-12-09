use std::net::{SocketAddr, UdpSocket};

use thiserror::Error;

use crate::{
    protocol::{self},
    socket::{self, socket_poll, socket_send},
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to bind socket: {0}")]
    SocketBindFailed(String),
    #[error("failed to set socket to nonblocking: {0}")]
    SocketNonBlockingFailed(String),
    #[error("failed to lock client to server at {0}: {1}")]
    LockingFailed(SocketAddr, String),
    #[error("failed to send data: {0}")]
    SendFailed(String),
    #[error("socket error: {0}")]
    SocketError(#[from] socket::Error),
    #[error("protocol error: {0}")]
    ProtocolError(#[from] protocol::Error),
}

pub fn create_client() -> DisconnectedClient {
    DisconnectedClient::default()
}

#[derive(Default)]
pub struct DisconnectedClient {}

impl DisconnectedClient {
    pub fn connect(
        self,
        client_addr: SocketAddr,
        server_addr: SocketAddr,
    ) -> Result<ConnectedClient, Error> {
        let socket =
            UdpSocket::bind(client_addr).map_err(|err| Error::SocketBindFailed(err.to_string()))?;
        socket
            .set_nonblocking(true)
            .map_err(|err| Error::SocketNonBlockingFailed(err.to_string()))?;

        socket
            .connect(server_addr)
            .map_err(|err| Error::LockingFailed(server_addr, err.to_string()))?;

        Ok(ConnectedClient { socket })
    }
}

pub struct ConnectedClient {
    socket: UdpSocket,
}

impl ConnectedClient {
    pub fn disconnect(self) -> DisconnectedClient {
        DisconnectedClient {}
    }

    pub fn send(&self, buf: &[u8]) -> Result<usize, Error> {
        Ok(socket_send(&self.socket, buf)?)
    }

    pub fn poll(&self) -> Vec<Vec<u8>> {
        socket_poll(&self.socket)
    }

    pub fn addr(&self) -> SocketAddr {
        self.socket.local_addr().unwrap()
    }
}
