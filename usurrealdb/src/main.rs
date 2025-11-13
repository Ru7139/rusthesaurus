#[cfg(test)]
mod test_of_those;

use surrealdb::{Surreal, engine::any::Any};

static _SDB: std::sync::OnceLock<Surreal<Any>> = std::sync::OnceLock::new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
