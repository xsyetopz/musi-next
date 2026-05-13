use std::env::{args_os, current_dir};
use std::process::Command;

use musi_foundation::process as foundation_process;
use musi_native::NativeHost;
use musi_vm::{EffectCall, Value, VmError};

use super::errors::{
    foreign_rejected, invalid_runtime_args, runtime_effect_failed, runtime_effect_unsupported,
};
use super::values::{foreign_string_arg, saturating_usize_to_i64, string_arg};

pub(super) fn register(host: &mut NativeHost) {
    register_foreign_handlers(host);
    register_effect_handlers(host);
}

fn register_foreign_handlers(host: &mut NativeHost) {
    host.register_foreign_handler("musi:process::argCountIntrinsic", |_foreign, args| {
        if !args.is_empty() {
            return Err(VmError::new(musi_vm::VmErrorKind::ForeignCallRejected {
                foreign: "musi:process::argCountIntrinsic".into(),
            }));
        }
        Ok(Value::Int(saturating_usize_to_i64(args_os().count())))
    });
    host.register_foreign_handler_with_context(
        "musi:process::argAtIntrinsic",
        |ctx, foreign, args| {
            let [Value::Int(index)] = args else {
                return Err(foreign_rejected(foreign));
            };
            let arg_value = usize::try_from(*index).map_or_else(
                |_| String::new(),
                |index| {
                    args_os()
                        .nth(index)
                        .map(|arg| arg.to_string_lossy().into_owned())
                        .unwrap_or_default()
                },
            );
            ctx.alloc_string(arg_value)
        },
    );
    host.register_foreign_handler_with_context(
        "musi:process::cwdIntrinsic",
        |ctx, foreign, args| {
            if !args.is_empty() {
                return Err(foreign_rejected(foreign));
            }
            current_dir()
                .map(|cwd| cwd.to_string_lossy().into_owned())
                .map_err(|_| foreign_rejected(foreign))
                .and_then(|cwd| ctx.alloc_string(cwd))
        },
    );
    host.register_foreign_handler_with_context(
        "musi:process::runIntrinsic",
        |ctx, foreign, args| {
            let command = foreign_string_arg(ctx, foreign, args)?;
            let status = if cfg!(windows) {
                Command::new("cmd")
                    .args([windows_shell_flag().as_str(), command])
                    .status()
            } else {
                Command::new("sh").args(["-c", command]).status()
            };
            Ok(Value::Int(i64::from(
                status
                    .map_err(|_| foreign_rejected(foreign))?
                    .code()
                    .unwrap_or(-1),
            )))
        },
    );
    host.register_foreign_handler("musi:process::exitIntrinsic", |foreign, _args| {
        Err(VmError::new(musi_vm::VmErrorKind::ForeignCallRejected {
            foreign: foreign.name().into(),
        }))
    });
}

fn register_effect_handlers(host: &mut NativeHost) {
    host.register_effect_handler(
        foundation_process::EFFECT,
        foundation_process::ARG_COUNT_OP,
        |effect, args| {
            if !args.is_empty() {
                return Err(invalid_runtime_args(effect, "no arguments", args.len()));
            }
            Ok(Value::Int(saturating_usize_to_i64(args_os().count())))
        },
    );

    host.register_effect_handler_with_context(
        foundation_process::EFFECT,
        foundation_process::ARG_AT_OP,
        |ctx, effect, args| {
            let [Value::Int(index)] = args else {
                return Err(invalid_runtime_args(effect, "integer index", args.len()));
            };
            let arg_value = usize::try_from(*index).map_or_else(
                |_| String::new(),
                |index| {
                    args_os()
                        .nth(index)
                        .map(|arg| arg.to_string_lossy().into_owned())
                        .unwrap_or_default()
                },
            );
            ctx.alloc_string(arg_value)
        },
    );

    host.register_effect_handler_with_context(
        foundation_process::EFFECT,
        foundation_process::CWD_OP,
        |ctx, effect, args| {
            if !args.is_empty() {
                return Err(invalid_runtime_args(effect, "no arguments", args.len()));
            }
            let cwd = current_dir().map_err(|error| runtime_effect_failed(effect, error))?;
            ctx.alloc_string(cwd.to_string_lossy().into_owned())
        },
    );

    host.register_effect_handler_with_context(
        foundation_process::EFFECT,
        foundation_process::RUN_OP,
        |ctx, effect, args| {
            let command = string_arg(ctx, effect, args, "processRun")?;
            Ok(Value::Int(run_shell_command(command, effect)?))
        },
    );

    host.register_effect_handler(
        foundation_process::EFFECT,
        foundation_process::EXIT_OP,
        |effect, args| {
            let [Value::Int(_code)] = args else {
                return Err(invalid_runtime_args(effect, "integer code", args.len()));
            };
            Err(runtime_effect_unsupported(effect))
        },
    );
}

fn run_shell_command(command: &str, effect: &EffectCall) -> Result<i64, VmError> {
    let status = if cfg!(windows) {
        Command::new("cmd")
            .args([windows_shell_flag().as_str(), command])
            .status()
    } else {
        Command::new("sh").args(["-c", command]).status()
    }
    .map_err(|error| runtime_effect_failed(effect, error))?;
    Ok(i64::from(status.code().unwrap_or(-1)))
}

fn windows_shell_flag() -> String {
    ['/', 'C'].into_iter().collect()
}
