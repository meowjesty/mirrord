use std::{ffi::CStr, os::unix::io::RawFd, slice, sync::atomic::Ordering};

use frida_gum::interceptor::Interceptor;
use libc::{c_char, c_int, c_void, sockaddr, socklen_t};
use mirrord_macro::hook_fn;
use mirrord_protocol::AddrInfoHint;
use socket2::SockAddr;
use tracing::{debug, error, trace, warn};

use super::ops::*;
use crate::{error::LayerError, replace, socket::AddrInfoHintExt};

/// TODO(alex) [high] 2022-08-15: Flow is (for named connect):
/// 1. `socket` with domain set to `PF_ROUTE` (fd 28);
/// 2. `bind` called on (fd 28) with address `sa_family: 16` and `sa_data` just a bunch of `0`s;
/// 3. `getsockname` on (fd 28);
/// 4. `socket` with domain `AF_INET`;
/// 5. `setsockopt` on (fd 28) with option 11;
/// 6. `connect` on (fd 28);
///
/// ADD(alex) [high] 2022-08-15: So, what happens is that it'll first try to connect to a local
/// address. When the DNS feature is disabled, the first call to connect will have my lan IP, only
/// after we do a second `socket` call and get the proper address lookup. It's like we must first
/// connect to retrieve the address from "google.com", before we do an actual call to connect to it.
#[hook_fn]
pub(crate) unsafe extern "C" fn socket_detour(
    domain: c_int,
    type_: c_int,
    protocol: c_int,
) -> c_int {
    trace!(
        "socket_detour -> domain {:#?} | type:{:#?} | protocol {:#?}",
        domain,
        type_,
        protocol
    );

    if IS_INTERNAL_CALL.load(Ordering::Acquire) {
        debug!("socket_detour -> bypassed");
        FN_SOCKET(domain, type_, protocol)
    } else {
        let (Ok(result) | Err(result)) =
            socket(domain, type_, protocol).map_err(|fail| match fail {
                LayerError::BypassedType(_) | LayerError::BypassedDomain(_) => {
                    FN_SOCKET(domain, type_, protocol)
                }
                other => other.into(),
            });

        result
    }
}

#[hook_fn]
pub(crate) unsafe extern "C" fn bind_detour(
    sockfd: c_int,
    raw_address: *const sockaddr,
    address_length: socklen_t,
) -> c_int {
    trace!(
        "bind_detour -> sockfd {:#?} | raw_address {:#?}",
        sockfd,
        *raw_address
    );

    if IS_INTERNAL_CALL.load(Ordering::Acquire) {
        debug!("bind_detour -> bypassed");
        FN_BIND(sockfd, raw_address, address_length)
    } else {
        let address = SockAddr::init(|storage, len| {
            storage.copy_from_nonoverlapping(raw_address.cast(), 1);
            len.copy_from_nonoverlapping(&address_length, 1);

            Ok(())
        })
        .map_err(LayerError::from);

        match address {
            Ok(((), address)) => {
                let (Ok(result) | Err(result)) =
                    bind(sockfd, address)
                        .map(|()| 0)
                        .map_err(|fail| match fail {
                            LayerError::LocalFDNotFound(_)
                            | LayerError::BypassedPort(_)
                            | LayerError::AddressConversion => {
                                FN_BIND(sockfd, raw_address, address_length)
                            }
                            other => other.into(),
                        });
                result
            }
            Err(_) => {
                warn!("bind_detour -> Could not convert address, bypassing!");

                FN_BIND(sockfd, raw_address, address_length)
            }
        }
    }
}

#[hook_fn]
pub(crate) unsafe extern "C" fn listen_detour(sockfd: RawFd, backlog: c_int) -> c_int {
    debug!(
        "listen_detour -> sockfd {:#?} | backlog {:#?}",
        sockfd, backlog
    );

    if IS_INTERNAL_CALL.load(Ordering::Acquire) {
        debug!("listen_detour -> bypassed");
        FN_LISTEN(sockfd, backlog)
    } else {
        let (Ok(result) | Err(result)) =
            listen(sockfd, backlog)
                .map(|()| 0)
                .map_err(|fail| match fail {
                    LayerError::LocalFDNotFound(_) | LayerError::SocketInvalidState(_) => {
                        FN_LISTEN(sockfd, backlog)
                    }
                    other => other.into(),
                });
        result
    }
}

