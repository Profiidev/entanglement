use std::io::Cursor;

use base64::prelude::*;
use entity::user;
use image::{ImageFormat, imageops::FilterType};
use sea_orm::{IntoActiveModel, prelude::*};

pub struct UserTable<'db> {
  db: &'db DatabaseConnection,
}

impl<'db> UserTable<'db> {
  pub fn new(db: &'db DatabaseConnection) -> Self {
    Self { db }
  }

  pub async fn create_user(
    &self,
    username: String,
    email: String,
  ) -> centaurus::error::Result<Uuid> {
    let url = crate::gravatar::get_gravatar_url(&email);
    let data = match reqwest::get(&url).await {
      Ok(response) => {
        if response.status().is_success() {
          match response.bytes().await {
            Ok(bytes) => {
              let img = image::load_from_memory(&bytes)?;
              let img = img.resize_exact(128, 128, FilterType::Lanczos3);

              let mut buf = Cursor::new(Vec::new());
              img.write_to(&mut buf, ImageFormat::WebP)?;
              let avatar = BASE64_STANDARD.encode(buf.into_inner());
              Some(avatar)
            }
            Err(_) => None,
          }
        } else {
          None
        }
      }
      Err(_) => None,
    };

    let model = user::Model {
      id: Uuid::new_v4(),
      name: username,
      email,
      avatar: data,
    }
    .into_active_model();

    let ret = model.insert(self.db).await?;

    Ok(ret.id)
  }

  pub async fn try_get_user_by_email(&self, email: &str) -> Result<Option<user::Model>, DbErr> {
    user::Entity::find()
      .filter(user::Column::Email.eq(email.to_string()))
      .one(self.db)
      .await
  }

  pub async fn get_user_by_id(&self, id: Uuid) -> Result<user::Model, DbErr> {
    user::Entity::find_by_id(id)
      .one(self.db)
      .await?
      .ok_or(DbErr::RecordNotFound(format!(
        "User with id {} not found",
        id
      )))
  }
}
