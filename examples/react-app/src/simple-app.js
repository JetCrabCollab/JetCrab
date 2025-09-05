/**
 * Simple React App - JetCrab Example
 * 
 * This is a simplified React application that works with JetCrab runtime.
 * Since JetCrab doesn't support require() yet, we'll create a simple interactive app.
 */

console.log('🦀 Starting Simple React App with JetCrab Runtime...');

// Simple React-like component system
class SimpleComponent {
    constructor(props = {}) {
        this.props = props;
        this.state = {};
    }
    
    setState(newState) {
        this.state = { ...this.state, ...newState };
        this.render();
    }
    
    render() {
        return '';
    }
}

// Simple Counter Component
class Counter extends SimpleComponent {
    constructor(props) {
        super(props);
        this.state = { count: 0, step: 1 };
    }
    
    increment() {
        this.setState({ count: this.state.count + this.state.step });
    }
    
    decrement() {
        this.setState({ count: this.state.count - this.state.step });
    }
    
    reset() {
        this.setState({ count: 0 });
    }
    
    setStep(step) {
        this.setState({ step: parseInt(step) });
    }
    
    render() {
        const color = this.state.count > 0 ? '#28a745' : 
                     this.state.count < 0 ? '#dc3545' : '#667eea';
        
        return `
            <div style="text-align: center; padding: 2rem; background: rgba(255,255,255,0.95); border-radius: 1rem; margin: 2rem;">
                <h2>Interactive Counter</h2>
                <div style="font-size: 3rem; font-weight: bold; color: ${color}; margin: 1rem 0;">
                    ${this.state.count}
                </div>
                <div style="display: flex; gap: 1rem; justify-content: center; margin: 1rem 0;">
                    <button onclick="counter.increment()" style="padding: 0.75rem 1.5rem; background: #667eea; color: white; border: none; border-radius: 0.5rem; cursor: pointer;">
                        +${this.state.step}
                    </button>
                    <button onclick="counter.decrement()" style="padding: 0.75rem 1.5rem; background: #6c757d; color: white; border: none; border-radius: 0.5rem; cursor: pointer;">
                        -${this.state.step}
                    </button>
                    <button onclick="counter.reset()" style="padding: 0.75rem 1.5rem; background: #dc3545; color: white; border: none; border-radius: 0.5rem; cursor: pointer;">
                        Reset
                    </button>
                </div>
                <div style="margin-top: 2rem;">
                    <label style="display: block; margin-bottom: 0.5rem;">Step Size: ${this.state.step}</label>
                    <input type="range" min="1" max="10" value="${this.state.step}" 
                           onchange="counter.setStep(this.value)" 
                           style="width: 100%; max-width: 200px;">
                </div>
            </div>
        `;
    }
}

// Simple Todo Component
class TodoApp extends SimpleComponent {
    constructor(props) {
        super(props);
        this.state = { 
            todos: [], 
            newTodo: '', 
            filter: 'all' 
        };
        this.loadTodos();
    }
    
    loadTodos() {
        try {
            const saved = localStorage.getItem('jetcrab-todos');
            if (saved) {
                this.setState({ todos: JSON.parse(saved) });
            }
        } catch (e) {
            console.error('Error loading todos:', e);
        }
    }
    
    saveTodos() {
        try {
            localStorage.setItem('jetcrab-todos', JSON.stringify(this.state.todos));
        } catch (e) {
            console.error('Error saving todos:', e);
        }
    }
    
    addTodo() {
        if (this.state.newTodo.trim()) {
            const todo = {
                id: Date.now(),
                text: this.state.newTodo.trim(),
                completed: false
            };
            this.setState({ 
                todos: [todo, ...this.state.todos],
                newTodo: ''
            });
            this.saveTodos();
        }
    }
    
    toggleTodo(id) {
        this.setState({
            todos: this.state.todos.map(todo => 
                todo.id === id ? { ...todo, completed: !todo.completed } : todo
            )
        });
        this.saveTodos();
    }
    
    deleteTodo(id) {
        this.setState({
            todos: this.state.todos.filter(todo => todo.id !== id)
        });
        this.saveTodos();
    }
    
