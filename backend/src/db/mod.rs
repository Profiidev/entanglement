use centaurus::db::init::Connection;

use crate::db::invalid_jwt::InvalidJwtTable;
use crate::db::key::KeyTable;
use crate::db::user::UserTable;

pub mod invalid_jwt;
pub mod key;
pub mod user;

pub trait DBTrait {
  fn invalid_jwt(&self) -> InvalidJwtTable<'_>;
  fn key(&self) -> KeyTable<'_>;
  fn user(&self) -> UserTable<'_>;
}

impl DBTrait for Connection {
  fn invalid_jwt(&self) -> InvalidJwtTable<'_> {
    InvalidJwtTable::new(self)
  }

  fn key(&self) -> KeyTable<'_> {
    KeyTable::new(self)
  }

  fn user(&self) -> UserTable<'_> {
    UserTable::new(self)
  }
}
