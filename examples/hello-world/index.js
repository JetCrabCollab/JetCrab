#!/usr/bin/env jetcrab

/**
 * Hello World - JetCrab Example
 * 
 * This example demonstrates basic JetCrab runtime features:
 * - Console API usage
 * - Process API access
 * - Environment variables
 * - Command line arguments
 */

console.log("🦀 Welcome to JetCrab Runtime!");
console.log("=====================================");

// Display runtime information
console.log("Runtime Information:");
console.log(`  Version: ${process.version}`);
console.log(`  Platform: ${process.platform || 'unknown'}`);
console.log(`  Current Directory: ${process.cwd()}`);
console.log("");

// Display command line arguments
console.log("Command Line Arguments:");
console.log(`  Script: ${process.argv[0]}`);
console.log(`  Arguments: ${process.argv.slice(1).join(' ')}`);
console.log("");

// Display environment information
console.log("Environment:");
console.log(`  NODE_ENV: ${process.env.NODE_ENV || 'development'}`);
console.log(`  USER: ${process.env.USER || process.env.USERNAME || 'unknown'}`);
console.log("");

// Demonstrate basic JavaScript features
console.log("JavaScript Features Demo:");
console.log("=========================");

// Variables and functions
const message = "Hello from JetCrab!";
const timestamp = new Date().toISOString();

function greet(name = "World") {
    return `Hello, ${name}! 🦀`;
}

// Object and array operations
const data = {
    message: message,
    timestamp: timestamp,
    features: ["JavaScript", "Rust", "WebAssembly", "Async/Await"]
};

console.log(greet("JetCrab Developer"));
console.log(`Message: ${data.message}`);
console.log(`Timestamp: ${data.timestamp}`);
console.log(`Features: ${data.features.join(", ")}`);
console.log("");

// Async operations simulation
console.log("Async Operations Demo:");
console.log("======================");

async function simulateAsyncWork() {
    console.log("Starting async work...");
    
    // Simulate async operation with Promise
    await new Promise(resolve => {
        setTimeout(() => {
            console.log("Async work completed!");
            resolve();
        }, 100);
    });
    
    console.log("All async operations finished!");
}

// Run async demo
simulateAsyncWork().then(() => {
    console.log("");
    console.log("🎉 Hello World example completed successfully!");
    console.log("Try running with different arguments:");
    console.log("  jetcrab run index.js --arg 'Custom Message'");
}).catch(error => {
    console.error("Error in async demo:", error);
});

// Handle command line arguments
if (process.argv.length > 2) {
    const customArg = process.argv[2];
    console.log(`\nCustom argument received: ${customArg}`);
}

