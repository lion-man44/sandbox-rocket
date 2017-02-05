extern crate serde;
extern crate serde_json;

use self::serde::Serialize;
use self::serde::Serializer;

#[derive(Queryable, Serialize, FromForm, Debug)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String
}

//impl Serialize for User {
//    fn serialize<S: Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
//        (&self.id, &self.name).serialize(s)
//    }
//}
