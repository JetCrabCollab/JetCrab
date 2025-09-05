// VM Module Test
console.log('=== VM Module Test ===');

// Test creating context
console.log('Testing context creation...');
const context = vm.createContext({ x: 1, y: 2 });
console.log('Context created:', typeof context);
console.log('Context has x:', context.has('x'));
console.log('Context get x:', context.get('x'));

// Test context operations
context.set('z', 3);
console.log('Context set z:', context.get('z'));
console.log('Context keys:', context.keys());

// Test running code in context
console.log('Testing runInContext...');
const result1 = vm.runInContext('x + y + z', context);
console.log('Result:', result1);

// Test running code in new context
console.log('Testing runInNewContext...');
const result2 = vm.runInNewContext('a + b', { a: 5, b: 10 });
console.log('Result:', result2);

// Test running code in this context
console.log('Testing runInThisContext...');
const result3 = vm.runInThisContext('2 + 3');
console.log('Result:', result3);

// Test script creation
console.log('Testing script creation...');
const script = vm.createScript('x * 2', { filename: 'test.js' });
console.log('Script created:', typeof script);
console.log('Script code:', script.getCode());

// Test script execution
const result4 = script.runInContext(context);
console.log('Script result:', result4);

// Test context validation
console.log('Testing isContext...');
console.log('Is context:', vm.isContext(context));
console.log('Is not context:', vm.isContext({}));

// Test memory measurement
console.log('Testing measureMemory...');
const memory = vm.measureMemory();
console.log('Memory usage:', memory);

// Test cached data
console.log('Testing cached data...');
const cachedData = vm.getCachedData();
console.log('Cached data:', cachedData);

console.log('=== VM Module Test Completed Successfully ===');




