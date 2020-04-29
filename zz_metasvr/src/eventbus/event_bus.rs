use async_trait::async_trait;

const SYS_EVENT: &str = "sys.event";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    host_id: String,
    event_type: u8,
    channel: String,
    event_body: String,
}

#[async_trait]
pub trait EventBridge {
    fn publish(&self, event: &Event);
}

pub struct RedisBridge {
    redis_pool: r2d2::Pool<RedisConnectionManager>,
}

type OnEventFn = fn(&Event);

impl RedisBridge {

    fn new() -> Self {
        let g_zzconf_holder = GlobalZZConf::get_zz_conf();
        let g_zzconf = g_zzconf_holder.lock().unwrap();

        let redis_conf = &g_zzconf.redis_conf;
        let redis_url = format!("redis://:{}@{}:{}/{}",
            redis_conf.passwd, redis_conf.host, redis_conf.port, 0);
        let manager = RedisConnectionManager::new(redis_url).unwrap();
        let redis_pool = r2d2::Pool::builder()
                        .max_size(2)
                        .build(manager)
                        .unwrap();

        RedisBridge {
            redis_pool: redis_pool,
        }
    }

}

impl EventBridge for RedisBridge {

    fn publish(&self, event: &Event) {
        let pool = &self.redis_pool;
        let mut conn = pool.get().unwrap();

        let channel: &String = &event.channel;
        let msg: String = serde_json::to_string(event).unwrap();
        let _r: i32 = conn.publish(channel.clone(), msg.clone()).unwrap();
    }

}

pub struct EventBus {
    event_bridge: Rc<RefCell<dyn EventBridge>>,
    handles: Vec<OnEventFn>,
}

impl EventBus {

    pub fn get() -> Arc<Mutex<Self>> {
        static mut INSTANCE: Option<Arc<Mutex<EventBus>>> = None;
        unsafe {
            INSTANCE.get_or_insert_with(|| {
                println!("init EventBus");

                let redis_bridge = RedisBridge::new();
                let event_bridge: Rc<RefCell<_>> = Rc::new(RefCell::new(redis_bridge));
                let mut event_bus = EventBus {
                    event_bridge: event_bridge,
                    handles: Vec::new(),
                };
                &event_bus.register(on_event);

                Arc::new(Mutex::new(event_bus))
            })
            .clone()
        }        
    }

    pub fn publish(&self, event: &Event) {
        let bridge: RefMut<dyn EventBridge> = self.event_bridge.borrow_mut();
        &bridge.publish(event);
    }

    pub fn register(&mut self, lsnr: OnEventFn) {
        self.handles.push(lsnr);
    }

    pub fn handle(&self, event: &Event) {
        for h in &self.handles {
            h(event);
        }
    }

}

impl Clone for EventBus {
    fn clone(&self) -> Self {
        EventBus {
            event_bridge: self.event_bridge.clone(),
            handles: self.handles.clone(),
        }
    }
}

#[allow(dead_code)]
fn start_event_loop() {
    let builder = std::thread::Builder::new().name(String::from("eventbus.thread"));
    let _handle = builder.spawn(move || {

        // pump event from redis and dispatch to handler
        // due to life circle restriction, redis pool in RedisBridge self cannot be used;
        let g_zzconf_holder = GlobalZZConf::get_zz_conf();
        let g_zzconf = g_zzconf_holder.lock().unwrap();

        let redis_conf = &g_zzconf.redis_conf;
        let redis_url = format!("redis://:{}@{}:{}/{}",
            redis_conf.passwd, redis_conf.host, redis_conf.port, 0);
        let manager = RedisConnectionManager::new(redis_url).unwrap();
        let redis_pool = r2d2::Pool::builder()
                            .max_size(2)
                            .build(manager)
                            .unwrap();

        let mut conn = redis_pool.get().unwrap();
        let _r: std::result::Result<(), _> = conn.subscribe(&[SYS_EVENT], |msg| {
            let result: redis::RedisResult<String> = msg.get_pattern();
            match result {
                Err(e) => {
                    error!("eventbus process subscribe msg caught:{:?}", e);

                    ControlFlow::Break(())
                },
                Ok(s) => {
                    let message: String = s.clone();
                    let event: Event = serde_json::from_str(&message).unwrap();
                    EventBus::get().lock().unwrap().handle(&event);
                    
                    ControlFlow::Continue
                },
            }
        });
    });
}

pub const EV_001: u8 = 1;    // 1: new login, notify sync session 

pub fn on_event(event: &Event) {
    // publish message loop back to self, 
    if event.host_id != GlobalSession::get().lock().unwrap().get_id() {
        return;
    }

    println!("on_event:{:?}", &event);

    match event.event_type {
        EV_001 => {
            
        },
        _ => {

        },
    }
}
