#[allow(dead_code)]
fn test_handler_regist(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
    .service(no_params)
    .service(redis_set)
    .service(redis_get)
    .service(pg_insert)
    .service(pg_select)
    .service(get_session);
}

#[get("/test/{msg}/{id}/hello")]
async fn index(info: web::Path<(String,u32)>) -> impl Responder {
    format!("Hello, {}!\r\n", info.0)
}

#[get("/test/")]
async fn no_params() -> &'static str {
    "Hello world!\r\n"
}

#[get("/test/redis_set/{key}/{value}")]
async fn redis_set(info: web::Path<(String, String)>,
    global_pool: web::Data<GlobalPool>,) -> impl Responder {

    let mut _ret_info: String = String::from("");

    let redis_pool = &global_pool.redis_pool;
    let mut conn = redis_pool.get().await.unwrap();
    let reply = deadpool_redis::cmd("SET")
                .arg(info.0.clone())
                .arg(info.1.clone())
                .execute_async(&mut conn)
                .await;
    match reply {
        Err(e) => {
            _ret_info = format!("redis cmd caught error: {:?}\n", e);
            error!("redis cmd caught error: {:?}", e);
        },
        Ok(_) => {
            _ret_info = format!("redis cmd execute {:?}\n", reply);
        },
    }

    _ret_info
}

#[get("/test/redis_get/{key}")]
async fn redis_get(info: web::Path<String>,
    global_pool: web::Data<GlobalPool>,) -> impl Responder {
    
    let mut _ret_info: String = String::from("");
    
    let redis_pool = &global_pool.redis_pool;
    let mut conn = redis_pool.get().await.unwrap();
    let reply = deadpool_redis::cmd("GET")
                .arg(info.clone())
                .execute_async(&mut conn)
                .await;
    match reply {
        Err(e) => {
            _ret_info = format!("redis cmd caught error:{:?}\n", e);
            error!("redis cmd caught error: {:?}", e);
        },
        Ok(_) => {
            _ret_info = format!("redis cmd execute {:?}\n", reply);
        },
    }
    
    
    _ret_info
}

#[get("/test/pg_insert/{balance}")]
async fn pg_insert(info: web::Path<(i64,)>, 
    global_pool: web::Data<GlobalPool>,) -> impl Responder {
    
    let mut _ret_info: String = String::from("");
    
    let pg_pool = &global_pool.pg_pool;
    let mut _pg_client = pg_pool.get().await.unwrap();

    let stmt: tokio_postgres::Statement = _pg_client
            .prepare("INSERT INTO accounts(id, balance) VALUES($1, $2)").await.unwrap();

    let uuid = uuid::Uuid::new_v4().to_string();
    let balance = info.0;

    let reply = _pg_client.query(&stmt, &[&uuid, &balance]).await;
    match reply {
        Err(e) => {
            _ret_info = format!("pg insert caught error: {:?}\n", e);
            error!("pg insert caught error: {:?}", e);
        },
        Ok(s) => {
            _ret_info = format!("pg insert execute {:?}\n", s);
        },
    }
    
    _ret_info
}

#[get("/test/pg_select/{id}")]
async fn pg_select(info: web::Path<(String,)>,
    global_pool: web::Data<GlobalPool>,) -> impl Responder {

    let mut _ret_info: String = String::from("");

    let pg_pool = &global_pool.pg_pool;
    let mut _pg_client = pg_pool.get().await.unwrap();

    let stmt: tokio_postgres::Statement = _pg_client
            .prepare("SELECT id, balance FROM accounts WHERE id = $1").await.unwrap();

    let id = info.0.clone();
    let reply = _pg_client.query(&stmt, &[&id]).await;
    match reply {
        Err(e) => {
            _ret_info = format!("pg select caught error: {:?}", e);
            error!("pg select caught error: {:?}", e);
        },
        Ok(_rows) => {
            _ret_info = format!("pg select execute ok");
            // for row in _rows {
            //     let uid: &str = row.get("id");                 // row.get(0)
            //     let balance: i64 = row.get("balance");         // row.get(1)
            //     println!("id:{}, balance:{}", uid, balance);
            // };
        },
    }
    
    _ret_info
}

#[get("/test/get_session/{acc_name}")]
async fn get_session(info: web::Path<(String,)>,) -> RetResponse {
    let acc_name = info.0.clone();
    let magic_key = GlobalSession::get().lock().unwrap().get_magic_key(acc_name);
    RetResponse {
        ret_code: CODE_OK,
        ret_info: "".to_string(),
        magic_key: magic_key,
    }
}

// #[get("/test/eventbus_pub")]
// async fn event_publish() -> RetResponse {
//     let session = Session {
//         acc_name: "user1".to_string(),
//         magic_key: "".to_string(),
//         login_time: Local::now().timestamp_millis(),
//     };
//     // GlobalSession::get().lock().unwrap().add_session(&session);

//     let event = Event {
//         host_id: GlobalSession::get().lock().unwrap().get_id(),
//         event_type: EV_001,
//         channel: SYS_EVENT.to_string(),
//         event_body: serde_json::to_string(&session).unwrap(),
//     };

//     // println!("{:?}", event);
//     EventBus::get().lock().unwrap().publish(&event);
    
//     RetResponse {
//         ret_code: CODE_OK,
//         ret_info: "".to_string(),
//         magic_key: "".to_string(),
//     }
// }
