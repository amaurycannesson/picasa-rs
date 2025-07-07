/// Trait for reporting progress during long-running operations
pub trait ProgressReporter {
    /// Set a message to display
    fn set_message(&self, message: String);

    /// Finish with a final message
    fn finish_with_message(&self, message: String);
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
