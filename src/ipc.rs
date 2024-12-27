use crate::error::ErrorCode;
use nix::sys::socket::{recv, send, socketpair, AddressFamily, MsgFlags, SockFlag, SockType};
use std::os::fd::{IntoRawFd, RawFd};

pub fn generate_socketpair() -> Result<(RawFd, RawFd), ErrorCode> {
    log::debug!("Generating socket pair");

    match socketpair(
        AddressFamily::Unix,
        SockType::SeqPacket,
        None,
        SockFlag::SOCK_CLOEXEC,
    ) {
        Ok((fd0, fd1)) => Ok((fd0.into_raw_fd(), fd1.into_raw_fd())),
        Err(_) => Err(ErrorCode::SocketError(0)),
    }
}

pub fn send_bool(fd: RawFd, value: bool) -> Result<(), ErrorCode> {
    log::debug!("Sending a bool value `{}`", value);

    let buf: [u8; 1] = [value as u8];
    if let Err(e) = send(fd, &buf, MsgFlags::empty()) {
        log::error!("Cannot send bool value through socket: {:?}", e);
        return Err(ErrorCode::SocketError(1));
    }

    Ok(())
}

pub fn recv_bool(fd: RawFd) -> Result<bool, ErrorCode> {
    let mut buf: [u8; 1] = [0];
    if let Err(e) = recv(fd, &mut buf, MsgFlags::empty()) {
        log::error!("Cannot receive bool value through socket: {:?}", e);
        return Err(ErrorCode::SocketError(2));
    }
    let value = buf[0] == 0;
    log::debug!("Received a bool value `{}`", value);
    Ok(value)
}
