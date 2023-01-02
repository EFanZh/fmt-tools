#[test]
fn test_format_args_owned() {
    let fmt = {
        let value = std::vec![4];
        let value_2 = Box::new(7);

        fmt_tools::format_args_owned!(
            "{:?}, {:?}, {name_argument}",
            value,
            std::vec![5],
            name_argument = value_2,
        )
    };

    assert_eq!(std::format!("{}", fmt), "[4], [5], 7");
    assert_eq!(std::format!("{:?}", fmt), "[4], [5], 7");
}
