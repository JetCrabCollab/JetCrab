// REPL Example - Interactive JavaScript Shell
// This example demonstrates how to use the REPL module for interactive JavaScript execution

console.log('🔄 Starting REPL Example...');

// Test REPL Session Creation
console.log('📝 Creating REPL Session...');
const replSession = repl.createSession({
    prompt: 'jetcrab> ',
    multilinePrompt: '  ... ',
    historyFile: '.jetcrab_history',
    maxHistorySize: 1000,
    autoCompletion: true,
    syntaxHighlighting: true,
    bracketMatching: true,
    editMode: 'emacs',
    tabCompletion: true,
    caseSensitive: false
});

console.log('✅ REPL Session created with ID:', replSession.id);

// Test REPL Start
console.log('🚀 Starting REPL Session...');
replSession.start().then(() => {
    console.log('✅ REPL Session started successfully');

    // Test command execution
    console.log('⚡ Testing command execution...');
    return replSession.execute('console.log("Hello from REPL!");');
}).then((result) => {
    console.log('📤 Command result:', result);

    // Test variable setting
    console.log('📝 Setting variables...');
    return replSession.execute('let x = 42; let y = "Hello"; let z = {name: "JetCrab"};');
}).then((result) => {
    console.log('📤 Variable setting result:', result);

    // Test complex expressions
    console.log('🧮 Testing complex expressions...');
    return replSession.execute('x + 10;');
}).then((result) => {
    console.log('📤 Expression result:', result);

    // Test function definition
    console.log('🔧 Testing function definition...');
    return replSession.execute('function greet(name) { return "Hello, " + name + "!"; }');
}).then((result) => {
    console.log('📤 Function definition result:', result);

    // Test function call
    console.log('📞 Testing function call...');
    return replSession.execute('greet("JetCrab");');
}).then((result) => {
    console.log('📤 Function call result:', result);

    // Stop the session
    console.log('🛑 Stopping REPL Session...');
    return replSession.stop();
}).then(() => {
    console.log('✅ REPL Session stopped successfully');
}).catch((error) => {
    console.error('❌ REPL Session error:', error);
});

// Test REPL Start with Options
console.log('🔄 Testing REPL start with options...');
const replWithOptions = repl.start({
    prompt: 'custom> ',
    multilinePrompt: '    ',
    historyFile: 'custom_history',
    maxHistorySize: 500,
    autoCompletion: false,
    syntaxHighlighting: false,
    bracketMatching: false,
    editMode: 'vi',
    tabCompletion: false,
    caseSensitive: true
});

console.log('✅ REPL with custom options created with ID:', replWithOptions.id);

// Test REPL utilities
console.log('🛠️ Testing REPL utilities...');

// Test result formatting
console.log('📊 Testing result formatting...');
const testResults = [
    undefined,
    null,
    'Hello, World!',
    42,
    { name: 'JetCrab', version: '0.4.0' },
    [1, 2, 3, 4, 5],
    true,
    false
];

testResults.forEach((result, index) => {
    const formatted = repl.utils.formatResult(result);
    console.log(`📤 Result ${index + 1}:`, formatted);
});

// Test expression completeness checking
console.log('🔍 Testing expression completeness...');
const testExpressions = [
    'console.log("Hello");',
    'if (true) {',
    'function test() {',
    'let arr = [1, 2, 3];',
    'obj.method(',
    'x + y',
    'const obj = {',
    'for (let i = 0; i < 10; i++) {'
];

testExpressions.forEach((expr, index) => {
    const isComplete = repl.utils.isCompleteExpression(expr);
    console.log(`📝 Expression ${index + 1}: "${expr}" - Complete: ${isComplete}`);
});

// Test REPL configuration
console.log('⚙️ REPL Configuration:');
console.log('📋 Prompt:', repl.config.prompt);
console.log('📋 Multiline Prompt:', repl.config.multilinePrompt);
console.log('📋 History File:', repl.config.historyFile);
console.log('📋 Max History Size:', repl.config.maxHistorySize);
console.log('📋 Auto Completion:', repl.config.autoCompletion);
console.log('📋 Syntax Highlighting:', repl.config.syntaxHighlighting);
console.log('📋 Bracket Matching:', repl.config.bracketMatching);
console.log('📋 Edit Mode:', repl.config.editMode);
console.log('📋 Tab Completion:', repl.config.tabCompletion);
console.log('📋 Case Sensitive:', repl.config.caseSensitive);

// Test REPL session management
console.log('📊 Testing REPL session management...');

// Get all sessions
repl.getSessions().then((sessions) => {
    console.log('📋 All sessions:', sessions);

    // Get specific session
    return repl.getSession(replSession.id);
}).then((session) => {
    console.log('📋 Session details:', session);

    // Clear all history
    return repl.clearAllHistory();
}).then(() => {
    console.log('🗑️ All history cleared');

    // Test global variables
    console.log('🌍 Testing global variables...');
    return repl.setGlobalVariable('globalVar', 'globalValue');
}).then(() => {
    return repl.getGlobalVariable('globalVar');
}).then((value) => {
    console.log('📊 Global variable value:', value);

    return repl.listGlobalVariables();
}).then((variables) => {
    console.log('📊 All global variables:', variables);

    console.log('✅ REPL example completed');
}).catch((error) => {
    console.error('❌ REPL example error:', error);
});

// Simulate interactive REPL usage
console.log('🎮 Simulating interactive REPL usage...');

// Simulate reading lines
setTimeout(() => {
    console.log('📖 Simulating line reading...');

    // Simulate reading a line
    replWithOptions.readLine('custom> ').then((line) => {
        console.log('📥 Read line:', line);

        // Simulate executing the line
        return replWithOptions.execute(line);
    }).then((result) => {
        console.log('📤 Execution result:', result);

        // Simulate reading another line
        return replWithOptions.readLine('custom> ');
    }).then((line) => {
        console.log('📥 Read another line:', line);

        // Simulate clearing history
        return replWithOptions.clearHistory();
    }).then(() => {
        console.log('🗑️ History cleared');

        // Stop the REPL
        return replWithOptions.stop();
    }).then(() => {
        console.log('🛑 REPL stopped');
    }).catch((error) => {
        console.error('❌ Interactive REPL error:', error);
    });
}, 1000);

console.log('✅ REPL example setup completed');



