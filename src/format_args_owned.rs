#[macro_export]
macro_rules! format_args_owned {
    ($($args:tt)*) => {
        $crate::fmt_fn::fmt_fn(
            move |__fmt_tools_formatter| ::core::fmt::Display::fmt(
                &::core::format_args!($($args)*),
                __fmt_tools_formatter,
            )
        )
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_format_args_owned() {
        let fmt = {
            let value = std::vec![4];

            crate::format_args_owned!("{:?}, {:?}", value, std::vec![5])
        };

        assert_eq!(std::format!("{}", fmt), "[4], [5]");
        assert_eq!(std::format!("{:?}", fmt), "[4], [5]");
    }
}
