use crate::domain::value_objects::id::ID;
use diesel::prelude::Queryable;

#[derive(Queryable)]
pub struct Admin {
    #[diesel(serialize_as = Option<i32>, deserialize_as = i32)]
    pub id: ID,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}
