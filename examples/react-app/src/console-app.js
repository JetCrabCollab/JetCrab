/**
 * Console App - JetCrab Example
 * 
 * This is a console-based application that works with JetCrab runtime.
 * It demonstrates interactive features without requiring a browser environment.
 */

console.log('🦀 Starting JetCrab Console App...');

// Simple state management
class StateManager {
    constructor() {
        this.state = {};
    }
    
    setState(newState) {
        this.state = { ...this.state, ...newState };
    }
    
    getState() {
        return this.state;
    }
}

// Counter functionality
class Counter {
    constructor() {
        this.state = new StateManager();
        this.state.setState({ count: 0, step: 1 });
    }
    
    increment() {
        const current = this.state.getState();
        this.state.setState({ count: current.count + current.step });
        this.display();
    }
    
    decrement() {
        const current = this.state.getState();
        this.state.setState({ count: current.count - current.step });
        this.display();
    }
    
    reset() {
        this.state.setState({ count: 0 });
        this.display();
    }
    
    setStep(step) {
        this.state.setState({ step: parseInt(step) });
        this.display();
    }
    
    display() {
        const { count, step } = this.state.getState();
        const color = count > 0 ? '🟢' : count < 0 ? '🔴' : '🔵';
        
        console.log(`\n${color} Counter: ${count} (step: ${step})`);
        console.log('Commands: + (increment), - (decrement), r (reset), s<number> (set step)');
    }
}

// Todo functionality
class TodoManager {
    constructor() {
        this.state = new StateManager();
        this.state.setState({ 
            todos: [], 
            nextId: 1,
            filter: 'all' 
        });
        this.loadTodos();
    }
    
    loadTodos() {
        // Simulate loading from storage
        const sampleTodos = [
            { id: 1, text: 'Learn JetCrab', completed: false },
            { id: 2, text: 'Build awesome apps', completed: false }
        ];
        this.state.setState({ todos: sampleTodos, nextId: 3 });
    }
    
    addTodo(text) {
        if (text.trim()) {
            const current = this.state.getState();
            const newTodo = {
                id: current.nextId,
                text: text.trim(),
                completed: false
            };
            this.state.setState({ 
                todos: [...current.todos, newTodo],
                nextId: current.nextId + 1
            });
            this.display();
        }
    }
    
    toggleTodo(id) {
        const current = this.state.getState();
        this.state.setState({
            todos: current.todos.map(todo => 
                todo.id === id ? { ...todo, completed: !todo.completed } : todo
            )
        });
        this.display();
    }
    
    deleteTodo(id) {
        const current = this.state.getState();
        this.state.setState({
            todos: current.todos.filter(todo => todo.id !== id)
        });
        this.display();
    }
    
    setFilter(filter) {
        this.state.setState({ filter });
        this.display();
    }
    
    display() {
        const { todos, filter } = this.state.getState();
        
        const filteredTodos = todos.filter(todo => {
            switch (filter) {
                case 'active': return !todo.completed;
                case 'completed': return todo.completed;
                default: return true;
            }
        });
        
        const stats = {
            total: todos.length,
            active: todos.filter(t => !t.completed).length,
            completed: todos.filter(t => t.completed).length
        };
        
        console.log(`\n📝 Todo List (${filter})`);
        console.log(`📊 Stats: ${stats.total} total, ${stats.active} active, ${stats.completed} completed`);
        
        if (filteredTodos.length === 0) {
            console.log('   No todos found for this filter.');
        } else {
            filteredTodos.forEach(todo => {
                const status = todo.completed ? '✅' : '⏳';
                console.log(`   ${status} [${todo.id}] ${todo.text}`);
            });
        }
        
        console.log('\nCommands:');
        console.log('  add <text> - Add new todo');
        console.log('  toggle <id> - Toggle todo completion');
        console.log('  delete <id> - Delete todo');
        console.log('  filter <all|active|completed> - Filter todos');
    }
}

// Calculator functionality
class Calculator {
    constructor() {
        this.state = new StateManager();
        this.state.setState({ 
            result: 0, 
            history: [],
            lastOperation: null 
        });
    }
    
    add(value) {
        const current = this.state.getState();
        const newResult = current.result + value;
        this.state.setState({ 
            result: newResult,
            history: [...current.history.slice(-4), `+ ${value} = ${newResult}`],
            lastOperation: 'add'
        });
        this.display();
    }
    
