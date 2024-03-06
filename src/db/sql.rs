use chrono::NaiveDateTime;
use mysql::{prelude::Queryable, *};
use crate::tickets_store::tickets::Ticket;

pub fn get_conn() -> Result<Conn> {
    let url = "mysql://fredrik:password@localhost:3306/r-tickets";
    let pool = Pool::new(url)?;
    let conn = pool.get_conn()?;
    return Ok(conn.unwrap());
}

pub fn select_tickets(state: String) -> std::result::Result<Vec<Ticket>, mysql::Error> {
    let mut conn: Conn = get_conn().unwrap();
    let stmt: Statement = conn.prep("SELECT 
        t.id
        , subject
        , ticket as content
        , state
        , s.title AS state_title 
        , UNIX_TIMESTAMP(created) as created_nix
        , UNIX_TIMESTAMP(changed) as changed_nix
    FROM tickets t JOIN states s ON s.id = t.state WHERE t.state = :state; ")?;
    
    let result = conn.exec_map(&stmt, params! {"state" => &state},
    |(id, subject, content, state, state_title, created_nix, changed_nix)|
        Ticket {
            id: mysql::from_value(id), 
            subject: mysql::from_value(subject), 
            content: mysql::from_value(content), 
            state: mysql::from_value(state),
            state_title: mysql::from_value(state_title),
            created: NaiveDateTime::from_timestamp_opt(mysql::from_value::<i64>(created_nix),0),
            changed: NaiveDateTime::from_timestamp_opt(mysql::from_value(changed_nix),0)
        }
    ).unwrap();

    return Ok(result);
}