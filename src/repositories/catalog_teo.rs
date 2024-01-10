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

pub async fn create_services_db(mut db: Connection<Db>, source_name: String, target_name: String, type_name: String) -> std::result::Result<bool, bool> {
    let res = sqlx::query(r"
    with source as
            (insert into catalog_teo.services (name) VALUES ($1) ON CONFLICT (name) DO NOTHING returning id),
         target as
            (insert into catalog_teo.services (name) values ($2) ON CONFLICT (name) DO NOTHING returning id),
         type as
            (insert into catalog_teo.type_connection (name) values ($3) ON CONFLICT (name) DO NOTHING returning id)
    insert
    into catalog_teo.services_connections (source_service_id, target_service_id, type_connection_id)
    select source_sel.id, target_sel.id, type_sel.id
    from (SELECT source.id
          from source
          where exists (select id from source)
          union all
          select id
          from catalog_teo.services
          where name = $1) as source_sel
             join (SELECT target.id, $2 as name
                   from target
                   where exists (select id as id from target)
                   union all
                   select id, name
                   from catalog_teo.services
                   where name = $2) as target_sel on target_sel.name = $2
             join (SELECT type.id, $3 as name
                   from type
                   where exists (select id from type)
                   union all
                   select id, name
                   from catalog_teo.type_connection
                   where name = $3) as type_sel on type_sel.name = $3")
        .bind(&source_name)
        .bind(&target_name)
        .bind(&type_name)
        .execute(&mut **db)
        .await;

    match res {
        Ok(_) => Ok(true),
        Err(err) => {
            println!("${}", err);
            Err(false)
        }
    }
}
