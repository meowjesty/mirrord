use std::path::Path;

use apple_codesign::{CodeSignatureFlags, SettingsScope, SigningSettings, UnifiedSigner};

use crate::error::Result;

pub(crate) fn sign<PI: AsRef<Path>, PO: AsRef<Path>>(input: PI, output: PO) -> Result<()> {
    // in the past, we used the codesign binary
    // but we had an issue where it received EBADF (bad file descriptor)
    // probably since in some flows, like Go,
    // it calls fork then execve, then we do the same from our code (to call codesign)
    // we switched to use apple codesign crate to avoid this issue (of creating process)
    // but if we sign in place we get permission error, probably because someone is holding the
    // handle to the named temp file.
    let mut settings = SigningSettings::default();
    // Replace any existing flags with just the adhoc flag.
    // Important because some binaries (e.g. `go`), have the "runtime" flag set, which means,
    // opting into the hardened runtime, which strips away DYLD_INSERT_LIBRARIES etc.
    settings.set_code_signature_flags(SettingsScope::Main, CodeSignatureFlags::ADHOC);
    let signer = UnifiedSigner::new(settings);
    signer.sign_path(input, output)?;
    Ok(())
}
