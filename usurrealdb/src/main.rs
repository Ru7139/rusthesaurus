#[cfg(test)]
mod test_of_those;

// use futures::StreamExt;
// use std::sync::LazyLock;
// use surrealdb::{Surreal, engine::any::Any, opt::auth::Root};

// static SDB: LazyLock<Surreal<Any>> = LazyLock::new(Surreal::init);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // SDB.connect("http://127.0.0.1:65535").await?;
    // SDB.signin(Root {
    //     username: "ruut",
    //     password: "ruut",
    // })
    // .await?;

    // SDB.use_ns("CN").use_db("SZSE").await?;

    // let db_info: surrealdb::Value = SDB.query("INFO FOR DB").await?.take(0)?;
    // let text = db_info.to_string();

    // let re = regex::Regex::new(r"DEFINE TABLE (\S+)").unwrap();
    // let table_names: Vec<String> = re
    //     .captures_iter(&text)
    //     .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
    //     .collect();

    // futures::stream::iter(table_names)
    //     .map(|table| {
    //         let sdb = SDB.clone();
    //         async move {
    //             let query_msg = format!("DELETE FROM `{}`", table);
    //             sdb.query(query_msg).await
    //         }
    //     })
    //     .buffer_unordered(10)
    //     .collect::<Vec<_>>()
    //     .await;

    Ok(())
}
