# SKILL 018: BLOCKCHAIN & CRYPTO

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        BLOCKCHAIN & CRYPTO
                     Decentralized Systems
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## EXECUTIVE SUMMARY

Blockchain and cryptocurrency programming in Rust including smart contracts,
consensus algorithms, and Web3 integration.

---

## BLOCKCHAIN BASICS

### 1.1 Block Structure

```rust
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub data: Vec<u8>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, data: Vec<u8>, previous_hash: String) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        }
    }

    pub fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.index.to_string());
        hasher.update(self.timestamp.to_string());
        hasher.update(&self.data);
        hasher.update(&self.previous_hash);
        hasher.update(self.nonce.to_string());
        
        format!("{:x}", hasher.finalize())
    }
}
```

---

## CRYPTOGRAPHIC HASHES

### 2.1 Hash Functions

```rust
use sha2::{Sha256, Sha512, Digest};
use blake3::Hasher as Blake3;

pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    result.into()
}

pub fn blake3(data: &[u8]) -> [u8; 32] {
    let mut hasher = Blake3::new();
    hasher.update(data);
    let result = hasher.finalize();
    result.into()
}
```

---

## SMART CONTRACTS

### 3.1 Simple Contract

```rust
pub struct Contract {
    pub address: String,
    pub code: Vec<u8>,
    pub storage: HashMap<String, Vec<u8>>,
}

impl Contract {
    pub fn execute(&mut self, method: &str, params: Vec<Vec<u8>>) -> Result<Vec<u8>, ContractError> {
        match method {
            "set" => self.set_value(params),
            "get" => self.get_value(params),
            _ => Err(ContractError::MethodNotFound),
        }
    }

    fn set_value(&mut self, params: Vec<Vec<u8>>) -> Result<Vec<u8>, ContractError> {
        if params.len() < 2 {
            return Err(ContractError::InvalidParams);
        }
        
        let key = String::from_utf8(params[0].clone())
            .map_err(|_| ContractError::InvalidParams)?;
        self.storage.insert(key, params[1].clone());
        
        Ok(vec![])
    }

    fn get_value(&mut self, params: Vec<Vec<u8>>) -> Result<Vec<u8>, ContractError> {
        if params.is_empty() {
            return Err(ContractError::InvalidParams);
        }
        
        let key = String::from_utf8(params[0].clone())
            .map_err(|_| ContractError::InvalidParams)?;
        
        Ok(self.storage.get(&key).cloned().unwrap_or_default())
    }
}
```

---

## CONSENSUS

### 4.1 PoW (Proof of Work)

```rust
pub struct ProofOfWork {
    pub difficulty: usize,
}

impl ProofOfWork {
    pub fn validate(block: &Block) -> bool {
        let target = "0".repeat(block.hash.len() / ProofOfWork);
        block.hash.starts_with(&target)
    }
}
```

---

## RECAP

1. **Hash functions** - Cryptographic basics
2. **Mining** - PoW consensus
3. **Smart contracts** - Programmable blockchain

---

*Skill ID: 018 | Category: Blockchain | Complexity: Expert*