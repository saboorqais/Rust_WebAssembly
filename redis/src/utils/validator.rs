pub trait CommandValidator {
    fn validate(parts: &Vec<&str>) -> Result<(), String>;
}
pub struct LPushValidator;
pub struct SetValidator;
pub struct LPopValidator;
pub struct XADDValidator;
pub struct XGROUPADDValidator;
pub struct XGROUPREADValidator;

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
impl CommandValidator for XGROUPREADValidator {
    fn validate(parts: &Vec<&str>) -> Result<(), String> {
        if parts.len() < 6 {
            return Err("-ERR wrong number of arguments for 'XREADGROUP'\n".to_string());
        }

        if parts[1] != "GROUP" {
            return Err("-ERR second argument must be 'GROUP'\n".to_string());
        }

        let mut i = 4; // Start after `XREADGROUP GROUP <group> <consumer>`

        // Optional COUNT
        let mut count_seen = false;
        if parts.get(i).map(|s| *s) == Some("COUNT") {
            count_seen = true;
            if parts.len() <= i + 1 {
                return Err("-ERR COUNT must be followed by a number\n".to_string());
            }
            if parts[i + 1].parse::<usize>().is_err() {
                return Err("-ERR COUNT value must be a valid integer\n".to_string());
            }
            i += 2; // Skip COUNT and its number
        }

        // Expect STREAMS keyword
        if parts.get(i).map(|s| *s) != Some("STREAMS") {
            return Err("-ERR expected 'STREAMS' keyword\n".to_string());
        }
        i += 1;

        let remaining = &parts[i..].to_vec();
        println!(" I am printing {:?}",remaining);
        let middle_index = remaining.len() /2;
        if remaining.get(middle_index).copied() != Some(">"){
            return Err("-ERR must specify equal number of stream names and IDs\n".to_string());
        }
        if remaining.len() % 2 == 0 || remaining.is_empty() {
            return Err("-ERR must specify equal number of stream names and IDs\n".to_string());
        }

        Ok(())
    }
}

impl CommandValidator for XGROUPADDValidator {
    fn validate(parts: &Vec<&str>) -> Result<(), String> {
        let arguments_length = parts.len() - 3;
        if parts.len() < 5 {
            Err("-ERR wrong number of arguments for 'XGROUP ADD'\n".to_string())
        } else if !(arguments_length % 2 == 0) {
            Err("-ERR wrong number of arguments for 'XAdd' Values\n".to_string())
        }
        else if parts[2]!="GROUP" {
            Err("-ERR Second Argument should be GROUP".to_string())
        }
        else {
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
