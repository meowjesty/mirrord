use std::{ffi::CString, os::unix::process::parent_id};

use base64::prelude::*;
use libc::{c_char, c_int};
use mirrord_layer_macro::hook_guard_fn;
use tracing::Level;

use super::*;
#[cfg(target_os = "macos")]
use crate::exec_utils::*;
use crate::{
    common::CheckedInto,
    detour::Detour,
    hooks::HookManager,
    replace,
    socket::{UserSocket, SHARED_SOCKETS_ENV_VAR},
    SOCKETS,
};

/// Converts the [`SOCKETS`] map into a vector of pairs `(Fd, UserSocket)`, so we can rebuild
/// it as a map.
#[mirrord_layer_macro::instrument(level = Level::TRACE, ret)]
fn shared_sockets() -> Vec<(i32, UserSocket)> {
    SOCKETS
        .iter()
        .map(|inner| (*inner.key(), UserSocket::clone(inner.value())))
        .collect::<Vec<_>>()
}

/// Takes an [`Argv`] with the enviroment variables from an `exec` call, extending it with
/// an encoded version of our [`SOCKETS`].
///
/// The check for [`libc::FD_CLOEXEC`] is performed during the [`SOCKETS`] initialization
/// by the child process.
#[mirrord_layer_macro::instrument(
    level = Level::DEBUG,
    ret,
    fields(
        pid = std::process::id(),
        parent_pid = parent_id(),
    )
)]
pub(crate) fn execve(env_vars: Detour<Argv>) -> Detour<*const *const c_char> {
    let mut env_vars = env_vars.or_bypass(|x| match x {
        crate::detour::Bypass::EmptyOption => Detour::Success(Argv(Vec::new())),
        other => Detour::Bypass(other),
    })?;

    let encoded = bincode::encode_to_vec(shared_sockets(), bincode::config::standard())
        .map(|bytes| BASE64_URL_SAFE.encode(bytes))?;

    env_vars.push(CString::new(format!("{SHARED_SOCKETS_ENV_VAR}={encoded}"))?);

    Detour::Success(env_vars.leak())
}

/// Hook for `libc::execv` for linux only.
///
/// On macos this just calls `execve(path, argv, _environ)`, so we'll be handling it in our
/// [`execve_detour`].
#[cfg(not(target_os = "macos"))]
#[hook_guard_fn]
unsafe extern "C" fn execv_detour(path: *const c_char, argv: *const *const c_char) -> c_int {
    let encoded = bincode::encode_to_vec(shared_sockets(), bincode::config::standard())
        .map(|bytes| BASE64_URL_SAFE.encode(bytes))
        .unwrap_or_default();

    // `encoded` is emtpy if the encoding failed, so we don't set the env var.
    if !encoded.is_empty() {
        std::env::set_var("MIRRORD_SHARED_SOCKETS", encoded);
    }

    FN_EXECV(path, argv)
}

/// Hook for `libc::execve`.
///
/// We can't change the pointers, to get around that we create our own and **leak** them.
///
/// - #[cfg(target_os = "macos")]
///
/// We change 3 arguments and then call the original functions:
///
/// 1. The executable path - we check it for SIP, create a patched binary and use the path to the
/// new path instead of the original path. If there is no SIP, we use a new string with the same
/// path.
/// 2. argv - we strip mirrord's temporary directory from the start of arguments.
/// So if `argv[1]` is "/var/folders/1337/mirrord-bin/opt/homebrew/bin/npx", switch it
/// to "/opt/homebrew/bin/npx". Also here we create a new array with pointers
/// to new strings, even if there are no changes needed (except for the case of an error).
/// 3. envp - We found out that Turbopack (Vercel) spawns a clean "Node" instance without env,
/// basically stripping all of the important mirrord env.
/// [#2500](https://github.com/metalbear-co/mirrord/issues/2500)
/// We restore the `DYLD_INSERT_LIBRARIES` environment variable and all env vars
/// starting with `MIRRORD_` if the dyld var can't be found in `envp`.
///
/// If there is an error in the detour, we don't exit or anything, we just call the original libc
/// function with the original passed arguments.
#[hook_guard_fn]
pub(crate) unsafe extern "C" fn execve_detour(
    path: *const c_char,
    argv: *const *const c_char,
    envp: *const *const c_char,
) -> c_int {
    // Hopefully `envp` is a properly null-terminated list.
    let checked_envp = envp.checked_into();

    if let Detour::Success(modified_envp) = execve(checked_envp) {
        #[cfg(target_os = "macos")]
        match patch_sip_for_new_process(path, argv, modified_envp) {
            Detour::Success((new_path, new_argv, new_envp)) => FN_EXECVE(
                new_path.into_raw().cast_const(),
                new_argv.leak(),
                new_envp.leak(),
            ),
            _ => FN_EXECVE(path, argv, envp),
        }

        #[cfg(target_os = "linux")]
        FN_EXECVE(path, argv, modified_envp)
    } else {
        FN_EXECVE(path, argv, envp)
    }
}

/// Enables `exec` hooks.
pub(crate) unsafe fn enable_exec_hooks(hook_manager: &mut HookManager) {
    #[cfg(not(target_os = "macos"))]
    replace!(hook_manager, "execv", execv_detour, FnExecv, FN_EXECV);

    replace!(hook_manager, "execve", execve_detour, FnExecve, FN_EXECVE);
}