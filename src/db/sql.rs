use mysql::{prelude::{ FromRow, Queryable}, *};
use dotenv::dotenv;
use std::env;

pub fn get_conn() -> Result<PooledConn> {
    dotenv().ok();
    
    let connection_string = env::var("CONNECTION_STRING").expect("CONNECTION_STRING must be set");   
    let connection_opts = mysql::Opts::from_url(&connection_string)
        .unwrap();

    let pool = Pool::new(connection_opts)?;
    let conn = pool.get_conn().unwrap(); 
    return Ok(conn);
}

pub fn select<T,P,F,U>(query: String, params: P, mut f: F) -> std::result::Result<Vec<U>, mysql::Error> where 
    P: Into<Params>,
    T: FromRow,
    F: FnMut(T) -> U
{
    let mut conn: PooledConn = get_conn().unwrap();
    let stmt: Statement = conn.prep(query)?;
    let result = conn.exec_fold(stmt, params, Vec::new(), |mut acc, row| {
        acc.push(f(row));
        acc
    })?;

    Ok(result)
}