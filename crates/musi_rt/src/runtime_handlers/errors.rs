use std::fmt::Display;

use musi_vm::{ForeignCall, VmError, VmErrorKind};

pub(super) fn invalid_runtime_args(
    effect: &ForeignCall,
    expected: &str,
    found: impl Display,
) -> VmError {
    let detail = format!("{} expected {expected}, found {found}", effect.name());
    VmError::new(VmErrorKind::ForeignCallRejected {
        foreign: detail.into_boxed_str(),
    })
}

pub(super) fn runtime_foreign_failed(effect: &ForeignCall, source: impl Display) -> VmError {
    let detail = format!("{} failed ({source})", effect.name());
    VmError::new(VmErrorKind::ForeignCallRejected {
        foreign: detail.into_boxed_str(),
    })
}

pub(super) fn runtime_host_unavailable(effect: &ForeignCall, subject: &str) -> VmError {
    let detail = format!("{} host unavailable ({subject})", effect.name());
    VmError::new(VmErrorKind::ForeignCallRejected {
        foreign: detail.into_boxed_str(),
    })
}

pub(super) fn runtime_foreign_unsupported(effect: &ForeignCall) -> VmError {
    VmError::new(VmErrorKind::ForeignCallRejected {
        foreign: effect.name().into(),
    })
}

pub(super) fn foreign_rejected(foreign: &ForeignCall) -> VmError {
    VmError::new(VmErrorKind::ForeignCallRejected {
        foreign: foreign.name().into(),
    })
}
