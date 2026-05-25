use clap::{Arg, Command};
use colored::*;
use rand::Rng;
use std::ptr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Represents a video codec context (simulated)
#[repr(C)]
struct CodecContext {
    buffer_ptr: *mut u8,
    frame_counter: u32,
    state: u32,
    magic: u32, // Magic number to detect corruption
}

impl CodecContext {
    fn new() -> Self {
        let buffer = vec![0u8; 1024];
        let buffer_ptr = Box::into_raw(buffer.into_boxed_slice()) as *mut u8;
        
        CodecContext {
            buffer_ptr,
            frame_counter: 0,
            state: 0x12345678,
            magic: 0xDEADBEEF,
        }
    }

    unsafe fn process_frame(&mut self) -> bool {
        // Simulate frame processing
        if self.magic != 0xDEADBEEF {
            println!("{} UAF detected! Magic number corrupted: 0x{:08X}", "🚨".bright_red(), self.magic);
            return false;
        }
        
        self.frame_counter += 1;
        self.state = rand::thread_rng().gen();
        true
    }

    unsafe fn release(&mut self) {
        if !self.buffer_ptr.is_null() {
            let _ = Box::from_raw(std::slice::from_raw_parts_mut(self.buffer_ptr, 1024));
            self.buffer_ptr = ptr::null_mut();
        }
        // Corrupt magic to simulate freed memory
        self.magic = 0xFEEDFACE;
    }
}

fn main() {
    let matches = Command::new("CVE-2025-21082 UAF PoC")
        .version("1.0")
        .author("Security Researcher")
        .about("Demonstrates HyperOS AVCodec Use-After-Free vulnerability")
        .arg(
            Arg::new("mode")
                .short('m')
                .long("mode")
                .value_name("MODE")
                .help("Execution mode: vulnerable or patched")
                .required(true)
                .value_parser(["vulnerable", "patched"]),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose memory address logging")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let mode = matches.get_one::<String>("mode").unwrap();
    let verbose = matches.get_flag("verbose");

    println!("{}", "🔬 CVE-2025-21082: HyperOS AVCodec UAF PoC".bright_cyan());
    println!("Mode: {}", mode.bright_yellow());
    println!("Verbose: {}\n", verbose);

    match mode.as_str() {
        "vulnerable" => vulnerable_scenario(verbose),
        "patched" => patched_scenario(verbose),
        _ => unreachable!(),
    }
}

fn vulnerable_scenario(verbose: bool) {
    println!("{} Running vulnerable scenario...", "⚠️".bright_yellow());
    
    unsafe {
        // Allocate codec context on heap
        let codec_ptr = Box::into_raw(Box::new(CodecContext::new()));
        
        if verbose {
            println!("CodecContext allocated at: {:p}", codec_ptr);
        }

        // Shared pointer for worker thread
        let shared_ptr = Arc::new(Mutex::new(codec_ptr));
        let worker_ptr = Arc::clone(&shared_ptr);

        println!("{} Starting worker thread...", "🧵".bright_blue());
        
        // Start worker thread that will access codec context
        let worker_handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(100)); // Simulate async work
            
            let ptr_guard = worker_ptr.lock().unwrap();
            let codec_ptr = *ptr_guard;
            
            if !codec_ptr.is_null() {
                println!("{} Worker thread accessing codec context...", "🔄".bright_green());
                
                // This is the UAF - accessing freed memory
                let mut codec = &mut *codec_ptr;
                
                // Try to process frame on potentially freed memory
                for i in 0..5 {
                    if !codec.process_frame() {
                        println!("{} UAF vulnerability triggered on frame {}!", "💥".bright_red(), i);
                        break;
                    }
                    thread::sleep(Duration::from_millis(50));
                }
            }
        });

        // Simulate race condition - release codec while worker is using it
        thread::sleep(Duration::from_millis(50));
        
        println!("{} Main thread releasing codec context (UAF trigger)...", "🗑️".bright_red());
        
        {
            let ptr_guard = shared_ptr.lock().unwrap();
            let codec_ptr = *ptr_guard;
            if !codec_ptr.is_null() {
                (*codec_ptr).release();
                
                // Simulate heap reclamation with different data
                let corrupted_data = Box::new([0xAA, 0xBB, 0xCC, 0xDD]);
                let corrupted_ptr = Box::into_raw(corrupted_data) as *mut CodecContext;
                
                if verbose {
                    println!("Memory reclaimed with corrupted data at: {:p}", corrupted_ptr);
                }
                
                // Free the original context
                let _ = Box::from_raw(codec_ptr);
            }
        }

        // Wait for worker thread to complete
        worker_handle.join().unwrap();
        
        println!("\n{} Vulnerable scenario completed - UAF demonstrated!", "🚨".bright_red());
        println!("{} In a real exploit, this could lead to RCE", "⚠️".bright_yellow());
    }
}

fn patched_scenario(verbose: bool) {
    println!("{} Running patched scenario...", "✅".bright_green());
    
    unsafe {
        // Allocate codec context on heap
        let codec_ptr = Box::into_raw(Box::new(CodecContext::new()));
        
        if verbose {
            println!("CodecContext allocated at: {:p}", codec_ptr);
        }

        // Shared pointer for worker thread
        let shared_ptr = Arc::new(Mutex::new(codec_ptr));
        let worker_ptr = Arc::clone(&shared_ptr);

        println!("{} Starting worker thread...", "🧵".bright_blue());
        
        // Start worker thread
        let worker_handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            
            let ptr_guard = worker_ptr.lock().unwrap();
            let codec_ptr = *ptr_guard;
            
            if !codec_ptr.is_null() {
                println!("{} Worker thread processing frames safely...", "🔄".bright_green());
                
                let mut codec = &mut *codec_ptr;
                
                // Process frames safely
                for i in 0..5 {
                    if !codec.process_frame() {
                        println!("{} Unexpected error on frame {}", "❌".bright_red(), i);
                        break;
                    }
                    println!("{} Frame {} processed successfully", "✅".bright_green(), i);
                    thread::sleep(Duration::from_millis(50));
                }
                
                println!("{} Worker thread completed safely", "✅".bright_green());
            }
        });

        // PATCH: Wait for worker thread to complete before releasing
        println!("{} Waiting for worker thread to complete (patch applied)...", "⏳".bright_blue());
        worker_handle.join().unwrap();
        
        // Now safely release the codec context
        println!("{} Safely releasing codec context...", "🗑️".bright_green());
        {
            let ptr_guard = shared_ptr.lock().unwrap();
            let codec_ptr = *ptr_guard;
            if !codec_ptr.is_null() {
                (*codec_ptr).release();
                let _ = Box::from_raw(codec_ptr);
                
                if verbose {
                    println!("CodecContext safely freed");
                }
            }
        }
        
        println!("\n{} Patched scenario completed - No UAF occurred!", "✅".bright_green());
        println!("{} Proper synchronization prevents the vulnerability", "🛡️".bright_blue());
    }
}