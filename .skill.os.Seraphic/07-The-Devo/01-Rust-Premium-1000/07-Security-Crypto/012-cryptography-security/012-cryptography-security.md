# SKILL 012: RUST SECURITY & CRYPTOGRAPHY

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        SECURITY & CRYPTOGRAPHY IN RUST
                     Secure Systems & Crypto Implementation
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## EXECUTIVE SUMMARY

Comprehensive mastery of security and cryptography in Rust including
symmetric/asymmetric encryption, hashing, digital signatures, TLS,
secure protocols, and vulnerability mitigation.

## TABLE OF CONTENTS

1. [Cryptographic Primitives](#cryptographic-primitives)
2. [TLS/SSL](#tlsssl)
3. [Secure Protocols](#secure-protocols)
4. [Vulnerability Prevention](#vulnerability-prevention)
5. [Key Management](#key-management)

---

## CRYPTOGRAPHIC PRIMITIVES

### 1.1 AES-GCM

```rust
use std::arch::aes::AES;

pub struct AesGcm {
    key: [u8; 32],
}

impl AesGcm {
    pub fn new(key: &[u8; 32]) -> Self {
        AesGcm { key: *key }
    }

    pub fn encrypt(&self, nonce: &[u8; 12], plaintext: &[u8], aad: &[u8]) -> Vec<u8> {
        let mut ciphertext = vec![0u8; plaintext.len()];
        
        // Initialize AES-CTR
        let mut ctr = [0u8; 16];
        ctr[..12].copy_from_slice(nonce);
        
        // Generate keystream
        let mut expanded_key = [0u8; 240];
        KeyExpansion(&self.key, &mut expanded_key);
        
        // Encrypt
        for (i, block) in plaintext.chunks(16).enumerate() {
            let mut keystream = [0u8; 16];
            ctr[12..].copy_from_slice(&(i as u32).to_le_bytes());
            
            let encrypted_ctr = encrypt_block(&expanded_key, &ctr);
            for (j, &byte) in block.iter().enumerate() {
                ciphertext[i * 16 + j] = byte ^ encrypted_ctr[j];
            }
        }
        
        // Add authentication tag
        let tag = compute_gcm_tag(&ciphertext, aad, nonce, &self.key);
        ciphertext.extend_from_slice(&tag);
        
        ciphertext
    }
}
```

---

## TLS/SSL

### 2.1 TLS Handshake

```rust
pub struct TlsConnection {
    socket: TcpStream,
    cipher_suite: CipherSuite,
    state: TlsState,
    peer_cert: Option<Certificate>,
}

impl TlsConnection {
    pub fn connect(host: &str) -> Result<TlsConnection, TlsError> {
        let socket = TcpStream::connect(format!("{}:443", host))?;
        
        // TLS handshake
        let mut conn = TlsConnection {
            socket,
            cipher_suite: CipherSuite::Aes256GcmSha384,
            state: TlsState::ClientHello,
            peer_cert: None,
        };
        
        conn.client_hello()?;
        conn.server_hello()?;
        conn.verify_certificate()?;
        
        conn.state = TlsState::Established;
        
        Ok(conn)
    }
    
    fn client_hello(&mut self) -> Result<(), TlsError> {
        let client_hello = ClientHello {
            version: Version::Tls12,
            random: generate_random(),
            session_id: None,
            cipher_suites: vec![
                CipherSuite::Aes256GcmSha384,
                CipherSuite::Aes128GcmSha256,
            ],
            extensions: vec![
                Extension::Sni("example.com".to_string()),
                Extension::Alpn(vec!["h2".to_string()]),
            ],
        };
        
        self.socket.write_all(&client_hello.serialize())?;
        Ok(())
    }
}
```

---

## VULNERABILITY PREVENTION

### 3.1 Secure Coding

```rust
/// Constant-time comparison
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    
    diff == 0
}

/// Timing-safe authentication
pub fn verify_hmac(data: &[u8], key: &[u8], expected: &[u8]) -> bool {
    let computed = compute_hmac(data, key);
    constant_time_eq(&computed, expected)
}
```

---

## RECAP

1. **Never roll your own crypto** - Use battle-tested libraries
2. **GCM for authenticated encryption** - AEAD always
3. **TLS 1.3 minimum** - Disable older versions
4. **Constant-time for secrets** - Prevent timing attacks
5. **Certificate pinning** - Mitigate MITM

---

*Skill ID: 012 | Category: Security | Complexity: Expert*
*Version: 1.0.0 | Last Updated: 2024*