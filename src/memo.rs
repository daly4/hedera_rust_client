use crate::error::HederaError;

pub fn check_memo_length(val: &String) -> Result<(), HederaError> {
    if val.len() <= 100 {
        return Ok(());
    }
    Err(HederaError::MemoLength(val.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_memo() {
        // empty string
        let memo = String::new();
        check_memo_length(&memo).unwrap();

        // ok string
        let memo = "This is a test".to_string();
        check_memo_length(&memo).unwrap();

        // at limit
        let buf = [0; 100].to_vec();
        let memo = String::from_utf8(buf).unwrap();
        check_memo_length(&memo).unwrap();

        // over limit
        let buf = [0; 101].to_vec();
        let memo = String::from_utf8(buf).unwrap();
        let r = check_memo_length(&memo);
        assert!(r.is_err());
    }
}
