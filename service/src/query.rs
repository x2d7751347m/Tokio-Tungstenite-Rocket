use ::entity::{post, post::Entity as Post};
use ::entity::{user, user::Entity as User};
use ::entity::{email, email::Entity as Email};
use sea_orm::*;
use sea_orm::sea_query::Expr;

pub struct Query;

impl Query {
    pub async fn find_post_by_id(db: &DbConn, id: i32) -> Result<Option<post::Model>, DbErr> {
        Post::find_by_id(id).one(db).await
    }
    pub async fn find_user_by_id(db: &DbConn, id: i32) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(id).one(db).await
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
