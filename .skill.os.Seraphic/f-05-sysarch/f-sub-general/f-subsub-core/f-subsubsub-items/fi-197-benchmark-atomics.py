---
id: fi-197-benchmark-atomics.py
category: f-05-sysarch
---

import subprocess
import os
import sys

# ⚡ RS-011: ATOMIC FABRIC BENCHMARK
# Compiles the wait_free_matrix.rs example and runs a high-contention stress test.

SKILL_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
EXAMPLE_PATH = os.path.join(SKILL_ROOT, "03-Examples/wait_free_matrix.rs")
OUTPUT_BIN = "/tmp/atomic_bench"

def run_benchmark():
    print(f"🚀 COMPILING RS-011 STRESS TEST...")
    # Wrap in a mini-main if needed, but our example already has a test module.
    # To run as a standalone bench, we need a main function.
    
    with open(EXAMPLE_PATH, "r") as f:
        code = f.read()
    
    # Simple wrapper to run the SPSC test in main
    main_wrapper = """
fn main() {
    println!("--- SERAPHIC ATOMIC STRESS TEST ---");
    let start = std::time::Instant::now();
    
    // Run 1,000,000 iterations of SPSC push/pop
    let buffer = std::sync::Arc::new(SpscBuffer::<u32, 1024>::new());
    let b_clone = buffer.clone();

    let producer = std::thread::spawn(move || {
        for i in 0..1_000_000 {
            while b_clone.push(i).is_none() { std::hint::spin_loop(); }
        }
    });

    for i in 0..1_000_000 {
        loop {
            if let Some(val) = buffer.pop() {
                if val % 100_000 == 0 { println!("  Processed {} iterations...", val); }
                break;
            }
            std::hint::spin_loop();
        }
    }
    producer.join().unwrap();
    
    let duration = start.elapsed();
    println!("✅ SUCCESS: 1,000,000 operations in {:?}", duration);
    println!("   Throughput: {:.2} ops/sec", 1_000_000.0 / duration.as_secs_f64());
}
"""
    
    temp_rs = "/tmp/rs_011_bench.rs"
    with open(temp_rs, "w") as f:
        f.write(code + "\n" + main_wrapper)
        
    try:
        subprocess.run(["rustc", "--test", EXAMPLE_PATH, "-o", OUTPUT_BIN], check=False) # Compile tests
        print("⚡ RUNNING UNIT TESTS...")
        subprocess.run([OUTPUT_BIN], check=True)
        
        print("\n⚡ RUNNING MAX THROUGHPUT BENCHMARK...")
        subprocess.run(["rustc", "-O", temp_rs, "-o", OUTPUT_BIN], check=True)
        subprocess.run([OUTPUT_BIN], check=True)
        
    except Exception as e:
        print(f"❌ BENCHMARK FAILED: {str(e)}")
    finally:
        if os.path.exists(temp_rs): os.remove(temp_rs)

if __name__ == "__main__":
    run_benchmark()
