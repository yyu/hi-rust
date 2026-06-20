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

// it seems we don't have to add constraints to T or F here
struct MsgFilter<T, F> {
    logger: T,
    msgfilter: F,
}

// we need constraint for F; remove it and see how compiler will complain
impl<T, F: Fn(u8, &str) -> bool> MsgFilter<T, F> {
    fn new(logger: T, msgfilter: F) -> Self {
        MsgFilter{logger, msgfilter}
    }
}

impl<T: Logger, F: Fn(u8, &str) -> bool> Logger for MsgFilter<T, F> {
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
