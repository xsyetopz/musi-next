use std::io;
use std::sync::Arc;

use musi_foundation::{io as foundation_io, log as foundation_log};
use musi_native::NativeHost;
use musi_vm::{EffectCall, ForeignCall, Value, VmError, VmHostContext};

use crate::output::RuntimeOutputSinkCell;

use super::errors::{
    foreign_rejected, invalid_runtime_args, runtime_effect_failed, runtime_host_unavailable,
};
use super::values::string_arg;

pub(super) fn register(host: &mut NativeHost, output: &RuntimeOutputSinkCell) {
    register_foreign_log_handlers(host, output);
    register_foreign_io_handlers(host, output);
    register_log_effect_handlers(host, output);
    register_io_effect_handlers(host, output);
}

fn register_foreign_log_handlers(host: &mut NativeHost, output: &RuntimeOutputSinkCell) {
    let foreign_log_info_output = Arc::clone(output);
    host.register_foreign_handler_with_context(
        "musi:log::infoIntrinsic",
        move |ctx, foreign, args| {
            let message = foreign_string_arg(ctx, foreign, args)?;
            let line = format!("[musi:log] {message}");
            foreign_log_info_output
                .lock()
                .map_err(|_| foreign_rejected(foreign))?
                .write_stderr(&line, true)
                .map_err(|_| foreign_rejected(foreign))?;
            Ok(Value::Unit)
        },
    );
    let foreign_log_write_output = Arc::clone(output);
    host.register_foreign_handler_with_context(
        "musi:log::writeIntrinsic",
        move |ctx, foreign, args| {
            let [Value::Int(level), message] = args else {
                return Err(foreign_rejected(foreign));
            };
            let message = ctx
                .string(message)
                .ok_or_else(|| foreign_rejected(foreign))?;
            let line = format!("[std:{level}] {}", message.as_str());
            foreign_log_write_output
                .lock()
                .map_err(|_| foreign_rejected(foreign))?
                .write_stderr(&line, true)
                .map_err(|_| foreign_rejected(foreign))?;
            Ok(Value::Unit)
        },
    );
}

fn register_foreign_io_handlers(host: &mut NativeHost, output: &RuntimeOutputSinkCell) {
    let foreign_stdout_output = Arc::clone(output);
    host.register_foreign_handler_with_context(
        "musi:io::writeIntrinsic",
        move |ctx, foreign, args| {
            write_foreign_stream(
                ctx,
                foreign,
                args,
                StreamKind::Stdout,
                false,
                &foreign_stdout_output,
            )
        },
    );
    let foreign_stdout_line_output = Arc::clone(output);
    host.register_foreign_handler_with_context(
        "musi:io::writeLnIntrinsic",
        move |ctx, foreign, args| {
            write_foreign_stream(
                ctx,
                foreign,
                args,
                StreamKind::Stdout,
                true,
                &foreign_stdout_line_output,
            )
        },
    );
    let foreign_stderr_output = Arc::clone(output);
    host.register_foreign_handler_with_context(
        "musi:io::writeErrIntrinsic",
        move |ctx, foreign, args| {
            write_foreign_stream(
                ctx,
                foreign,
                args,
                StreamKind::Stderr,
                false,
                &foreign_stderr_output,
            )
        },
    );
    let foreign_stderr_line_output = Arc::clone(output);
    host.register_foreign_handler_with_context(
        "musi:io::writeErrLnIntrinsic",
        move |ctx, foreign, args| {
            write_foreign_stream(
                ctx,
                foreign,
                args,
                StreamKind::Stderr,
                true,
                &foreign_stderr_line_output,
            )
        },
    );
    host.register_foreign_handler_with_context(
        "musi:io::readLineIntrinsic",
        |ctx, foreign, args| {
            if !args.is_empty() {
                return Err(foreign_rejected(foreign));
            }
            ctx.alloc_string(read_line_foreign(foreign)?)
        },
    );
}

fn register_log_effect_handlers(host: &mut NativeHost, output: &RuntimeOutputSinkCell) {
    let log_info_output = Arc::clone(output);
    host.register_effect_handler_with_context(
        foundation_log::EFFECT,
        foundation_log::INFO_OP,
        move |ctx, effect, args| {
            let message = string_arg(ctx, effect, args, "logInfo")?;
            let line = format!("[musi:log] {message}");
            log_info_output
                .lock()
                .map_err(|_| runtime_host_unavailable(effect, "runtime output lock"))?
                .write_stderr(&line, true)
                .map_err(|error| runtime_effect_failed(effect, error))?;
            Ok(Value::Unit)
        },
    );

    let log_write_output = Arc::clone(output);
    host.register_effect_handler_with_context(
        foundation_log::EFFECT,
        foundation_log::WRITE_OP,
        move |ctx, effect, args| {
            let [Value::Int(level), message] = args else {
                return Err(invalid_runtime_args(
                    effect,
                    "integer level and string message",
                    args.len(),
                ));
            };
            let message = ctx
                .string(message)
                .ok_or_else(|| invalid_runtime_args(effect, "string message", message.kind()))?;
            let line = format!("[std:{level}] {}", message.as_str());
            log_write_output
                .lock()
                .map_err(|_| runtime_host_unavailable(effect, "runtime output lock"))?
                .write_stderr(&line, true)
                .map_err(|error| runtime_effect_failed(effect, error))?;
            Ok(Value::Unit)
        },
    );
}

