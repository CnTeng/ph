use clap_complete::CompletionCandidate;

use crate::config::Config;

pub fn root_completer(_: &std::ffi::OsStr) -> Vec<CompletionCandidate> {
	if let Ok(config) = Config::load() {
		config.persistence.keys().map(|path| path.into()).collect()
	} else {
		Vec::new()
	}
}
