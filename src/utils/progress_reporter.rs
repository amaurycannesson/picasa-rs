use std::time::Duration;

/// Trait for reporting progress during long-running operations
pub trait ProgressReporter {
    /// Set a message to display
    fn set_message(&self, message: String);

    /// Finish with a final message
    fn finish_with_message(&self, message: String);
}

/// CLI implementation using indicatif
pub struct CliProgressReporter {
    spinner: indicatif::ProgressBar,
}

impl CliProgressReporter {
    pub fn new() -> Self {
        use indicatif::{ProgressBar, ProgressStyle};

        let spinner = ProgressBar::new_spinner().with_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} [{elapsed_precise}] {msg}")
                .unwrap(),
        );

        spinner.enable_steady_tick(Duration::from_millis(100));

        Self { spinner }
    }
}

impl ProgressReporter for CliProgressReporter {
    fn set_message(&self, message: String) {
        self.spinner.set_message(message);
    }

    fn finish_with_message(&self, message: String) {
        self.spinner.finish_with_message(message);
    }
}

/// No-op implementation for non-CLI contexts (web, MCP, etc.)
pub struct NoOpProgressReporter;

impl ProgressReporter for NoOpProgressReporter {
    fn set_message(&self, _message: String) {
        // No-op
    }

    fn finish_with_message(&self, _message: String) {
        // No-op
    }
}
