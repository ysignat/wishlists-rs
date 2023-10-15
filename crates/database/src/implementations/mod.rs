use async_trait::async_trait;

pub mod item_pictures;
pub mod items;
pub mod subscriptions;
pub mod user_avatars;
pub mod users;
pub mod wishlists;

#[async_trait]
trait KVTrait<K, V, E> {
    async fn get(&self, key: K) -> Result<V, E>;
    async fn put(&self, key: K, value: V) -> Result<(), E>;
    async fn delete(&self, key: K) -> Result<(), E>;
}
