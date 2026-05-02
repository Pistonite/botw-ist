/// Enable register Read and Write traces
#[macro_export]
macro_rules! trace_register {
    ($($arg:tt)*) => {
        #[cfg(feature = "trace-register")]
        {
            cu::trace!($($arg)*);
        }
    };
}

/// Enable memory Read and Write traces
#[macro_export]
macro_rules! trace_memory {
    ($($arg:tt)*) => {
        #[cfg(feature = "trace-memory")]
        {
            cu::trace!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! trace_call {
    ($($arg:tt)*) => {
        #[cfg(feature = "trace-call")]
        {
            cu::trace!($($arg)*);
        }
    };
}
