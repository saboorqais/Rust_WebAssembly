use crate::utils::validator::XGROUPADDValidator;
use crate::Logger;
use crate::types::*;
use crate::utils::validator::{
    CommandValidator, LPopValidator, LPushValidator, SetValidator, XADDValidator,
};
pub fn execute_command(parts: Vec<&str>, db: &Db, cache: &CACHE, logging: bool) -> String {
    if parts.is_empty() {
        return "-ERR empty command\n".to_string();
    }
    match parts[0].to_uppercase().as_str() {
        "SET" => {
            validate_or_return!(SetValidator, parts);
            if logging {
                Logger::log_aof(&parts);
            }
            RedisValue::set(parts, db, cache);
            "+OK\n".to_string()
        }
        "LPUSH" => {
            validate_or_return!(LPushValidator, parts);
            if logging {
                Logger::log_aof(&parts);
            }
            RedisValue::lpush(parts, db, cache);
            "+OK\n".to_string()
        }
        "LPOP" => {
            validate_or_return!(LPopValidator, parts);
            if logging {
                Logger::log_aof(&parts);
            }
            RedisValue::lpop(parts, db, cache);
            "+OK\n".to_string()
        }
        "XADD"  => {
            validate_or_return!(XADDValidator, parts);
            if logging {
                Logger::log_aof(&parts);
            }
            println!("Hash({:?})", parts);
            let response = RedisValue::x_add(parts, db);
            response
        }
        "XGROUPADD"  => {
            validate_or_return!(XGROUPADDValidator, parts);
            if logging {
                Logger::log_aof(&parts);
            }
            println!("Hash({:?})", parts);
            let response = RedisValue::x_group_add(parts, db);
            response
        }
        "XREAD"  => {
            // validate_or_return!(XADDValidator, parts);
            if logging {
                Logger::log_aof(&parts);
            }
            println!("Hash({:?})", parts);
            let response = RedisValue::x_read(parts, db);
            response
        }
        "EXPIRE" if parts.len() == 3 => {
            if logging {
                Logger::log_aof(&parts);
            }
            RedisValue::expire(parts, db, cache);
            "+OK\n".to_string()
        }
        "GET" if parts.len() == 2 && parts[1] == "*" => {
            let response = RedisValue::get_all(db);
            response.to_string()
        }
        "GET" if parts.len() == 2 => {
            let response = RedisValue::get_key(parts, db);
            response.to_string()
        }
        "DEL" if parts.len() == 2 => {
            if logging {
                Logger::log_aof(&parts);
            }

            RedisValue::remove(parts, db);
            "+OK\n".to_string()
        }
        "FLUSH" if parts.len() == 1 => {
            db.lock().unwrap().clear();
            let response = Logger::remove_aof();
            response
        }
        "EXIT" => "EXIT".to_string(), // for handling shutdown in server
        _ => "-ERR unknown or unsupported command\n".to_string(),
    }
}
