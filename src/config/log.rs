use tracing_error::ErrorLayer;
use tracing_subscriber::{EnvFilter, fmt};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init_log() {

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(ErrorLayer::default())
        .with(fmt::layer().with_writer(std::io::stdout))
        .init();
}

/*
pub fn init_log() {
    let mut cfg = Config::new().level(parse_log_level(&CONTEXT.config.log.level));
    cfg = cfg.file_split(&CONTEXT.config.log.dir,
                         Rolling::new(parse_rolling_type(CONTEXT.config.log.rolling.as_str())),
                         parse_keep_type(&CONTEXT.config.log.keep_type),
                         parse_packer(&CONTEXT.config.log.pack_compress),
    );
    cfg = cfg.chan_len(CONTEXT.config.log.chan_len);
    let _ = fast_log::init(cfg);
}

fn parse_rolling_type(log_rolling: &str) -> RollingType {
    let lower = log_rolling.to_lowercase();
    let rolling_type;
    if log_rolling.ends_with("B") {
        rolling_type = RollingType::BySize(parse_log_size(&CONTEXT.config.log.rolling));
    } else if lower.as_str().ends_with("minute")
        || lower.as_str().ends_with("hour")
        || lower.as_str().ends_with("day") {
        match lower.as_str() {
            "minute" => {
                rolling_type = RollingType::ByDate(DateType::Minute);
            }
            "hour" => {
                rolling_type = RollingType::ByDate(DateType::Hour);
            }
            "day" => {
                rolling_type = RollingType::ByDate(DateType::Day);
            }
            _ => {
                if lower.ends_with("minute") {
                    let value: u64 = lower.trim_end_matches("minute").parse().expect("parse number fail");
                    rolling_type = RollingType::ByDuration((DateTime::now().0, Duration::from_secs(value * 60)));
                } else if lower.ends_with("hour") {
                    let value: u64 = lower.trim_end_matches("hour").parse().expect("parse number fail");
                    rolling_type = RollingType::ByDuration((DateTime::now().0, Duration::from_secs(value * 60 * 60)));
                } else if lower.ends_with("day") {
                    let value: u64 = lower.trim_end_matches("day").parse().expect("parse number fail");
                    rolling_type = RollingType::ByDuration((DateTime::now().0, Duration::from_secs(value * 24 * 60 * 60)));
                } else {
                    panic!("unknown log_rolling '{}'", log_rolling);
                }
            }
        }
    } else {
        panic!("unknown log_rolling '{}'", log_rolling);
    }
    return rolling_type;
}

fn parse_packer(packer: &str) -> Box<dyn Packer> {
    match packer {
        // "lz4" => Box::new(fast_log::plugin::packer::LZ4Packer {}),
        // "zip" => Box::new(fast_log::plugin::packer::ZipPacker {}),
        // "gzip" => Box::new(fast_log::plugin::packer::GZipPacker {}),
        _ => Box::new(fast_log::plugin::packer::LogPacker {}),
    }
}

fn parse_log_size(arg: &str) -> LogSize {
    match arg {
        arg if arg.ends_with("MB") => {
            let end = arg.find("MB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::MB(num.parse::<usize>().unwrap())
        }
        arg if arg.ends_with("KB") => {
            let end = arg.find("KB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::KB(num.parse::<usize>().unwrap())
        }
        arg if arg.ends_with("GB") => {
            let end = arg.find("GB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::GB(num.parse::<usize>().unwrap())
        }
        _ => LogSize::MB(100),
    }
}

fn parse_keep_type(arg: &str) -> KeepType {
    match arg {
        arg if arg.starts_with("KeepNum(") => {
            let end = arg.find(")").unwrap();
            let num = arg["KeepNum(".len()..end].to_string();
            return KeepType::KeepNum(num.parse::<i64>().unwrap());
        }
        arg if arg.starts_with("KeepTime(") => {
            let end = arg.find(")").unwrap();
            let num = arg["KeepTime(".len()..end].to_string();
            return KeepType::KeepTime(Duration::from_secs(num.parse::<u64>().unwrap()));
        }
        arg if arg.to_uppercase().as_str() == "ALL" => {
            return KeepType::All;
        }
        _ => {
            panic!("unknown keep_type '{}'", arg)
        }
    }
}

fn parse_log_level(arg: &str) -> log::LevelFilter {
    return match arg {
        "off" => log::LevelFilter::Off,
        "warn" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        "trace" => log::LevelFilter::Trace,
        "info" => log::LevelFilter::Info,
        "debug" => log::LevelFilter::Debug,
        _ => log::LevelFilter::Info,
    };
}
*/
