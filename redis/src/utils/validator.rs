pub trait CommandValidator {
    fn validate(parts: &Vec<&str>) -> Result<(), String>;
}
pub struct LPushValidator;
pub struct SetValidator;
pub struct LPopValidator;
pub struct XADDValidator;
pub struct XGROUPADDValidator;
impl CommandValidator for XADDValidator {
    fn validate(parts: &Vec<&str>) -> Result<(), String> {
        let arguments_length = parts.len() - 3;
        if parts.len() <= 4 {
            Err("-ERR wrong number of arguments for 'XAdd'\n".to_string())
        } else if !(arguments_length % 2 == 0) {
            Err("-ERR wrong number of arguments for 'XAdd' Values\n".to_string())
        } else {
            Ok(())
        }
    }
}
impl CommandValidator for XGROUPADDValidator {
    fn validate(parts: &Vec<&str>) -> Result<(), String> {
        let arguments_length = parts.len() - 3;
        if parts.len() < 4 {
            Err("-ERR wrong number of arguments for 'XGROUP ADD'\n".to_string())
        } else if !(arguments_length % 2 == 0) {
            Err("-ERR wrong number of arguments for 'XAdd' Values\n".to_string())
        } else {
            Ok(())
        }
    }
}
impl CommandValidator for SetValidator {
    fn validate(parts: &Vec<&str>) -> Result<(), String> {
        if parts.len() < 3 {
            Err("-ERR wrong number of arguments for 'SET'\n".to_string())
        } else {
            Ok(())
        }
    }
}

impl CommandValidator for LPushValidator {
    fn validate(parts: &Vec<&str>) -> Result<(), String> {
        if parts.len() != 3 {
            Err("-ERR wrong number of arguments for 'lpush'\n".to_string())
        } else {
            Ok(())
        }
    }
}
impl CommandValidator for LPopValidator {
    fn validate(parts: &Vec<&str>) -> Result<(), String> {
        if parts.len() != 2 {
            Err("-ERR wrong number of arguments for 'lpop'\n".to_string())
        } else {
            Ok(())
        }
    }
}
