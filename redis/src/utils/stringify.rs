use std::collections::HashMap;

pub fn stringify_map(map: &HashMap<&String, &HashMap<String, String>>) -> String {
    let mut result = String::from("{");

    for (i, (outer_key, inner_map)) in map.iter().enumerate() {
        result.push_str(&format!("\"{}\":{{", outer_key));

        for (j, (k, v)) in inner_map.iter().enumerate() {
            result.push_str(&format!("\"{}\":\"{}\"", k, v));
            if j < inner_map.len() - 1 {
                result.push(',');
            }
        }

        result.push('}');
        if i < map.len() - 1 {
            result.push(',');
        }
    }

    result.push('}');
    result
}