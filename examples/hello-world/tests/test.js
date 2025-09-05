/**
 * Test suite for Hello World example
 * 
 * This demonstrates how to write tests for JetCrab applications
 */

console.log("Running Hello World tests...");

// Test basic JavaScript functionality
function testBasicFeatures() {
    console.log("Testing basic JavaScript features...");

    // Test variables
    const message = "Hello, JetCrab!";
    assert(message === "Hello, JetCrab!", "String assignment");

    // Test functions
    function add(a, b) {
        return a + b;
    }
    assert(add(2, 3) === 5, "Function addition");

    // Test objects
    const obj = { name: "JetCrab", version: "0.4.0" };
    assert(obj.name === "JetCrab", "Object property access");

    // Test arrays
    const arr = [1, 2, 3, 4, 5];
    assert(arr.length === 5, "Array length");
    assert(arr[0] === 1, "Array element access");

    console.log("✅ Basic features tests passed");
}

// Test process API
function testProcessAPI() {
    console.log("Testing Process API...");

    // Test process.version
    assert(typeof process.version === "string", "process.version is string");
    assert(process.version.length > 0, "process.version is not empty");

    // Test process.argv
    assert(Array.isArray(process.argv), "process.argv is array");
    assert(process.argv.length > 0, "process.argv is not empty");

    // Test process.cwd
    assert(typeof process.cwd() === "string", "process.cwd returns string");

    // Test process.env
    assert(typeof process.env === "object", "process.env is object");

    console.log("✅ Process API tests passed");
}

// Test console API
function testConsoleAPI() {
    console.log("Testing Console API...");

    // Test console.log (should not throw)
    try {
        console.log("Test log message");
        console.error("Test error message");
        console.warn("Test warning message");
        console.info("Test info message");
    } catch (error) {
        assert(false, "Console API should not throw errors");
    }

    console.log("✅ Console API tests passed");
}

// Test async functionality
async function testAsyncFeatures() {
    console.log("Testing async features...");

    // Test Promise
    const promise = new Promise((resolve) => {
        setTimeout(() => resolve("async result"), 10);
    });

    const result = await promise;
    assert(result === "async result", "Promise resolution");

    // Test async/await
    async function asyncFunction() {
        return "async function result";
    }

    const asyncResult = await asyncFunction();
    assert(asyncResult === "async function result", "Async function");

    console.log("✅ Async features tests passed");
}

// Test fetch API (mock)
function testFetchAPI() {
    console.log("Testing Fetch API...");

    // Test that fetch function exists
    assert(typeof fetch === "function", "fetch is a function");

    // Test that fetch returns a Promise
    const fetchPromise = fetch("https://example.com");
    assert(fetchPromise instanceof Promise, "fetch returns a Promise");

    console.log("✅ Fetch API tests passed");
}

// Utility function for assertions
function assert(condition, message) {
    if (!condition) {
        throw new Error(`Assertion failed: ${message}`);
    }
}

// Run all tests
async function runTests() {
    try {
        testBasicFeatures();
        testProcessAPI();
        testConsoleAPI();
        await testAsyncFeatures();
        testFetchAPI();

        console.log("");
        console.log("🎉 All tests passed successfully!");
        console.log("Hello World example is working correctly.");

    } catch (error) {
        console.error("❌ Test failed:", error.message);
        process.exit(1);
    }
}

// Run tests if this file is executed directly
if (process.argv[1] && process.argv[1].includes("test.js")) {
    runTests();
}

