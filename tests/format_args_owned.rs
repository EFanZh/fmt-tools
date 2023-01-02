#[test]
fn test_format_args_owned() {
    let fmt = {
        let value_1 = vec![2];
        let value_2 = Box::new('A');
        let value_3 = Box::new(5);

        fmt_tools::format_args_owned!("{:?}, {named}, {value_3}", value_1, named = value_2)
    };

    assert_eq!(format!("{fmt:?}"), "[2], A, 5");
    assert_eq!(format!("{fmt}"), "[2], A, 5");
}
