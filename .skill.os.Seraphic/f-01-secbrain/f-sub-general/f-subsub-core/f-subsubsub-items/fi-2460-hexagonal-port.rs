---
id: fi-2460-hexagonal-port.rs
category: f-01-secbrain
---

/// 🏗️ Example: Hexagonal Port Trait
/// Decouples Domain Logic from Infrastructure.
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: uuid::Uuid) -> Result<User, DbError>;
    async fn save(&self, user: &User) -> Result<(), DbError>;
}