    setFilter(filter) {
        this.setState({ filter });
    }
    
    clearCompleted() {
        this.setState({
            todos: this.state.todos.filter(todo => !todo.completed)
        });
        this.saveTodos();
    }
    
    render() {
        const filteredTodos = this.state.todos.filter(todo => {
            switch (this.state.filter) {
                case 'active': return !todo.completed;
                case 'completed': return todo.completed;
                default: return true;
            }
        });
        
        const stats = {
            total: this.state.todos.length,
            active: this.state.todos.filter(t => !t.completed).length,
            completed: this.state.todos.filter(t => t.completed).length
        };
        
        return `
            <div style="background: rgba(255,255,255,0.95); border-radius: 1rem; padding: 2rem; margin: 2rem;">
                <h2>Todo List</h2>
                
                <div style="display: flex; gap: 1rem; margin-bottom: 2rem;">
                    <input type="text" placeholder="Add a new todo..." 
                           value="${this.state.newTodo}"
                           onchange="todoApp.setState({newTodo: this.value})"
                           onkeypress="if(event.key==='Enter') todoApp.addTodo()"
                           style="flex: 1; padding: 0.75rem; border: 2px solid #e9ecef; border-radius: 0.5rem;">
                    <button onclick="todoApp.addTodo()" 
                            style="padding: 0.75rem 1.5rem; background: #667eea; color: white; border: none; border-radius: 0.5rem; cursor: pointer;">
                        Add Todo
                    </button>
                </div>
                
                <div style="display: flex; gap: 0.5rem; margin-bottom: 1rem;">
                    ${['all', 'active', 'completed'].map(filterType => `
                        <button onclick="todoApp.setFilter('${filterType}')" 
                                style="padding: 0.5rem 1rem; background: ${this.state.filter === filterType ? '#667eea' : '#6c757d'}; color: white; border: none; border-radius: 0.25rem; cursor: pointer;">
                            ${filterType} (${filterType === 'all' ? stats.total : stats[filterType]})
                        </button>
                    `).join('')}
                </div>
                
                ${stats.completed > 0 ? `
                    <button onclick="todoApp.clearCompleted()" 
                            style="padding: 0.5rem 1rem; background: #dc3545; color: white; border: none; border-radius: 0.25rem; cursor: pointer; margin-bottom: 1rem;">
                        Clear Completed (${stats.completed})
                    </button>
                ` : ''}
                
                ${filteredTodos.length === 0 ? `
                    <div style="text-align: center; padding: 2rem; color: #666; background: rgba(255,255,255,0.5); border-radius: 0.5rem;">
                        ${this.state.todos.length === 0 ? 
                            '<p>No todos yet. Add one above! 📝</p>' : 
                            `<p>No ${this.state.filter} todos found. 🎉</p>`
                        }
                    </div>
                ` : `
                    <div>
                        ${filteredTodos.map(todo => `
                            <div style="display: flex; align-items: center; gap: 1rem; padding: 1rem; border: 1px solid #e9ecef; border-radius: 0.5rem; margin-bottom: 0.5rem; background: white;">
                                <input type="checkbox" ${todo.completed ? 'checked' : ''} 
                                       onchange="todoApp.toggleTodo(${todo.id})"
                                       style="width: 1.2rem; height: 1.2rem;">
                                <span style="flex: 1; ${todo.completed ? 'text-decoration: line-through; opacity: 0.6;' : ''}">${todo.text}</span>
                                <button onclick="todoApp.deleteTodo(${todo.id})" 
                                        style="background: #dc3545; color: white; border: none; border-radius: 0.25rem; padding: 0.5rem; cursor: pointer;">
                                    Delete
                                </button>
                            </div>
                        `).join('')}
                    </div>
                `}
                
                <div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 1rem; margin-top: 2rem;">
                    <div style="text-align: center; padding: 1rem; background: rgba(255,255,255,0.1); border-radius: 0.5rem;">
                        <div style="font-size: 2rem; font-weight: bold;">${stats.total}</div>
                        <div style="font-size: 0.9rem; opacity: 0.8;">Total</div>
                    </div>
                    <div style="text-align: center; padding: 1rem; background: rgba(255,255,255,0.1); border-radius: 0.5rem;">
                        <div style="font-size: 2rem; font-weight: bold;">${stats.active}</div>
                        <div style="font-size: 0.9rem; opacity: 0.8;">Active</div>
                    </div>
                    <div style="text-align: center; padding: 1rem; background: rgba(255,255,255,0.1); border-radius: 0.5rem;">
                        <div style="font-size: 2rem; font-weight: bold;">${stats.completed}</div>
                        <div style="font-size: 0.9rem; opacity: 0.8;">Completed</div>
                    </div>
                </div>
            </div>
        `;
    }
}

