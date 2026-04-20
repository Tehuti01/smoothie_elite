# SKILL 011-D: MONGODB & NOSQL

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        MONGODB & NOSQL
                     Document Database Access
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## EXECUTIVE SUMMARY

MongoDB implementation in Rust for document storage and NoSQL patterns.

---

## MONGODB

```rust
use mongodb::{Client, Database, Collection, bson::Document};

pub struct MongoDB {
    client: Client,
    db: Database,
}

impl MongoDB {
    pub fn new(uri: &str, db_name: &str) -> Result<Self, mongodb::Error> {
        let client = Client::with_uri_str(uri)?;
        let db = client.database(db_name);
        Ok(MongoDB { client, db })
    }

    pub fn collection<T: Serialize>(&self, name: &str) -> Collection<T> {
        self.db.collection(name)
    }

    pub async fn insert<T: Serialize>(&self, collection: &str, doc: T) -> Result<InsertOneResult, mongodb::Error> {
        self.collection::<Document>(collection)
            .insert_one(doc.into(), None)
            .await
    }

    pub async fn find<T: DeserializeOwned>(&self, collection: &str, filter: Document) -> Result<Vec<T>, mongodb::Error> {
        self.collection::<T>(collection)
            .find(filter, None)
            .await?
            .collect::<Result<Vec<T>, _>>()
            .await
    }
}
```

---

*Skill ID: 011-D | Category: Database | Complexity: Expert*