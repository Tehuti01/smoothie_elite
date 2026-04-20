# SKILL RUST-CLI: RUST CLI APPS

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        RUST COMMAND LINE APPS
                     CLAP, BUILDING CLI TOOLS
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## CLI WITH CLAP

```rust
use clap::{Parser, Arg, Command};

#[derive(Parser)]
#[command(name = "myapp")]
#[command(about = "A great CLI app")]
#[command(long_about = "A longer description")]
pub struct Cli {
    #[command(short, long, default_value = "world")]
    name: String,
    
    #[command(short, long)]
    verbose: bool,
    
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
pub enum Commands {
    /// Serve the app
    Serve(ServeArgs),
    /// Build the app
    Build(BuildArgs),
}

#[derive(Parser)]
pub struct ServeArgs {
    #[command(short, long, default_value = "8080")]
    port: u16,
}

pub fn main() {
    let cli = Cli::parse();
    println!("Hello, {}!", cli.name);
}
```

---

*Skill ID: RUST-CLI | Category: CLI | Complexity: Expert*