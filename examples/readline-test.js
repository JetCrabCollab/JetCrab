// Readline Module Test
console.log('=== Readline Module Test ===');

// Test creating readline interface
console.log('Creating readline interface...');
const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
    prompt: 'jetcrab> '
});

console.log('Interface created:', typeof rl);
console.log('Interface prompt:', rl.getPrompt());
console.log('Interface paused:', rl.isPaused());
console.log('Interface closed:', rl.isClosed());

// Test interface methods
console.log('Testing interface methods...');
rl.setPrompt('new-prompt> ');
console.log('New prompt:', rl.getPrompt());

rl.write('Test output');
rl.writeLine('Test line output');

// Test cursor operations
rl.cursorTo(10);
rl.moveCursor(5);
const cursorPos = rl.getCursorPos();
console.log('Cursor position:', cursorPos);

// Test line operations
rl.clearLine();
rl.clearScreenDown();

// Test history operations
rl.addHistory('command1');
rl.addHistory('command2');
rl.addHistory('command3');
console.log('History:', rl.getHistory());

rl.setHistorySize(2);
console.log('History after resize:', rl.getHistory());

rl.clearHistory();
console.log('History after clear:', rl.getHistory());

// Test pause/resume
rl.pause();
console.log('Interface paused after pause():', rl.isPaused());

rl.resume();
console.log('Interface paused after resume():', rl.isPaused());

// Test question method
console.log('Testing question method...');
rl.question('What is your name? ', (answer) => {
    console.log('Answer received:', answer);
});

// Test event handling
rl.on('line', (line) => {
    console.log('Line event:', line);
});

rl.on('close', () => {
    console.log('Close event received');
});

// Test static methods
console.log('Testing static methods...');
readline.clearLine(process.stdout, 1);
readline.clearScreenDown(process.stdout);
readline.cursorTo(process.stdout, 0, 0);
readline.moveCursor(process.stdout, 5, 0);

// Test emitKeypressEvents
readline.emitKeypressEvents(process.stdin, rl);

// Test close
rl.close();
console.log('Interface closed after close():', rl.isClosed());

console.log('=== Readline Module Test Completed Successfully ===');