#[hook_fn]
pub(super) unsafe extern "C" fn connect_detour(
    sockfd: RawFd,
    // TODO(alex) [high] 2022-08-03: We're trying to connect to 255.127.0.0, why? Looks like the
    // DNS stuff is returning correct values (this address appears nowhere).
    raw_address: *const sockaddr,
    address_length: socklen_t,
) -> c_int {
    trace!("connect_detour -> sockfd {:#?}", sockfd);

    if IS_INTERNAL_CALL.load(Ordering::Acquire) {
        debug!("connect_detour -> bypassed");

        FN_CONNECT(sockfd, raw_address, address_length)
    } else {
        // TODO: Is this conversion safe?
        let address = SockAddr::new(*(raw_address as *const _), address_length);
        debug!("connect_detour -> address {:#?}", address);

        let address = address.as_socket().ok_or(LayerError::AddressConversion);

        // TODO(alex) [high] 2022-08-03: Drilling down, maybe we need to bypass a bunch of stuff
        // when connect is being called, then release the bypass?
        match address {
            Ok(address) => {
                let (Ok(result) | Err(result)) =
                    connect(sockfd, address)
                        .map(|()| 0)
                        .map_err(|fail| match fail {
                            LayerError::LocalFDNotFound(_) | LayerError::SocketInvalidState(_) => {
                                FN_CONNECT(sockfd, raw_address, address_length)
                            }
                            other => other.into(),
                        });

                result
            }
            Err(_) => {
                warn!("connect_detour -> Could not convert address, bypassing!");

                FN_CONNECT(sockfd, raw_address, address_length)
            }
        }
    }
}

#[hook_fn]
pub(super) unsafe extern "C" fn getpeername_detour(
    sockfd: RawFd,
    address: *mut sockaddr,
    address_len: *mut socklen_t,
) -> c_int {
    trace!("getpeername_detour -> sockfd {:#?}", sockfd);

    if IS_INTERNAL_CALL.load(Ordering::Acquire) {
        debug!("getpeername_detour -> bypassed");
        FN_GETPEERNAME(sockfd, address, address_len)
    } else {
        let (Ok(result) | Err(result)) = getpeername(sockfd, address, address_len)
            .map(|()| 0)
            .map_err(|fail| match fail {
                LayerError::LocalFDNotFound(_) | LayerError::SocketInvalidState(_) => {
                    FN_GETPEERNAME(sockfd, address, address_len)
                }
                other => other.into(),
            });
        result
    }
}

#[hook_fn]
pub(super) unsafe extern "C" fn getsockname_detour(
    sockfd: RawFd,
    address: *mut sockaddr,
    address_len: *mut socklen_t,
) -> i32 {
    trace!("getsockname_detour -> sockfd {:#?}", sockfd);

    if IS_INTERNAL_CALL.load(Ordering::Acquire) {
        debug!("getsockname_detour -> bypassed");

        let result = FN_GETSOCKNAME(sockfd, address, address_len);
        let sockaddr = SockAddr::new(*(address as *const _), *address_len);
        debug!(
            "getsockname_detour -> sockaddr {:#?} | std {:#?}",
            sockaddr,
            sockaddr.as_socket()
        );

        result
    } else {
        let (Ok(result) | Err(result)) = getsockname(sockfd, address, address_len)
            .map(|()| 0)
            .map_err(|fail| match fail {
                LayerError::LocalFDNotFound(_) | LayerError::SocketInvalidState(_) => {
                    FN_GETSOCKNAME(sockfd, address, address_len)
                }
                other => other.into(),
            });
        result
    }
}

