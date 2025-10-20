mod includes;
use includes::*;

#[derive(Deserialize)]
struct LoginData {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct AuthResult {
	known: bool,
}

#[tokio::main]
async fn main() {
	let max_connections = 10;

	// Make a pool for connections
	let pool = match MySqlPoolOptions::new()
		.max_connections(max_connections)
		.connect(&std::env::var("DATABASE_URL").unwrap())
		.await	
	{
		Ok(val) => {
			println!("Connected to database succesfully");
			val
		},
		Err(err) => panic!("Wasn't able to connect to database, {}", err)
	};
	
	// Configure service
    let service = Router::new()
    	.route("/verify", post(verify_user))
    	.with_state(pool);

	// Setup listener
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    let listener = match TcpListener::bind(addr).await {
    	Ok(val) => {
    		println!("Listener has been set up");
    		val
    	},
    	Err(err) => panic!("Wasn't able to set up listener, {}", err)
    };

	// Start the service
    println!("Rust API running at http://{}", addr);
	axum::serve(listener, service.into_make_service()).await.unwrap();
}

async fn verify_user(State(pool): State<sqlx::MySqlPool>, Json(data): Json<LoginData>) -> Json<AuthResult> {
	let result = sqlx::query!(
		r#"
		SELECT user_id FROM users WHERE username = ? AND password = ?
		"#,
		data.username,
		data.password
	)
	.fetch_one(&pool)
	.await;

	let valid = result.is_ok();

	println!("Got query, returning {}", valid);
	Json(AuthResult { known: valid })
}
