use std::str::FromStr;

use log::Level;

use crate::result::Result;

pub fn init_log(log_level: &str) -> Result<()> {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(Level::from_str(log_level)?.to_level_filter())
        .try_init()?;
    Ok(())
}
