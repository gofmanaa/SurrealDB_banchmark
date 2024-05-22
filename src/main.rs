use std::time::Instant;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

use time::OffsetDateTime;


#[derive(Debug, Serialize, Deserialize)]
struct Name {
    first: String,
    last: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    title: String,
    name: Name,
    marketing: bool,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[derive(Serialize)]
struct UpdatedAt {
    #[serde(with="time::serde::iso8601")]
    updated_at: OffsetDateTime,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Connect to the server
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;

    // Number of records for benchmarking
    let num_records = 500_000;

    // Insert benchmark
    let start_time = Instant::now();
    for i in 0..num_records {
        let _: Option<Record> = db
            .create(("person", format!("id{}", i)))
            .content(Person {
                title: "Employee".to_string(),
                name: Name {
                    first: format!("First {}", i),
                    last: format!("Last {}", i),
                },
                marketing: i % 2 == 0,
            })
            .await?;
    }
    let duration = start_time.elapsed();
    println!("Insert performance: {:?}", duration);

    // Read benchmark
    let start_time = Instant::now();
    let _: Vec<Record> = db.select("person").await?;
    let duration = start_time.elapsed();
    println!("Read performance: {:?}", duration);

    // Update benchmark
    let start_time = Instant::now();
    for i in 0..num_records {
        let _: Option<Record> = db
            .update(("person",  format!("id{}", i)))
            .merge(UpdatedAt {
	            updated_at: OffsetDateTime::now_utc(),
	        }).await?;
    }
    let duration = start_time.elapsed();
    println!("Update performance: {:?}", duration);

    // Delete benchmark
    let start_time = Instant::now();
    for i in 0..num_records -1 {
        let _person: Option<Person> = db.delete(("person", format!("id{}", i))).await?;
    }
    let duration = start_time.elapsed();
    println!("Delete performance: {:?}", duration);

    Ok(())
}
