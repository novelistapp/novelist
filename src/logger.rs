use chrono;
use fern;
use log;
use std::io;

pub(crate) fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} | {}  | {} | {}",
                chrono::Local::now().format("(%H:%M:%S)"),
                record.target(),
                record.level(),
                message
            ))
        }).level(log::LevelFilter::Debug)
        .chain(io::stdout())
        // .chain(fern::log_file("novelist.log")?)
        .apply()?;
    Ok(())
}
