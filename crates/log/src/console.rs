use termion::color;

use crate::{LogEntry, LogSeverity, log_hook};

pub struct Console;

impl Console {
	pub fn init() {
		log_hook(|entry: &LogEntry| {
			println!(
				"{}[{}][{}] {}",
				color_for_entry(&entry),
				entry.date.to_rfc3339(),
				entry.severity.to_string(),
				entry.buffer
			);
		});
	}
}

/// Figures out and returns the correct color for the entry.
///
/// # Arguments
///
/// `entry` - The entry to extract the color.
fn color_for_entry(entry: &LogEntry) -> String {
	match entry.severity {
		LogSeverity::INFO => format!("{}", color::Fg(color::Blue)),
		LogSeverity::WARNING => format!("{}", color::Fg(color::Yellow)),
		LogSeverity::ERROR => format!("{}", color::Fg(color::Red))
	}
}