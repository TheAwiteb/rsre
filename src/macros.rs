/// execute the block if `debug` feature is enabled
#[macro_export(local_inner_macros)]
macro_rules! debug {
    ($e: expr) => {
        #[cfg(feature = "debug")]
        {
            $e
        }
    };
    ($b: block) => {
        #[cfg(feature = "debug")]
        $b
    };
}