#[hook_fn]
pub(crate) unsafe extern "C" fn accept_detour(
    sockfd: c_int,
    address: *mut sockaddr,
    address_len: *mut socklen_t,
) -> i32 {
    trace!("accept_detour -> sockfd {:#?}", sockfd);

    let accept_result = FN_ACCEPT(sockfd, address, address_len);

    if accept_result == -1 || IS_INTERNAL_CALL.load(Ordering::Acquire) {
        accept_result
    } else {
        let (Ok(result) | Err(result)) = accept(sockfd, address, address_len, accept_result)
            .map_err(|fail| match fail {
                LayerError::SocketInvalidState(_) | LayerError::LocalFDNotFound(_) => accept_result,
                other => {
                    error!("accept error is {:#?}", other);
                    other.into()
                }
            });

        result
    }
}

#[cfg(target_os = "linux")]
#[hook_fn]
pub(crate) unsafe extern "C" fn accept4_detour(
    sockfd: i32,
    address: *mut sockaddr,
    address_len: *mut socklen_t,
    flags: i32,
) -> i32 {
    trace!("accept4_detour -> sockfd {:#?}", sockfd);

    let accept_result = FN_ACCEPT4(sockfd, address, address_len, flags);

    if accept_result == -1 || IS_INTERNAL_CALL.load(Ordering::Acquire) {
        accept_result
    } else {
        let (Ok(result) | Err(result)) = accept(sockfd, address, address_len, accept_result)
            .map_err(|fail| match fail {
                LayerError::SocketInvalidState(_) | LayerError::LocalFDNotFound(_) => accept_result,
                other => {
                    error!("accept4 error is {:#?}", other);
                    other.into()
                }
            });

        result
    }
}

#[cfg(target_os = "linux")]
#[hook_fn]
#[allow(non_snake_case)]
pub(super) unsafe extern "C" fn uv__accept4_detour(
    sockfd: i32,
    address: *mut sockaddr,
    address_len: *mut socklen_t,
    flags: i32,
) -> i32 {
    trace!("uv__accept4_detour -> sockfd {:#?}", sockfd);

    accept4_detour(sockfd, address, address_len, flags)
}

/// https://github.com/metalbear-co/mirrord/issues/184
#[hook_fn]
pub(super) unsafe extern "C" fn fcntl_detour(fd: c_int, cmd: c_int, mut arg: ...) -> c_int {
    trace!("fcntl_detour -> fd {:#?} | cmd {:#?}", fd, cmd);

    let arg = arg.arg::<usize>();
    let fcntl_result = FN_FCNTL(fd, cmd, arg);

    if fcntl_result == -1 || IS_INTERNAL_CALL.load(Ordering::Acquire) {
        fcntl_result
    } else {
        let (Ok(result) | Err(result)) = fcntl(fd, cmd, fcntl_result)
            .map(|()| fcntl_result)
            .map_err(|fail| match fail {
                LayerError::LocalFDNotFound(_) => fcntl_result,
                other => other.into(),
            });

        trace!("fcntl_detour -> result {:#?}", result);
        result
    }
}

#[hook_fn]
pub(super) unsafe extern "C" fn dup_detour(fd: c_int) -> c_int {
    trace!("dup_detour -> fd {:#?}", fd);

    let dup_result = FN_DUP(fd);

    if dup_result == -1 || IS_INTERNAL_CALL.load(Ordering::Acquire) {
        dup_result
    } else {
        let (Ok(result) | Err(result)) =
            dup(fd, dup_result)
                .map(|()| dup_result)
                .map_err(|fail| match fail {
                    LayerError::LocalFDNotFound(_) => dup_result,
                    _ => fail.into(),
                });

        trace!("dup_detour -> result {:#?}", result);
        result
    }
}

