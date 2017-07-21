use utils::schema::users;

#[derive(Clone,Debug)]
#[derive(Queryable)]
#[has_many(list,reply)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}