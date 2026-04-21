---
id: fi-2459-typestate-safety.rs
category: f-01-secbrain
---

/// 🏗️ Example: Typestate Pattern
/// Ensures logical correctness at compile-time.
pub struct Order<State> {
    id: u32,
    _state: std::marker::PhantomData<State>,
}

pub struct Unpaid;
pub struct Paid;

impl Order<Unpaid> {
    pub fn pay(self) -> Order<Paid> {
        Order { id: self.id, _state: std::marker::PhantomData }
    }
}

impl Order<Paid> {
    pub fn ship(self) { /* Only callable when paid */ }
}
