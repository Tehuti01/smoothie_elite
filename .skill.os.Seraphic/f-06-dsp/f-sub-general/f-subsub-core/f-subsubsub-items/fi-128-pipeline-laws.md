---
id: fi-128-pipeline-laws.md
category: f-06-dsp
---

# 📜 PIPELINE LAWS (PRACTICES)

To achieve L0, following laws are mandated for all `process()` functions:

### 1. The Lookahead Prohibition
Unless a user specifically requests a "Lookahead" mode, any delay or FFT window must be handled with sample-accurate feedback. 
- **Requirement:** 0.0ms Reported Latency to the Host.

### 2. Block-Size Alignment
Ensure all internal processing is agnostic to the host's block size. 
- **Constraint:** Handle individual samples or fixed-size 64-sample sub-blocks ONLY.

### 3. Register Pinning & Cache Warming
Use thread-local storage for frequently accessed states to pin them to the processor's register file. 
- **Guideline:** Pre-touch the state before the first sample of the block to warm the L1 cache.

### 4. Wait-Free IPC
All communication between the UI and the Audio thread must happen via **SPSC (Single-Producer Single-Consumer) Ring Buffers**. 
- **Constraint:** No locks. No mutexes. No shared state beyond atomic pointers.

---
*L0 Pipeline Protocol: ENFORCED.*