// Main App
class App extends SimpleComponent {
    constructor() {
        super();
        this.state = { currentPage: 'home' };
    }
    
    navigate(page) {
        this.setState({ currentPage: page });
    }
    
    render() {
        const navItems = [
            { id: 'home', label: 'Home', icon: '🏠' },
            { id: 'counter', label: 'Counter', icon: '🔢' },
            { id: 'todos', label: 'Todos', icon: '✅' }
        ];
        
        let content = '';
        switch (this.state.currentPage) {
            case 'counter':
                content = counter.render();
                break;
            case 'todos':
                content = todoApp.render();
                break;
            default:
                content = `
                    <div style="background: rgba(255,255,255,0.95); border-radius: 1rem; padding: 2rem; margin: 2rem;">
                        <h2>Welcome to JetCrab Simple App! 🦀</h2>
                        <p>This is a simplified React-like application running on JetCrab runtime.</p>
                        <p>Use the navigation above to explore different features.</p>
                        
                        <div style="margin-top: 2rem;">
                            <h3>Features Demonstrated:</h3>
                            <ul style="margin-top: 1rem; padding-left: 2rem;">
                                <li>Component-based architecture</li>
                                <li>State management</li>
                                <li>Event handling</li>
                                <li>Local storage integration</li>
                                <li>Responsive design</li>
                                <li>Modern JavaScript features</li>
                            </ul>
                        </div>
                    </div>
                `;
        }
        
        return `
            <div style="min-height: 100vh; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: #333;">
                <header style="background: rgba(255,255,255,0.1); backdrop-filter: blur(10px); padding: 1rem 2rem; border-bottom: 1px solid rgba(255,255,255,0.2);">
                    <div style="max-width: 1200px; margin: 0 auto; display: flex; justify-content: space-between; align-items: center;">
                        <a href="#" onclick="app.navigate('home'); return false;" style="display: flex; align-items: center; gap: 0.5rem; font-size: 1.5rem; font-weight: bold; color: white; text-decoration: none;">
                            🦀 JetCrab Simple App
                        </a>
                        <nav style="display: flex; gap: 2rem;">
                            ${navItems.map(item => `
                                <a href="#" onclick="app.navigate('${item.id}'); return false;" 
                                   style="color: white; text-decoration: none; padding: 0.5rem 1rem; border-radius: 0.5rem; background: ${this.state.currentPage === item.id ? 'rgba(255,255,255,0.3)' : 'transparent'};">
                                    <span style="margin-right: 0.5rem;">${item.icon}</span>
                                    ${item.label}
                                </a>
                            `).join('')}
                        </nav>
                    </div>
                </header>
                <main style="flex: 1; padding: 2rem; max-width: 1200px; margin: 0 auto; width: 100%;">
                    ${content}
                </main>
            </div>
        `;
    }
}

// Initialize the app
function initializeApp() {
    console.log('Initializing Simple App...');
    
    // Create component instances
    window.counter = new Counter();
    window.todoApp = new TodoApp();
    window.app = new App();
    
    // Render the app
    document.body.innerHTML = app.render();
    
    console.log('✅ Simple app rendered successfully!');
}

// Wait for DOM to be ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initializeApp);
} else {
    initializeApp();
}

// Handle errors
window.addEventListener('error', (event) => {
    console.error('Application error:', event.error);
});

window.addEventListener('unhandledrejection', (event) => {
    console.error('Unhandled promise rejection:', event.reason);
});
