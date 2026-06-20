pub trait Logger {
    /// Log a message at the given verbosity level.
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

struct MsgFilter<F> {
    logger: StderrLogger,
    msgfilter: F,
}

impl<F: Fn(u8, &str) -> bool> MsgFilter<F> {
    fn new(logger: StderrLogger, msgfilter: F) -> Self {
        MsgFilter{logger, msgfilter}
    }
}

impl<F: Fn(u8, &str) -> bool> Logger for MsgFilter<F> {
    fn log(&self, verbosity: u8, message: &str) {
        if (self.msgfilter)(verbosity, message) {
            self.logger.log(verbosity, message);
        }
    }
}

fn main() {
    let logger = MsgFilter::new(StderrLogger, |_verbosity, msg| msg.contains("yikes"));
    logger.log(5, "FYI");
    logger.log(1, "yikes, something went wrong");
    logger.log(2, "uhoh");
}