#[hook_fn]
pub(super) unsafe extern "C" fn dup2_detour(oldfd: c_int, newfd: c_int) -> c_int {
    trace!("dup2_detour -> oldfd {:#?} | newfd {:#?}", oldfd, newfd);

    if oldfd == newfd {
        return newfd;
    }

    let dup2_result = FN_DUP2(oldfd, newfd);

    if dup2_result == -1 || IS_INTERNAL_CALL.load(Ordering::Acquire) {
        dup2_result
    } else {
        let (Ok(result) | Err(result)) =
            dup(oldfd, dup2_result)
                .map(|()| dup2_result)
                .map_err(|fail| match fail {
                    LayerError::LocalFDNotFound(_) => dup2_result,
                    _ => fail.into(),
                });

        trace!("dup2_detour -> result {:#?}", result);
        result
    }
}

#[cfg(target_os = "linux")]
#[hook_fn]
pub(super) unsafe extern "C" fn dup3_detour(oldfd: c_int, newfd: c_int, flags: c_int) -> c_int {
    trace!(
        "dup3_detour -> oldfd {:#?} | newfd {:#?} | flags {:#?}",
        oldfd,
        newfd,
        flags
    );

    let dup3_result = FN_DUP3(oldfd, newfd, flags);

    if dup3_result == -1 || IS_INTERNAL_CALL.load(Ordering::Acquire) {
        dup3_result
    } else {
        let (Ok(result) | Err(result)) =
            dup(oldfd, dup3_result)
                .map(|()| dup3_result)
                .map_err(|fail| match fail {
                    LayerError::LocalFDNotFound(_) => dup3_result,
                    _ => fail.into(),
                });

        trace!("dup3_detour -> result {:#?}", result);
        result
    }
}
/// Turns the raw pointer parameters into Rust types and calls `ops::getaddrinfo`.
///
/// # Warning:
/// - `raw_hostname`, `raw_servname`, and/or `raw_hints` might be null!
#[hook_fn]
pub(super) unsafe extern "C" fn getaddrinfo_detour(
    raw_node: *const c_char,
    raw_service: *const c_char,
    raw_hints: *const libc::addrinfo,
    out_addr_info: *mut *mut libc::addrinfo,
) -> c_int {
    trace!(
        "getaddrinfo_detour -> raw_node {:#?} | raw_service {:#?} | raw_hints {:#?} | out? {:#?}",
        raw_node,
        raw_service,
        *raw_hints,
        out_addr_info.is_null(),
    );

    let node = match (!raw_node.is_null())
        .then(|| CStr::from_ptr(raw_node).to_str())
        .transpose()
        .map_err(|fail| {
            error!("Failed converting raw_node from `c_char` with {:#?}", fail);

            libc::EAI_MEMORY
        }) {
        Ok(node) => node.map(String::from),
        Err(fail) => return fail,
    };

    let service = match (!raw_service.is_null())
        .then(|| CStr::from_ptr(raw_service).to_str())
        .transpose()
        .map_err(|fail| {
            error!(
                "Failed converting raw_service from `c_char` with {:#?}",
                fail
            );

            libc::EAI_MEMORY
        }) {
        Ok(service) => service.map(String::from),
        Err(fail) => return fail,
    };

    let hints = (!raw_hints.is_null()).then(|| AddrInfoHint::from_raw(*raw_hints));

    let (Ok(result) | Err(result)) = getaddrinfo(node, service, hints)
        .map(|c_addr_info_ptr| {
            out_addr_info.copy_from_nonoverlapping(&c_addr_info_ptr, 1);

            0
        })
        .map_err(From::from);

    result
}

/// Deallocates a `*mut libc::addrinfo` that was previously allocated with `Box::new` in
/// `getaddrinfo_detour` and converted into a raw pointer by `Box::into_raw`.
///
/// Also follows the `addr_info.ai_next` pointer, deallocating the next pointers in the linked list.
///
/// # Protocol
///
/// No need to send any sort of `free` message to `mirrord-agent`, as the `addrinfo` there is not
/// kept around.
///
/// # Warning
///
/// The `addrinfo` pointer has to be allocated respecting the `Box`'s
/// [memory layout](https://doc.rust-lang.org/std/boxed/index.html#memory-layout).
#[hook_fn]
pub(super) unsafe extern "C" fn freeaddrinfo_detour(addrinfo: *mut libc::addrinfo) {
    trace!("freeaddrinfo_detour -> addrinfo {:#?}", *addrinfo);

    // Iterate over `addrinfo` linked list dropping it.
    let mut current = addrinfo;
    while !current.is_null() {
        let current_box = Box::from_raw(current);

        current = (*current).ai_next;
        drop(current_box);
    }
}

