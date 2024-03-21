use std::collections::HashMap;

pub fn get_search(mut _query: &HashMap<String, String>) -> String  {
    let mut state: &str = &"";
    
    if _query.contains_key("search") {
        state = _query.get("search").unwrap();
    }

    return state.to_string();
}