fn register_io_effect_handlers(host: &mut NativeHost, output: &RuntimeOutputSinkCell) {
    let stdout_output = Arc::clone(output);
    host.register_effect_handler_with_context(
        foundation_io::EFFECT,
        foundation_io::WRITE_OP,
        move |ctx, effect, args| {
            write_stream(ctx, effect, args, StreamKind::Stdout, false, &stdout_output)
        },
    );
    let stdout_line_output = Arc::clone(output);
    host.register_effect_handler_with_context(
        foundation_io::EFFECT,
        foundation_io::WRITE_LN_OP,
        move |ctx, effect, args| {
            write_stream(
                ctx,
                effect,
                args,
                StreamKind::Stdout,
                true,
                &stdout_line_output,
            )
        },
    );
    let stderr_output = Arc::clone(output);
    host.register_effect_handler_with_context(
        foundation_io::EFFECT,
        foundation_io::WRITE_ERR_OP,
        move |ctx, effect, args| {
            write_stream(ctx, effect, args, StreamKind::Stderr, false, &stderr_output)
        },
    );
    let stderr_line_output = Arc::clone(output);
    host.register_effect_handler_with_context(
        foundation_io::EFFECT,
        foundation_io::WRITE_ERR_LN_OP,
        move |ctx, effect, args| {
            write_stream(
                ctx,
                effect,
                args,
                StreamKind::Stderr,
                true,
                &stderr_line_output,
            )
        },
    );
    host.register_effect_handler_with_context(
        foundation_io::EFFECT,
        foundation_io::READ_LINE_OP,
        |ctx, effect, args| {
            if !args.is_empty() {
                return Err(invalid_runtime_args(effect, "no arguments", args.len()));
            }
            ctx.alloc_string(read_line(effect)?)
        },
    );
}

#[derive(Clone, Copy)]
enum StreamKind {
    Stdout,
    Stderr,
}

fn write_stream(
    ctx: &VmHostContext<'_>,
    effect: &EffectCall,
    args: &[Value],
    stream: StreamKind,
    line: bool,
    output: &RuntimeOutputSinkCell,
) -> Result<Value, VmError> {
    let [text] = args else {
        return Err(invalid_runtime_args(effect, "one string", args.len()));
    };
    let Some(text) = ctx.string(text) else {
        return Err(invalid_runtime_args(effect, "string argument", text.kind()));
    };
    let write_result = match stream {
        StreamKind::Stdout => output
            .lock()
            .map_err(|_| runtime_host_unavailable(effect, "runtime output lock"))?
            .write_stdout(text.as_str(), line),
        StreamKind::Stderr => output
            .lock()
            .map_err(|_| runtime_host_unavailable(effect, "runtime output lock"))?
            .write_stderr(text.as_str(), line),
    };
    write_result.map_err(|error| runtime_effect_failed(effect, error))?;
    Ok(Value::Unit)
}

fn read_line(effect: &EffectCall) -> Result<String, VmError> {
    let mut line = String::new();
    let _bytes = io::stdin()
        .read_line(&mut line)
        .map_err(|error| runtime_effect_failed(effect, error))?;
    if line.ends_with('\n') {
        let _ = line.pop();
        if line.ends_with('\r') {
            let _ = line.pop();
        }
    }
    Ok(line)
}

fn foreign_string_arg<'a>(
    ctx: &'a VmHostContext<'_>,
    foreign: &ForeignCall,
    args: &'a [Value],
) -> Result<&'a str, VmError> {
    let [value] = args else {
        return Err(foreign_rejected(foreign));
    };
    ctx.string(value)
        .map(|text| text.as_str())
        .ok_or_else(|| foreign_rejected(foreign))
}

fn write_foreign_stream(
    ctx: &VmHostContext<'_>,
    foreign: &ForeignCall,
    args: &[Value],
    stream: StreamKind,
    line: bool,
    output: &RuntimeOutputSinkCell,
) -> Result<Value, VmError> {
    let text = foreign_string_arg(ctx, foreign, args)?;
    let write_result = match stream {
        StreamKind::Stdout => output
            .lock()
            .map_err(|_| foreign_rejected(foreign))?
            .write_stdout(text, line),
        StreamKind::Stderr => output
            .lock()
            .map_err(|_| foreign_rejected(foreign))?
            .write_stderr(text, line),
    };
    write_result.map_err(|_| foreign_rejected(foreign))?;
    Ok(Value::Unit)
}

fn read_line_foreign(foreign: &ForeignCall) -> Result<String, VmError> {
    let mut line = String::new();
    let _bytes = io::stdin()
        .read_line(&mut line)
        .map_err(|_| foreign_rejected(foreign))?;
    if line.ends_with('\n') {
        let _ = line.pop();
        if line.ends_with('\r') {
            let _ = line.pop();
        }
    }
    Ok(line)
}
