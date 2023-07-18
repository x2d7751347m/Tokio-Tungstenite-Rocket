use std::time::SystemTime;

use dto::dto::{ReqSignUp, UserPatch, EmailPost, EmailPatch};
use ::entity::{post, post::Entity as Post, user, user::Entity as User, email, prelude::Email};
use bcrypt::{hash, verify, DEFAULT_COST};
use sea_orm::{*, prelude::DateTimeUtc};

use crate::Query;

pub struct Mutation;

impl Mutation {
    pub async fn create_post(
        db: &DbConn,
        form_data: post::Model,
    ) -> Result<post::ActiveModel, DbErr> {
        post::ActiveModel {
            title: Set(form_data.title.to_owned()),
            text: Set(form_data.text.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_post_by_id(
        db: &DbConn,
        id: i64,
        form_data: post::Model,
    ) -> Result<post::Model, DbErr> {
        let post: post::ActiveModel = Post::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        post::ActiveModel {
            id: post.id,
            title: Set(form_data.title.to_owned()),
            text: Set(form_data.text.to_owned()),
        }
        .update(db)
        .await
    }

    pub async fn delete_post(db: &DbConn, id: i64) -> Result<DeleteResult, DbErr> {
        let post: post::ActiveModel = Post::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        post.delete(db).await
    }

    pub async fn delete_all_posts(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Post::delete_many().exec(db).await
    }

    pub async fn create_user(
        db: &DbConn,
        form_data: ReqSignUp,
    ) -> Result<user::ActiveModel, DbErr> {
        user::ActiveModel {
        username: Set(form_data.username.to_owned()),
        password: Set(hash(&form_data.password, DEFAULT_COST).unwrap()),
        nickname: Set(form_data.nickname.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn create_email(
        db: &DbConn,
        form_data: EmailPost,
        user_id: i64,
    ) -> Result<email::ActiveModel, DbErr> {
        // if Query::find_email_by_email(db, form_data.email.clone()).await.is_ok(){
        //     return Err(DbErr::RecordNotInserted);
        // }
        email::ActiveModel {
            email: Set(form_data.email.to_owned()),
            user_id: Set(user_id),
            ..Default::default()
        }
        .save(db)
        .await
    }
    
    pub async fn update_user_by_id(
        db: &DbConn,
        id: i64,
        form_data: UserPatch,
    ) -> Result<user::Model, DbErr> {
        // if Query::find_user_by_username(db, form_data.username.clone().unwrap()).await.is_ok(){
        //     return Err(DbErr::RecordNotUpdated);
        // }
        let mut user: user::ActiveModel = User::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find user.".to_owned()))
            .map(Into::into)?;

        user.username = match form_data.username {
            Some(b) => Set(b.to_owned()),
            None => NotSet
        };
        match form_data.nickname {
            Some(b) => 
            {
                user.nickname = Set(b.to_owned())
            },
            None => {},
        };
        user.password = match form_data.password {
            Some(b) => Set(hash(&b, DEFAULT_COST).unwrap()),
            None => NotSet,
        };
        user.updated_at = Set(DateTimeUtc::from(SystemTime::now()));
        user.update(db).await
    }
    
    pub async fn update_email_by_id(
        db: &DbConn,
        id: i64,
        form_data: EmailPatch,
    ) -> Result<email::Model, DbErr> {
        // if Query::find_email_by_email(db, form_data.email.clone().unwrap()).await.is_ok(){
        //     return Err(DbErr::RecordNotUpdated);
        // }
        let mut email: email::ActiveModel = Email::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find email.".to_owned()))
            .map(Into::into)?;

        email.email = match form_data.email {
            Some(b) => Set(b.to_owned()),
            None => NotSet
        };
        email.user_id = match form_data.user_id {
            Some(b) => Set(b.to_owned()),
            None => NotSet
        };
        email.updated_at = Set(DateTimeUtc::from(SystemTime::now()));
        email.update(db).await
    }

    pub async fn delete_user(db: &DbConn, id: i64) -> Result<DeleteResult, DbErr> {
        let user: user::ActiveModel = User::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find user.".to_owned()))
            .map(Into::into)?;

        user.delete(db).await
    }

    pub async fn delete_all_users(db: &DbConn) -> Result<DeleteResult, DbErr> {
        User::delete_many().exec(db).await
    }

    pub async fn delete_email(db: &DbConn, id: i64) -> Result<DeleteResult, DbErr> {
        let email: email::ActiveModel = Email::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find email.".to_owned()))
            .map(Into::into)?;

        email.delete(db).await
    }

    pub async fn delete_all_emails(db: &DbConn, user_id: i64) -> Result<DeleteResult, DbErr> {
        Email::delete_many()
        .filter(email::Column::UserId.eq(user_id))
        .exec(db).await
    }
}
