use log::{Log, Console, info, warning, error};

fn main() {
	Log::init();
	Console::init();

	info("This is an info");
	warning("This is a warning");
	error("This is an error");
}