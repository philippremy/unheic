use std::{ffi::OsStr, process::exit};

use backtrace::Backtrace;
use log::error;
use native_dialog::DialogBuilder;

pub(crate) fn register_panic_handler() -> crate::Result<()> {
    std::panic::set_hook(Box::new(move |panic_info| {
        error!("------------[ cut here ]------------");
        error!(
            "Thread panic: {}",
            panic_info
                .payload_as_str()
                .map(|non_owned| { non_owned.to_string() })
                .unwrap_or_else(|| { format!("{:?}", panic_info.payload()) })
        );
        error!(
            "Error location: {}",
            panic_info
                .location()
                .map(|loc| { format!("{}:{}", loc.file(), loc.line()) })
                .unwrap_or("N/A".to_string())
        );

        let mut generate_backtrace = false;

        // Ask if we should generate a backtrace
        if let Ok(answer) = DialogBuilder::message()
            .set_text(format!("Ein Fehler ist aufgetreten und das Programm muss beendet werden:\n\nFehler: {}.\n\nSoll ein Call Trace generiert und zu Verbesserungszwecken an den Entwickler gesendet werden (enthält keine persönlichen Informationen)?", panic_info.payload_as_str().map(|non_owned| { non_owned.to_string() }).unwrap_or_else(|| { format!("{:?}", panic_info.payload()) })))
            .set_title("Laufzeitfehler")
            .confirm()
            .show()
        {
            generate_backtrace = answer;
        }

        if generate_backtrace {
            let backtrace = Backtrace::new();
            error!("Call Trace:");
            for frame in backtrace.frames().iter() {
                if let Some(symbol) = frame.symbols().first() {
                    error!(
                        "  [<{:#?}>] {} ({}:{}:{})",
                        frame.symbol_address(),
                        symbol
                            .name()
                            .map(|symbol_name| { format!("{}", symbol_name) })
                            .unwrap_or("N/A".to_string()),
                        symbol
                            .filename()
                            .map(|path| { path.file_name().unwrap() })
                            .unwrap_or(OsStr::new("N/A"))
                            .display(),
                        symbol.lineno().unwrap_or(0),
                        symbol.colno().unwrap_or(0),
                    );
                } else {
                    error!("  [<{:#?}>] N/A", frame.symbol_address());
                }
            }
            error!(
                "---[ end trace {:#?} ]---",
                (&backtrace) as *const Backtrace
            );
        } else {
            error!("Backtrace:  Not requested.");
            error!("---[ end trace ]---");
        }
        exit(-1);
    }));
    Ok(())
}
