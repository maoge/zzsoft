// use async_trait::async_trait;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    host_id: String,
    event_type: u32,
    channel: String,
    event_body: String,
}

// #[async_trait]
// pub trait EventBridge {

//     async fn pub_message(&self, event: &Event);
    
//     // async fn register_handler(&self, channels: &[String]);

//     // async fn stop(&self);

// }

pub struct RedisBridge {
    redis_pool: deadpool_redis::Pool,
}

// #[async_trait]
// impl EventBridge for RedisBridge {
impl RedisBridge {

    async fn pub_message(&self, event: &Event) {
        let mut redis_conn = self.redis_pool.get().await.unwrap();
        self.do_pub(&redis_conn, event);
    }

    async fn do_pub(&self, redis_conn: &mut ConnectionWrapper, event: &Event) {
        deadpool_redis::cmd("PUBLISH")
            .arg(event.channel.clone())
            .arg(serde_json::to_string(event).unwrap())
            .execute_async(redis_conn)
            .await
            .unwrap();
    }

    // async fn register_handler(&self, topics: &[String]) {

    // }

    // async fn stop() {

    // }

}
