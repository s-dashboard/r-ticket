use mysql::{prelude::{ FromRow, Queryable}, *};

pub fn get_conn() -> Result<Conn> {
    let url = "mysql://fredrik:password@localhost:3306/r-tickets";
    let pool = Pool::new(url)?;
    let conn = pool.get_conn()?;
    return Ok(conn.unwrap());
}

pub fn select<T,P,F,U>(query: String, params: P, mut f: F) -> std::result::Result<Vec<U>, mysql::Error> where 
    P: Into<Params>,
    T: FromRow,
    F: FnMut(T) -> U
{
    let mut conn: Conn = get_conn().unwrap();
    let stmt: Statement = conn.prep(query)?;
    let result = conn.exec_fold(stmt, params, Vec::new(), |mut acc, row| {
        acc.push(f(row));
        acc
    })?;

    Ok(result)
}