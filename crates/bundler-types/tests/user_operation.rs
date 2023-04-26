#[cfg(test)]
pub mod test {
    use bundler_types::user_operation::UserOperation;

    #[test]
    fn test_fail_user_operation_constructor() {
        let sender = "0x1";
        let nonce = "1";
        let init_code = "0x1";
        let call_data = "0x1";

        let user_operation = UserOperation::new(sender, nonce, init_code, call_data);
        println!("{:?}", user_operation);
    }
}
