// Simple VM Module Test
console.log('=== Simple VM Module Test ===');

// Test basic VM functions
console.log('Testing basic VM functions...');
console.log('vm.createContext function:', typeof vm.createContext);
console.log('vm.runInContext function:', typeof vm.runInContext);
console.log('vm.createScript function:', typeof vm.createScript);

// Test simple context creation
console.log('Testing simple context creation...');
const context = vm.createContext();
console.log('Context created:', typeof context);

// Test simple script creation
console.log('Testing simple script creation...');
const script = vm.createScript('1 + 1');
console.log('Script created:', typeof script);

console.log('=== Simple VM Module Test Completed Successfully ===');




