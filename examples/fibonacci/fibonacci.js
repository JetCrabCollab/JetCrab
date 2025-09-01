// Fibonacci Sequence Examples
// Multiple implementations to demonstrate different approaches

console.log("=== FIBONACCI SEQUENCE EXAMPLES ===\n");

// Recursive Fibonacci
function fibonacciRecursive(n) {
    if (n <= 1) return n;
    return fibonacciRecursive(n - 1) + fibonacciRecursive(n - 2);
}

// Iterative Fibonacci
function fibonacciIterative(n) {
    if (n <= 1) return n;
    
    let a = 0, b = 1;
    for (let i = 2; i <= n; i++) {
        let temp = a + b;
        a = b;
        b = temp;
    }
    return b;
}

// Fibonacci with memoization
function fibonacciMemo(n, memo = {}) {
    if (n in memo) return memo[n];
    if (n <= 1) return n;
    
    memo[n] = fibonacciMemo(n - 1, memo) + fibonacciMemo(n - 2, memo);
    return memo[n];
}

// Generate Fibonacci sequence up to n
function generateFibonacciSequence(n) {
    let sequence = [];
    for (let i = 0; i <= n; i++) {
        sequence.push(fibonacciIterative(i));
    }
    return sequence;
}

// Test different implementations
console.log("1. Recursive Fibonacci:");
console.log("fibonacciRecursive(10):", fibonacciRecursive(10));

console.log("\n2. Iterative Fibonacci:");
console.log("fibonacciIterative(10):", fibonacciIterative(10));

console.log("\n3. Memoized Fibonacci:");
console.log("fibonacciMemo(10):", fibonacciMemo(10));

console.log("\n4. Fibonacci Sequence:");
console.log("Sequence up to 10:", generateFibonacciSequence(10));

console.log("\n5. Performance Comparison:");
console.log("Large number (35) with memoization:", fibonacciMemo(35));
console.log("Large number (35) with iteration:", fibonacciIterative(35));

// Fibonacci with error handling
function fibonacciSafe(n) {
    if (typeof n !== 'number' || n < 0 || !Number.isInteger(n)) {
        throw new Error("Input must be a non-negative integer");
    }
    
    if (n > 1000) {
        throw new Error("Input too large, may cause stack overflow");
    }
    
    return fibonacciIterative(n);
}

console.log("\n6. Safe Fibonacci:");
try {
    console.log("fibonacciSafe(20):", fibonacciSafe(20));
    console.log("fibonacciSafe(-5):", fibonacciSafe(-5));
} catch (error) {
    console.log("Error caught:", error.message);
}
