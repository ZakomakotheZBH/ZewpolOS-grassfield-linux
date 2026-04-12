// this new system is OPTIONAL so you can toggle it on and off
// SPDX-License-Identifier: GPL-2.0
//! A high-performance, optional Rust scripting bridge for the Linux Kernel.

use kernel::prelude::*;
use kernel::sync::Arc;

// --- CONFIGURATION ---
// In a real scenario, this would be tied to a Kconfig variable.
// We use a constant here for demonstration.
const CONFIG_RUST_SCRIPTING: bool = true;

// --- THE INTERFACE ---
/// This trait defines what your "script" can actually do. 
/// It's the sandbox boundary.
#[vtable]
pub trait KernelScript: Send + Sync {
    /// The primary execution hook for the script.
    fn run(&self, input: i32) -> Result<i32>;
    
    /// Returns the name of the script for sysfs/logging.
    fn name(&self) -> &'static str;
}

// --- THE MANAGER ---
/// Manages the lifecycle of the active script.
pub struct ScriptManager;

/// We use a static Option to hold the active script implementation.
/// 'static' is safe here because we manage it via registration.
static mut ACTIVE_SCRIPT: Option<Arc<dyn KernelScript>> = None;

impl ScriptManager {
    /// Register a new script. Can be called by any module.
    pub fn register(script: Arc<dyn KernelScript>) {
        pr_info!("Scripting: Loading '{}'\n", script.name());
        unsafe { ACTIVE_SCRIPT = Some(script) };
    }

    /// Remove the current script.
    pub fn unregister() {
        unsafe { ACTIVE_SCRIPT = None };
    }

    /// The "Hot Path" - Call this from your kernel subsystem.
    /// If scripting is disabled, this compiles down to a simple return.
    #[inline(always)]
    pub fn execute(val: i32) -> i32 {
        if !CONFIG_RUST_SCRIPTING {
            return val; // Zero-cost return if disabled
        }

        unsafe {
            if let Some(ref script) = ACTIVE_SCRIPT {
                // If script fails, we return the original value (fail-safe)
                return script.run(val).unwrap_or(val);
            }
        }
        val
    }
}

// --- EXAMPLE IMPLEMENTATION ---
/// This is an example of a "script" that lives in the same file.
struct MultiplyingScript;

impl KernelScript for MultiplyingScript {
    fn name(&self) -> &'static str { "Multiplier" }

    fn run(&self, input: i32) -> Result<i32> {
        // Logic: if even, double it; if odd, leave it.
        if input % 2 == 0 {
            Ok(input * 2)
        } else {
            Ok(input)
        }
    }
}

// --- MODULE BOILERPLATE ---
/// This allows the scripting system to be loaded as an optional module (.ko)
module! {
    type: ScriptSystemModule,
    name: "rust_script_system",
    author: "Linux Developer",
    description: "Optional Rust Scripting Tool",
    license: "GPL",
}

struct ScriptSystemModule;

impl kernel::Module for ScriptSystemModule {
    fn init(_module: &'static InternalModule) -> Result<Self> {
        pr_info!("Scripting System Initialized\n");
        
        // Automatically load the example script
        let script = Arc::try_new(MultiplyingScript)?;
        ScriptManager::register(script);
        
        Ok(ScriptSystemModule)
    }
}

impl Drop for ScriptSystemModule {
    fn drop(&mut self) {
        ScriptManager::unregister();
        pr_info!("Scripting System Unloaded\n");
    }
}
