use mysql::{prelude::{ FromRow, Queryable}, *};

use crate::{config::globals::with_settings, routes::authentication::UserContext};

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

pub fn execute_with_id<P, I>(query: String, params: I) -> Result<i32,> where
    P: Into<Params>,
    I: IntoIterator<Item = P>
{
    let mut conn: PooledConn = get_conn().unwrap();
    let stmt: Statement = conn.prep(query)?;

    let mut tx = conn.start_transaction(TxOpts::default())?; 
    let _ = tx.exec_batch(stmt, params);

    let result: Option<i32> = tx.query_first("SELECT LAST_INSERT_ID();")?;
    let _ = tx.commit();

    return Ok(result.unwrap());
}

pub fn execute<P, I>(query: String, params: I) -> Result<()> where
    P: Into<Params>,
    I: IntoIterator<Item = P>
{
    let mut conn: PooledConn = get_conn().unwrap();
    let stmt: Statement = conn.prep(query)?;

    conn.exec_batch(stmt, params)
}

pub fn mark_as_deleted(table_name: String, primary_key: i32, context: UserContext) -> Result<(), Error> {

    let update_sql = format!("UPDATE {table_name} SET deleted_by = :user_id, deleted = Now() WHERE id = :id");
    let result = execute(update_sql, vec![primary_key].iter().map(|p: &i32| params! {
        "id" => p, 
        "user_id" => context.user_id
    }));

    result
}