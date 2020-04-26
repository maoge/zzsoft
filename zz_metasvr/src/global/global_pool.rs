
pub struct GlobalPool {
    pg_pool: deadpool_postgres::Pool,    // deadpool::managed::Pool<ClientWrapper, tokio_postgres::Error>
    redis_pool: deadpool_redis::Pool,    // deadpool::managed::Pool<ConnectionWrapper, RedisError>
}

impl GlobalPool {

    pub fn get() -> Self {
        let g_zzconf_holder = GlobalZZConf::get_zz_conf();
        let g_zzconf = g_zzconf_holder.lock().unwrap();
    
        let redis_conf = &g_zzconf.redis_conf;
        let redis_url = format!("redis://:{}@{}:{}/{}",
            redis_conf.passwd, redis_conf.host, redis_conf.port, 0);
        let redis_pool_conf = deadpool_redis::Config {
            url: Some(redis_url),
            pool: Some(deadpool::managed::PoolConfig {
                max_size: redis_conf.max_size as usize,
                timeouts: Timeouts {
                    wait: Some(Duration::from_secs(redis_conf.idle_timeout)),
                    create: Some(Duration::from_secs(redis_conf.connection_timeout)),
                    recycle: Some(Duration::from_secs(redis_conf.max_lifetime)),
                },
            }),
        };
        let redis_pool = redis_pool_conf.create_pool().unwrap();

        let pg_conf = &g_zzconf.db_pg_conf;
        let mut pg_pool_cfg = tokio_postgres::Config::new();
        pg_pool_cfg.host(&(pg_conf.host[..]));
        pg_pool_cfg.port(pg_conf.port as u16);
        pg_pool_cfg.user(&(pg_conf.user[..]));
        pg_pool_cfg.password(&(pg_conf.password[..]));
        pg_pool_cfg.dbname(&(pg_conf.dbname[..]));
        pg_pool_cfg.connect_timeout(Duration::from_secs(pg_conf.connection_timeout));
        pg_pool_cfg.keepalives(true);
        pg_pool_cfg.keepalives_idle(Duration::from_secs(pg_conf.idle_timeout));
        let pg_mgr = deadpool_postgres::Manager::new(pg_pool_cfg, tokio_postgres::NoTls);
        let pg_pool = deadpool_postgres::Pool::new(pg_mgr, pg_conf.max_size as usize);
    
        GlobalPool {
            pg_pool: pg_pool,
            redis_pool: redis_pool,
        }

    }
}

impl Clone for GlobalPool {
    fn clone(&self) -> Self {
        GlobalPool {
            pg_pool: self.pg_pool.clone(),
            redis_pool: self.redis_pool.clone(),
        }
    }
}

pub struct GlobalSession {
    session_map: Rc<RefCell<HashMap<String, Session>>>, // acc_name -> Session
}

impl GlobalSession {
    pub fn get() -> Arc<Mutex<Self>> {
        static mut INSTANCE: Option<Arc<Mutex<GlobalSession>>> = None;
        unsafe {
            INSTANCE.get_or_insert_with(|| {
                println!("init GlobalSession");

                let session_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::with_capacity(16)));
                let global_session = GlobalSession {
                    session_map: session_map,
                };

                Arc::new(Mutex::new(global_session))
            })
            .clone()
        }        
    }
    
    pub fn add_session(&self, session: Session) {
        let mut map: RefMut<HashMap<String, Session>> = self.session_map.borrow_mut();
        map.insert(session.acc_name.clone(), session);
    }

    pub fn refresh_login_time(&self, acc_name: String, ts: i64) {
        let mut map: RefMut<HashMap<String, Session>> = self.session_map.borrow_mut();
        match map.get(&acc_name) {
            Some(v) => {
                let session = Session {
                    acc_name: v.acc_name.clone(),
                    magic_key: v.magic_key.clone(),
                    login_time: ts,
                };
                map.insert(acc_name, session);
            },
            None => {},
        }
    }

    pub fn get_magic_key(&self, acc_name: String) -> String {
        let mut s: String = "".to_string();
        let map: RefMut<HashMap<String, Session>> = self.session_map.borrow_mut();
        match map.get(&acc_name) {
            Some(v) => { s = v.magic_key.clone(); },
            None => { },
        }
        s
    }
}

impl Clone for GlobalSession {
    fn clone(&self) -> Self {
        GlobalSession {
            session_map: self.session_map.clone(),
        }
    }
}
