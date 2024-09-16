use std::ops::Deref;

#[derive(sqlx::FromRow, Debug)]
#[sqlx(transparent)]
pub struct ID(pub u64);

/// Enable `Deref` coercion `ID`.
impl Deref for ID {
    type Target = u64;
    fn deref(&self) -> &Self::Target { &self.0 }
}