#[hook_fn]
pub(super) unsafe extern "C" fn getsockopt_detour(
    sockfd: RawFd,
    level: c_int,
    option_name: c_int,
    option_value: *mut c_void,
    option_length: *mut socklen_t,
) -> c_int {
    trace!(
        "getsockopt_detour -> sockfd {:#?} | level {:#?} | option_name {:#?}",
        sockfd,
        level,
        option_name
    );

    FN_GETSOCKOPT(sockfd, level, option_name, option_value, option_length)
}

#[hook_fn]
pub(super) unsafe extern "C" fn setsockopt_detour(
    sockfd: RawFd,
    level: c_int,
    option_name: c_int,
    option_value: *const c_void,
    option_length: *mut socklen_t,
) -> c_int {
    trace!(
        "setsockopt_detour -> sockfd {:#?} | level {:#?} | option_name {:#?}",
        sockfd,
        level,
        option_name
    );

    FN_SETSOCKOPT(sockfd, level, option_name, option_value, option_length)
}

pub(crate) unsafe fn enable_socket_hooks(interceptor: &mut Interceptor, enabled_remote_dns: bool) {
    let _ = replace!(interceptor, "socket", socket_detour, FnSocket, FN_SOCKET);

    let _ = replace!(interceptor, "bind", bind_detour, FnBind, FN_BIND);
    let _ = replace!(interceptor, "listen", listen_detour, FnListen, FN_LISTEN);

    let _ = replace!(
        interceptor,
        "connect",
        connect_detour,
        FnConnect,
        FN_CONNECT
    );

    let _ = replace!(interceptor, "fcntl", fcntl_detour, FnFcntl, FN_FCNTL);
    let _ = replace!(interceptor, "dup", dup_detour, FnDup, FN_DUP);
    let _ = replace!(interceptor, "dup2", dup2_detour, FnDup2, FN_DUP2);

    let _ = replace!(
        interceptor,
        "getpeername",
        getpeername_detour,
        FnGetpeername,
        FN_GETPEERNAME
    );

    let _ = replace!(
        interceptor,
        "getsockname",
        getsockname_detour,
        FnGetsockname,
        FN_GETSOCKNAME
    );

    #[cfg(target_os = "linux")]
    {
        let _ = replace!(
            interceptor,
            "uv__accept4",
            uv__accept4_detour,
            FnUv__accept4,
            FN_UV__ACCEPT4
        );

        let _ = replace!(
            interceptor,
            "accept4",
            accept4_detour,
            FnAccept4,
            FN_ACCEPT4
        );

        let _ = replace!(interceptor, "dup3", dup3_detour, FnDup3, FN_DUP3);
    }

    let _ = replace!(interceptor, "accept", accept_detour, FnAccept, FN_ACCEPT);

    if enabled_remote_dns {
        let _ = replace!(
            interceptor,
            "getaddrinfo",
            getaddrinfo_detour,
            FnGetaddrinfo,
            FN_GETADDRINFO
        );

        let _ = replace!(
            interceptor,
            "freeaddrinfo",
            freeaddrinfo_detour,
            FnFreeaddrinfo,
            FN_FREEADDRINFO
        );
    }

    let _ = replace!(
        interceptor,
        "getsockopt",
        getsockopt_detour,
        FnGetsockopt,
        FN_GETSOCKOPT
    );

    let _ = replace!(
        interceptor,
        "setsockopt",
        setsockopt_detour,
        FnSetsockopt,
        FN_SETSOCKOPT
    );
}
