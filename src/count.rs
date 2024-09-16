use std::ops::Deref;

#[derive(sqlx::FromRow, Debug)]
#[sqlx(transparent)]
pub struct Count(pub i64);

/// Enable `Deref` coercion `Count`.
impl Deref for Count {
    type Target = i64;
    fn deref(&self) -> &Self::Target { &self.0 }
}
