// Simple Readline Module Test
console.log('=== Simple Readline Module Test ===');

// Test creating readline interface
console.log('Creating readline interface...');
const rl = readline.createInterface({
    prompt: 'test> '
});

console.log('Interface created:', typeof rl);
console.log('Interface prompt:', rl.getPrompt());
console.log('Interface paused:', rl.isPaused());
console.log('Interface closed:', rl.isClosed());

// Test basic methods
rl.setPrompt('new-prompt> ');
console.log('New prompt:', rl.getPrompt());

// Test history
rl.addHistory('command1');
console.log('History length:', rl.getHistory().length);

// Test close
rl.close();
console.log('Interface closed after close():', rl.isClosed());

console.log('=== Simple Readline Module Test Completed Successfully ===');


