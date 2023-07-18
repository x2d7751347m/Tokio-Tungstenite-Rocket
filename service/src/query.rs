use ::entity::{post, post::Entity as Post};
use ::entity::{user, user::Entity as User};
use ::entity::{email, email::Entity as Email};
use sea_orm::*;
use sea_orm::sea_query::Expr;

pub struct Query;

impl Query {
    pub async fn find_post_by_id(db: &DbConn, id: i64) -> Result<Option<post::Model>, DbErr> {
        Post::find_by_id(id).one(db).await
    }
    pub async fn find_user_by_id(db: &DbConn, id: i64) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(id).one(db).await
    }
    pub async fn find_email_by_id(db: &DbConn, id: i64) -> Result<Option<email::Model>, DbErr> {
        Email::find_by_id(id).one(db).await
    }
    
    pub async fn find_user_by_email(db: &DbConn, email: String) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find()
        // construct `RelationDef` on the fly
        .join_rev(
            JoinType::InnerJoin,
            email::Entity::belongs_to(user::Entity)
                .from(email::Column::UserId)
                .to(user::Column::Id)
                .into()
        )
        .filter(email::Column::Email.eq(&email))
        .one(db)
        .await
    }

    pub async fn find_emails_by_user_id(db: &DbConn, user_id: i64) -> Result<Vec<email::Model>, DbErr> {
        email::Entity::find()
        // construct `RelationDef` on the fly
        .filter(email::Column::UserId.eq(user_id))
        .all(db)
        .await
    }
    
    pub async fn find_user_by_username(db: &DbConn, username: String) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find()
        // construct `RelationDef` on the fly
        .filter(user::Column::Username.eq(&username))
        .one(db)
        .await
    }

    /// If ok, returns (user models, num pages).
    pub async fn find_users_in_page(
        db: &DbConn,
        page: u64,
        users_per_page: u64,
    ) -> Result<(Vec<user::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = User::find()
            .order_by_asc(user::Column::Id)
            .paginate(db, users_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated users
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    /// If ok, returns (email models, num pages).
    pub async fn find_emails_in_page(
        db: &DbConn,
        page: u64,
        emails_per_page: u64,
        user_id: i64
    ) -> Result<(Vec<email::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = Email::find()
        .filter(email::Column::UserId.eq(user_id))
            .order_by_asc(email::Column::Id)
            .paginate(db, emails_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated emails
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    /// If ok, returns (post models, num pages).
    pub async fn find_posts_in_page(
        db: &DbConn,
        page: u64,
        posts_per_page: u64,
    ) -> Result<(Vec<post::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = Post::find()
            .order_by_asc(post::Column::Id)
            .paginate(db, posts_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated posts
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