    subtract(value) {
        const current = this.state.getState();
        const newResult = current.result - value;
        this.state.setState({ 
            result: newResult,
            history: [...current.history.slice(-4), `- ${value} = ${newResult}`],
            lastOperation: 'subtract'
        });
        this.display();
    }
    
    multiply(value) {
        const current = this.state.getState();
        const newResult = current.result * value;
        this.state.setState({ 
            result: newResult,
            history: [...current.history.slice(-4), `* ${value} = ${newResult}`],
            lastOperation: 'multiply'
        });
        this.display();
    }
    
    divide(value) {
        if (value === 0) {
            console.log('❌ Error: Division by zero!');
            return;
        }
        const current = this.state.getState();
        const newResult = current.result / value;
        this.state.setState({ 
            result: newResult,
            history: [...current.history.slice(-4), `/ ${value} = ${newResult}`],
            lastOperation: 'divide'
        });
        this.display();
    }
    
    reset() {
        this.state.setState({ 
            result: 0, 
            history: [],
            lastOperation: null 
        });
        this.display();
    }
    
    display() {
        const { result, history } = this.state.getState();
        
        console.log(`\n🧮 Calculator Result: ${result}`);
        
        if (history.length > 0) {
            console.log('📜 Recent operations:');
            history.slice().reverse().forEach(op => {
                console.log(`   ${op}`);
            });
        }
        
        console.log('\nCommands:');
        console.log('  +<number> - Add');
        console.log('  -<number> - Subtract');
        console.log('  *<number> - Multiply');
        console.log('  /<number> - Divide');
        console.log('  reset - Reset to 0');
    }
}

// Main App
class ConsoleApp {
    constructor() {
        this.state = new StateManager();
        this.state.setState({ 
            currentMode: 'home',
            counter: new Counter(),
            todos: new TodoManager(),
            calculator: new Calculator()
        });
    }
    
    showWelcome() {
        console.log('\n🦀 Welcome to JetCrab Console App!');
        console.log('This is an interactive console application running on JetCrab runtime.');
        console.log('\nAvailable modes:');
        console.log('  counter - Interactive counter');
        console.log('  todos - Todo list manager');
        console.log('  calc - Calculator');
        console.log('  home - Return to main menu');
        console.log('\nType a mode name to switch, or "help" for more info.');
    }
    
    switchMode(mode) {
        this.state.setState({ currentMode: mode });
        
        switch (mode) {
            case 'counter':
                console.log('\n🔢 Counter Mode');
                this.state.getState().counter.display();
                break;
            case 'todos':
                console.log('\n📝 Todo Mode');
                this.state.getState().todos.display();
                break;
            case 'calc':
                console.log('\n🧮 Calculator Mode');
                this.state.getState().calculator.display();
                break;
            case 'home':
                this.showWelcome();
                break;
            default:
                console.log(`❌ Unknown mode: ${mode}`);
                this.showWelcome();
        }
    }
    
    processCommand(input) {
        const parts = input.trim().split(' ');
        const command = parts[0].toLowerCase();
        const args = parts.slice(1);
        
        const currentMode = this.state.getState().currentMode;
        
        switch (currentMode) {
            case 'counter':
                this.processCounterCommand(command, args);
                break;
            case 'todos':
                this.processTodoCommand(command, args);
                break;
            case 'calc':
                this.processCalcCommand(command, args);
                break;
            default:
                if (['counter', 'todos', 'calc', 'home'].includes(command)) {
                    this.switchMode(command);
                } else if (command === 'help') {
                    this.showHelp();
                } else {
                    console.log(`❌ Unknown command: ${command}`);
                    this.showWelcome();
                }
        }
    }
    
    processCounterCommand(command, args) {
        const counter = this.state.getState().counter;
        
        switch (command) {
            case '+':
                counter.increment();
                break;
            case '-':
                counter.decrement();
                break;
            case 'r':
            case 'reset':
                counter.reset();
                break;
            case 's':
                if (args.length > 0) {
                    counter.setStep(args[0]);
                } else {
                    console.log('❌ Please provide a step value: s <number>');
                }
                break;
            default:
                console.log('❌ Unknown counter command. Use +, -, r, or s<number>');
        }
    }
    
