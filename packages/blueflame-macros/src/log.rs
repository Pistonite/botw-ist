/// Enable register Read and Write traces
#[macro_export]
macro_rules! trace_register {
    ($($arg:tt)*) => {
        #[cfg(feature = "trace-register")]
        {
            log::trace!($($arg)*);
        }
    };
}

/// Enable memory Read and Write traces
#[macro_export]
macro_rules! trace_memory {
    ($($arg:tt)*) => {
        #[cfg(feature = "trace-memory")]
        {
            log::trace!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! trace_call {
    ($($arg:tt)*) => {
        #[cfg(feature = "trace-call")]
        {
            log::trace!($($arg)*);
        }
    };
}
