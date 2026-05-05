#![allow(dead_code)]

use axum::{routing::{get, post}, Json, Router, extract::Query};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use bcrypt::{hash, verify, DEFAULT_COST};
use std::net::SocketAddr;

// --- MODELOS ---

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub username: String,
    pub password: Option<String>,
    pub provider: Option<String>, // 'google', 'github', etc
    pub provider_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Circuito {
    pub id: Option<i32>,
    pub usuario: String,
    pub nome: String,
    pub dados_json: String,
}

#[derive(Deserialize)]
struct AuthCallback { code: String }

// --- MAIN ---

#[tokio::main]
async fn main() {
    init_db().expect("Erro SQLite");

    let app = Router::new()
        .route("/", get(|| async { "API v1.6 - Social Auth Ready" }))
        .route("/api/register", post(registrar_usuario))
        .route("/api/login", post(login_usuario))
        // Rotas para onde o usuário volta após o login no site externo
        .route("/api/auth/google/callback", get(callback_google))
        .route("/api/auth/github/callback", get(callback_github))
        .route("/api/save", post(salvar_circuito))
        .route("/api/load", post(listar_circuitos))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("\x1b[92m[SERVER]\x1b[0m Online em http://localhost:3000");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn init_db() -> rusqlite::Result<()> {
    let conn = Connection::open("quantsimul.db")?;
    conn.execute("CREATE TABLE IF NOT EXISTS usuarios (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        username TEXT UNIQUE NOT NULL,
        password_hash TEXT,
        provider TEXT DEFAULT 'local',
        provider_id TEXT UNIQUE
    )", [])?;
    conn.execute("CREATE TABLE IF NOT EXISTS circuitos (
        id INTEGER PRIMARY KEY AUTOINCREMENT, usuario TEXT NOT NULL, 
        nome TEXT NOT NULL, dados_json TEXT NOT NULL
    )", [])?;
    Ok(())
}

// --- LOGICA DE AUTH SOCIAL ---

async fn callback_google(Query(query): Query<AuthCallback>) -> Json<String> {
    // Aqui no futuro você trocaria o 'code' por um Token do Google
    // Por enquanto, simulamos o sucesso e redirecionamos para o banco
    println!("Google Auth Code recebido: {}", query.code);
    Json("Google Login Sucesso - Usuário autenticado".into())
}

async fn callback_github(Query(query): Query<AuthCallback>) -> Json<String> {
    println!("GitHub Auth Code recebido: {}", query.code);
    Json("GitHub Login Sucesso".into())
}

// --- HANDLERS EXISTENTES ---

async fn registrar_usuario(Json(payload): Json<User>) -> Json<String> {
    let conn = Connection::open("quantsimul.db").unwrap();
    let hashed = hash(payload.password.unwrap_or_default(), DEFAULT_COST).unwrap();
    match conn.execute("INSERT INTO usuarios (username, password_hash, provider) VALUES (?1, ?2, 'local')", params![payload.username, hashed]) {
        Ok(_) => Json("Conta criada!".into()),
        Err(_) => Json("Usuário já existe.".into()),
    }
}

async fn login_usuario(Json(payload): Json<User>) -> Json<String> {
    let conn = Connection::open("quantsimul.db").unwrap();
    let mut stmt = conn.prepare("SELECT password_hash FROM usuarios WHERE username = ?1").unwrap();
    let result = stmt.query_row([payload.username], |row| row.get::<_, String>(0));
    match result {
        Ok(h) => if verify(payload.password.unwrap_or_default(), &h).unwrap_or(false) { Json("Login OK".into()) } else { Json("Senha inválida".into()) },
        Err(_) => Json("Não encontrado".into()),
    }
}

async fn salvar_circuito(Json(p): Json<Circuito>) -> Json<String> {
    let conn = Connection::open("quantsimul.db").unwrap();
    conn.execute("INSERT INTO circuitos (usuario, nome, dados_json) VALUES (?1, ?2, ?3)", params![p.usuario, p.nome, p.dados_json]).unwrap();
    Json("Salvo!".into())
}

async fn listar_circuitos(Json(p): Json<User>) -> Json<Vec<Circuito>> {
    let conn = Connection::open("quantsimul.db").unwrap();
    let mut stmt = conn.prepare("SELECT id, nome, dados_json FROM circuitos WHERE usuario = ?1").unwrap();
    let rows = stmt.query_map([p.username], |row| {
        Ok(Circuito { id: Some(row.get(0)?), usuario: "".into(), nome: row.get(1)?, dados_json: row.get(2)? })
    }).unwrap();
    Json(rows.map(|r| r.unwrap()).collect())
}
