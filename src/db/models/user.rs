use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
}
