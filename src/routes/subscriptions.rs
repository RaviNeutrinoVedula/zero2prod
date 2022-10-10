use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;
use tracing::Instrument;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
	subscriber_email = %form.email,
	subscriber_name = %form.name
    )
)]
    
pub async fn subscribe(
    form: web::Form<FormData>,
    // Retrieving a connection from the application state!
    pool: web::Data<PgPool>,
) -> HttpResponse {
    // Using this approach to get around the other way of dynamically creating a DB
    // and tearing it down on every `cargo test`.
    // There should not be a where clause on this delete statement.

    match sqlx::query!(	r#" delete from subscriptions "#)
	.execute(pool.get_ref())
	.await {
	    Ok(_) => {
		println!("Cleared the subscriptions table");
		HttpResponse::Ok().finish()
	    },
	    Err(e) => {
		println!("Failed to clear the subscriptions table: {}", e);
		HttpResponse::InternalServerError().finish()
	    }
	};
// }

// #[tracing::instrument(
//     name = "Saving new subscriber details in the database",
//     skip(form, pool)
// )]

// pub async fn insert_subscriber(
//     pool: &PgPool,
//     form: &FormData,
    // ) -> Result<(), sqlx::Error> {
    
    match sqlx::query!(
	r#"
insert into subscriptions (id, email, name, subscribed_at) 
values ($1, $2, $3, $4)
"#,
	Uuid::new_v4(),
	form.email,
	form.name,
	Utc::now()
    )
	.execute(pool.get_ref())
	.await
	// .map_err(|e| {
	//     tracing::error!("Failed to execute query: {:?}", e);
	//     e
	// })?;
    // Ok(())
          {
	      Ok(_) => {
		  println!("Added a new subscriber");
		  HttpResponse::Ok().finish()
	      },
	    Err(e) => {
		println!("Failed to add a new subscriber: {}", e);
		HttpResponse::InternalServerError().finish()
	    }
          }   
}


