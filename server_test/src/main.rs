use std::error::Error;
use actix_web::web::{Data, Json};
use actix_web::{get, App, HttpResponse, HttpServer, post};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[derive(Serialize)]
struct Employee {
    id: i32,
    name: String,
}

#[derive(Deserialize)]
struct EmployeeData {
    name: String
}

#[post("/create_employee")]
pub async fn create_employee(employee_data: Json<EmployeeData>, conn: Data<Pool<Postgres>>) -> Result<HttpResponse, Box<dyn Error>> {
    let employee_data = employee_data.into_inner();

    sqlx::query!(
        r#"
        INSERT INTO employee (name)
        VALUES ($1)
        RETURNING id, name
        "#,
        employee_data.name
    )
      .fetch_one(&**conn)
      .await?;

    Ok(HttpResponse::Ok().json("employee added successfully"))
}

#[get("/get_employees")]
pub async fn get_employees(conn: Data<Pool<Postgres>>) -> Result<HttpResponse, Box<dyn Error>> {
    let employees = sqlx::query_as!(
        Employee,
        r#"
        SELECT id, name
        FROM employee
        "#)
      .fetch_all(&**conn)
      .await?;

    Ok(HttpResponse::Ok().json(employees))
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db_pool = create_db_pool().await?;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db_pool.clone()))
            .service(create_employee)
            .service(get_employees)
    })
    .bind("0.0.0.0:8888")?
    .run()
    .await?;

    Ok(())
}

pub async fn create_db_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let pool = Pool::<Postgres>::connect(&db_url).await?;

    Ok(pool)
}
