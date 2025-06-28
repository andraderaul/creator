use clap::Parser;

use creator::app::{execute_config, Config};
use creator::opts::Opts;

fn main() -> anyhow::Result<()> {
    println!("🚀 Creator v2.0 - Dynamic Configuration System");

    let opts = Opts::parse();

    // Try to load configuration with graceful error handling
    let config = match Config::try_from(opts) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("❌ Configuration error: {}", e);
            eprintln!();
            eprintln!("💡 Quick start options:");
            eprintln!(
                "   creator init                    # Initialize with interactive preset selection"
            );
            eprintln!("   creator init -p clean-architecture  # Use clean architecture preset");
            eprintln!("   creator init -p module-based         # Use module-based preset");
            eprintln!(
                "   creator list                    # List available structure (if config exists)"
            );
            eprintln!();
            eprintln!("📖 For more help: creator --help");

            std::process::exit(1);
        }
    };

    // Execute the configuration
    if let Err(e) = execute_config(config) {
        eprintln!("❌ Execution error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}