    processTodoCommand(command, args) {
        const todos = this.state.getState().todos;
        
        switch (command) {
            case 'add':
                if (args.length > 0) {
                    todos.addTodo(args.join(' '));
                } else {
                    console.log('❌ Please provide todo text: add <text>');
                }
                break;
            case 'toggle':
                if (args.length > 0) {
                    const id = parseInt(args[0]);
                    todos.toggleTodo(id);
                } else {
                    console.log('❌ Please provide todo ID: toggle <id>');
                }
                break;
            case 'delete':
                if (args.length > 0) {
                    const id = parseInt(args[0]);
                    todos.deleteTodo(id);
                } else {
                    console.log('❌ Please provide todo ID: delete <id>');
                }
                break;
            case 'filter':
                if (args.length > 0) {
                    todos.setFilter(args[0]);
                } else {
                    console.log('❌ Please provide filter: filter <all|active|completed>');
                }
                break;
            default:
                console.log('❌ Unknown todo command. Use add, toggle, delete, or filter');
        }
    }
    
    processCalcCommand(command, args) {
        const calc = this.state.getState().calculator;
        
        if (command.startsWith('+')) {
            const value = parseFloat(command.slice(1));
            if (!isNaN(value)) {
                calc.add(value);
            } else {
                console.log('❌ Invalid number for addition');
            }
        } else if (command.startsWith('-')) {
            const value = parseFloat(command.slice(1));
            if (!isNaN(value)) {
                calc.subtract(value);
            } else {
                console.log('❌ Invalid number for subtraction');
            }
        } else if (command.startsWith('*')) {
            const value = parseFloat(command.slice(1));
            if (!isNaN(value)) {
                calc.multiply(value);
            } else {
                console.log('❌ Invalid number for multiplication');
            }
        } else if (command.startsWith('/')) {
            const value = parseFloat(command.slice(1));
            if (!isNaN(value)) {
                calc.divide(value);
            } else {
                console.log('❌ Invalid number for division');
            }
        } else if (command === 'reset') {
            calc.reset();
        } else {
            console.log('❌ Unknown calc command. Use +<number>, -<number>, *<number>, /<number>, or reset');
        }
    }
    
    showHelp() {
        console.log('\n📚 JetCrab Console App Help');
        console.log('\nModes:');
        console.log('  counter - Interactive counter with step control');
        console.log('  todos - Todo list with filtering and management');
        console.log('  calc - Calculator with operation history');
        console.log('  home - Main menu');
        console.log('\nGeneral commands:');
        console.log('  help - Show this help');
        console.log('  <mode> - Switch to mode');
        console.log('\nCounter commands:');
        console.log('  + - Increment');
        console.log('  - - Decrement');
        console.log('  r - Reset');
        console.log('  s <number> - Set step size');
        console.log('\nTodo commands:');
        console.log('  add <text> - Add todo');
        console.log('  toggle <id> - Toggle completion');
        console.log('  delete <id> - Delete todo');
        console.log('  filter <all|active|completed> - Filter todos');
        console.log('\nCalculator commands:');
        console.log('  +<number> - Add');
        console.log('  -<number> - Subtract');
        console.log('  *<number> - Multiply');
        console.log('  /<number> - Divide');
        console.log('  reset - Reset to 0');
    }
}

// Initialize the app
function initializeApp() {
    console.log('Initializing JetCrab Console App...');
    
    const app = new ConsoleApp();
    app.showWelcome();
    
    // Simulate some interactions to demonstrate functionality
    console.log('\n🎯 Demo: Let\'s try some features...');
    
    // Demo counter
    console.log('\n--- Counter Demo ---');
    app.switchMode('counter');
    app.processCommand('+');
    app.processCommand('+');
    app.processCommand('s 5');
    app.processCommand('+');
    
    // Demo todos
    console.log('\n--- Todo Demo ---');
    app.switchMode('todos');
    app.processCommand('add Learn JetCrab runtime');
    app.processCommand('add Build awesome applications');
    app.processCommand('toggle 1');
    app.processCommand('filter completed');
    
    // Demo calculator
    console.log('\n--- Calculator Demo ---');
    app.switchMode('calc');
    app.processCommand('+10');
    app.processCommand('*3');
    app.processCommand('-5');
    
    console.log('\n🎉 Demo completed! The app is ready for interactive use.');
    console.log('In a real interactive environment, you would be able to type commands.');
    
    return app;
}

// Run the app
const app = initializeApp();

console.log('\n✅ JetCrab Console App initialized successfully!');
console.log('This demonstrates:');
console.log('- Component-based architecture');
console.log('- State management');
console.log('- Interactive command processing');
console.log('- Multiple application modes');
console.log('- Modern JavaScript features');
