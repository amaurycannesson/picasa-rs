use std::time::Duration;

use picasa_core::utils::progress_reporter::ProgressReporter;

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
