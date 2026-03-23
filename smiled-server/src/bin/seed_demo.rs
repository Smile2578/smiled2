//! Creates a demo cabinet + user for development/testing.
//! Usage: cargo run --bin seed_demo

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use sqlx::PgPool;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let cabinet_id =
        Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();

    // Upsert demo cabinet
    sqlx::query(
        r#"
        INSERT INTO cabinet (id, nom, adresse, siret, finess)
        VALUES ($1, 'Cabinet Smiled Demo', '123 Rue de la Santé, 75013 Paris', '12345678901234', '750000001')
        ON CONFLICT (id) DO NOTHING
        "#,
    )
    .bind(cabinet_id)
    .execute(&pool)
    .await
    .expect("Failed to create cabinet");

    println!("Cabinet created: {cabinet_id}");

    // Hash password
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(b"Smiled2026!", &salt)
        .expect("Failed to hash password")
        .to_string();

    // Upsert demo user
    let user_id = Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap();

    sqlx::query(
        r#"
        INSERT INTO utilisateur (id, cabinet_id, role, nom, prenom, email, password_hash, actif)
        VALUES ($1, $2, 'titulaire', 'Belissa', 'Simon', 'simon@smiled.io', $3, true)
        ON CONFLICT (email) DO UPDATE SET password_hash = EXCLUDED.password_hash
        "#,
    )
    .bind(user_id)
    .bind(cabinet_id)
    .bind(&password_hash)
    .execute(&pool)
    .await
    .expect("Failed to create user");

    println!("User created:");
    println!("  Email:    simon@smiled.io");
    println!("  Password: Smiled2026!");
    println!("  Role:     titulaire");
    println!("  Cabinet:  Cabinet Smiled Demo");
}
