use clap::Parser;

use creator::app::{execute_config, Config};
use creator::opts::Opts;

fn main() -> anyhow::Result<()> {
    println!("🚀 Creator v1.0 - Dynamic Configuration System");

    let opts = Opts::parse();

    // Try to load configuration with graceful error handling
    let config = match Config::try_from(opts) {
        Ok(config) => config,
        Err(e) => {
            let error_msg = e.to_string();

            // Check if it's the "no command specified" error (CLI-first behavior)
            if error_msg.contains("No command specified") {
                // Show user-friendly help for CLI-first design
                eprintln!("🚀 Creator v1.0 - Dynamic Configuration System");
                eprintln!();
                eprintln!("Creator requires explicit commands for automation-friendly operation.");
                eprintln!();
                eprintln!("💡 Quick start:");
                eprintln!(
                    "   creator interactive                       # Run guided interactive mode"
                );
                eprintln!("   creator init                              # Initialize project configuration");
                eprintln!("   creator create cats/components/cat-list   # Create item directly");
                eprintln!("   creator list                              # List available modules");
                eprintln!();
                eprintln!("📖 For detailed help: creator --help");
                std::process::exit(0); // Exit 0 for help, not error
            } else {
                // Other configuration errors
                eprintln!("❌ Configuration error: {}", e);
                eprintln!();
                eprintln!("💡 Troubleshooting:");
                eprintln!("   creator init                              # Initialize with interactive setup");
                eprintln!(
                    "   creator init -p clean-architecture        # Use clean architecture preset"
                );
                eprintln!("   creator init -p module-based              # Use module-based preset");
                eprintln!();
                eprintln!("📖 For more help: creator --help");
                std::process::exit(1);
            }
        }
    };

    // Execute the configuration
    if let Err(e) = execute_config(config) {
        eprintln!("❌ Execution error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}
