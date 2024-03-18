
// SELECT 
// 	CASE 
// 		WHEN customproperties.data_type = 'int' THEN 
// 			CONCAT('{ "type":"',customproperties.data_type,'","value":',customproperties_values.int_value,'}')
// 		WHEN customproperties.data_type = 'text' THEN 
// 			CONCAT('{ "type":"',customproperties.data_type,'","value":"',customproperties_values.text_value,'"}')
// 		WHEN customproperties.data_type = 'decimal' THEN 
// 			CONCAT('{ "type":"',customproperties.data_type,'","value":',customproperties_values.decimal_value,'}')            
// 		WHEN customproperties.data_type = 'boolean' THEN 
// 			CONCAT('{ "type":"',customproperties.data_type,'","value":', 
//             CASE WHEN customproperties_values.boolean_value = 1 THEN 'true' ELSE 'false' END
// 		,'}')
// 		ELSE
// 			CONCAT('{"type":"unknown","value":null}')          
//     END AS property_value
// FROM customproperties
// 	JOIN customproperties_values ON customproperties.id = customproperties_values.customproperty_id
// 	WHERE owner_type = 'tickets' AND customproperties_values.owner_id = 1