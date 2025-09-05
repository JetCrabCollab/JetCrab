// Simple Performance Hooks Module Test
console.log('=== Simple Performance Hooks Module Test ===');

// Test basic performance functions
console.log('Testing basic performance functions...');
console.log('performance.now function:', typeof performance.now);
console.log('performance.mark function:', typeof performance.mark);
console.log('performance.measure function:', typeof performance.measure);

// Test performance.now()
const currentTime = performance.now();
console.log('Current time:', currentTime);

// Test performance.mark()
performance.mark('simple-mark');
console.log('Simple mark created');

// Test performance.getEntries()
const entries = performance.getEntries();
console.log('Entries count:', entries.length);

// Test performance.timing
console.log('Navigation start:', performance.timing.navigationStart);

// Test performance.memory
console.log('Used JS heap size:', performance.memory.usedJSHeapSize);

console.log('=== Simple Performance Hooks Module Test Completed Successfully ===');




