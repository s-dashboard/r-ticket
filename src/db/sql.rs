use mysql::{prelude::{ FromRow, Queryable}, *};

use crate::config::globals::with_settings;

pub fn get_conn() -> Result<PooledConn> {

    let settings = with_settings()
        .expect("settings are invalid");
    
    let connection_string = settings.connection_string;
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

pub fn execute<P, I>(query: String, params: I) -> Result<()> where
    P: Into<Params>,
    I: IntoIterator<Item = P>
{
    let mut conn: PooledConn = get_conn().unwrap();
    let stmt: Statement = conn.prep(query)?;

    conn.exec_batch(stmt, params)
}