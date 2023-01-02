/// Like [`format_args`](`::core::format_args`), but takes ownership of its arguments.
#[macro_export]
macro_rules! format_args_owned {
    ($($args:tt)*) => {
        $crate::fmt_fn(
            move |__fmt_tools_formatter| ::core::fmt::Display::fmt(
                &::core::format_args!($($args)*),
                __fmt_tools_formatter,
            )
        )
    };
}
