use diesel::{deserialize::FromSql, pg::Pg, serialize::ToSql, sql_types::Integer};
use serde::Serialize;

#[derive(Clone, Debug, Serialize, PartialEq)]
pub enum ID {
    New,
    Existing(i32),
}

impl ID {
    pub fn is_existing(&self) -> bool {
        matches!(self, ID::Existing(_))
    }
}

impl From<i32> for ID {
    fn from(value: i32) -> Self {
        Self::Existing(value)
    }
}

impl From<ID> for Option<i32> {
    fn from(value: ID) -> Self {
        match value {
            ID::New => None,
            ID::Existing(id) => Some(id),
        }
    }
}

impl FromSql<Integer, Pg> for ID {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let id = <i32 as FromSql<Integer, Pg>>::from_sql(bytes)?;
        Ok(ID::Existing(id))
    }
}

impl ToSql<Integer, Pg> for ID {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        match self {
            ID::New => Err("Cannot serialize UserId".into()),
            ID::Existing(id) => <i32 as ToSql<Integer, Pg>>::to_sql(id, out),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::domain::value_objects::id::ID;

    #[test]
    fn from_i32() {
        let id: i32 = 42;
        let id: ID = id.into();
        assert_eq!(id, ID::Existing(42));
    }

    #[test]
    fn from_id_into_option_i32() {
        let new_id = ID::New;
        let existing_id = ID::Existing(42);

        let id: Option<i32> = new_id.into();
        assert_eq!(id, None);

        let id: Option<i32> = existing_id.into();
        assert_eq!(id, Some(42));
    }
}
