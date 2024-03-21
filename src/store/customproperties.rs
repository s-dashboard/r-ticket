use mysql::{params, Value};
use serde::{Deserialize, Serialize};

use crate::{db::sql, routes::authentication::UserContext};
use super::store::Store;

pub type CustomProperties = Vec<CustomProperty>;

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct CustomProperty {
    pub id: i32,
    pub data_type: Option<String>,
    pub label: Option<String>,
    pub value: Option<String>
}

impl CustomProperty {}

pub async fn property_list(owner_type: String, owner_id: i32, _context: UserContext) -> Result<impl warp::Reply, warp::Rejection> {    
    let store: Store = Store::new();
    let result: Vec<CustomProperty> = select_properties(owner_type, owner_id);

    result.iter().for_each(|item|{
        store.property_list.write().push(item.clone())        
    });

    let store_result = store.property_list.read();
    Ok(warp::reply::json(&*store_result))
}

fn select_properties(owner_type: String, owner_id: i32) -> Vec<CustomProperty> {

    let query: String = String::from("SELECT 
	customproperties.id
	, customproperties.data_type
	, customproperties.lable
	, CASE 
		WHEN customproperties_values.id IS NULL THEN
			CONCAT('{\"type\":\"',customproperties.data_type,'\",\"value\":null}')
		ELSE
			CASE
				WHEN customproperties.data_type = 'int' THEN customproperties_values.int_value
				WHEN customproperties.data_type = 'text' THEN customproperties_values.text_value
				WHEN customproperties.data_type = 'decimal' THEN customproperties_values.decimal_value          
				WHEN customproperties.data_type = 'boolean' THEN 
					CASE WHEN customproperties_values.boolean_value = 1 THEN 'true' ELSE 'false' END
				ELSE 
					'null'
		END
	END As property_value FROM customproperties
	    LEFT JOIN customproperties_values ON customproperties.id = customproperties_values.customproperty_id
	    WHERE owner_type = :owner_type 
		    AND (customproperties_values.owner_id = :owner_id OR customproperties_values.owner_id IS NULL)
		    AND customproperties.enabled = 1").to_owned();

    let result: Vec<CustomProperty> = sql::select(query.to_string(), params! {
        "owner_type" => &owner_type,
        "owner_id" => &owner_id
    }, property_selector()).unwrap();

    return result;
}

fn property_selector() -> impl Fn((Value, Value, Value, Value)) -> CustomProperty
{
    let selector = |(id, data_type, lable, property_value)|
    
    CustomProperty {
        id: mysql::from_value(id),
        data_type: mysql::from_value(data_type),
        label: mysql::from_value(lable),
        value: mysql::from_value(property_value)
    };

    return selector;
}
