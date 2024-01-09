use rocket_db_pools::Connection;

use crate::models::db::{Db, Result};
use crate::models::res::ServicesConnectionsRes;

pub async fn get_services_db(mut db: Connection<Db>) -> Result<Vec<ServicesConnectionsRes>> {
    let res: Vec<ServicesConnectionsRes> = sqlx::query_as!(ServicesConnectionsRes,
        r"select sc.id, so.name as source, st.name as target, tc.name as type_name, sc.created_at
        from catalog_teo.services_connections sc
            inner join catalog_teo.services so on so.id = sc.source_service_id
            inner join catalog_teo.services st on st.id = sc.target_service_id
            inner join catalog_teo.type_connection tc on tc.id = sc.type_connection_id;"
    )
    .fetch_all(&mut **db)
    .await?;

    Ok(res)
}
