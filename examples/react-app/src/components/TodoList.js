/**
 * TodoList Component
 * 
 * Todo list component demonstrating:
 * - useState and useEffect hooks
 * - Local storage integration
 * - Form handling
 * - List rendering with keys
 * - CRUD operations
 */

const React = require('react');

function TodoList({ onStatsUpdate }) {
    const [todos, setTodos] = React.useState([]);
    const [newTodo, setNewTodo] = React.useState('');
    const [filter, setFilter] = React.useState('all');

    // Load todos from localStorage on component mount
    React.useEffect(() => {
        const savedTodos = localStorage.getItem('jetcrab-todos');
        if (savedTodos) {
            try {
                const parsedTodos = JSON.parse(savedTodos);
                setTodos(parsedTodos);
                console.log('Loaded todos from localStorage:', parsedTodos.length);
            } catch (error) {
                console.error('Error loading todos:', error);
            }
        }
    }, []);

    // Save todos to localStorage whenever todos change
    React.useEffect(() => {
        localStorage.setItem('jetcrab-todos', JSON.stringify(todos));
        console.log('Saved todos to localStorage:', todos.length);
    }, [todos]);

    // Add new todo
    const addTodo = (e) => {
        e.preventDefault();
        if (newTodo.trim()) {
            const todo = {
                id: Date.now(),
                text: newTodo.trim(),
                completed: false,
                createdAt: new Date().toISOString()
            };
            setTodos(prev => [todo, ...prev]);
            setNewTodo('');
            onStatsUpdate('todosCreated');
        }
    };

    // Toggle todo completion
    const toggleTodo = (id) => {
        setTodos(prev => prev.map(todo => {
            if (todo.id === id) {
                const wasCompleted = todo.completed;
                const updatedTodo = { ...todo, completed: !todo.completed };
                if (!wasCompleted && updatedTodo.completed) {
                    onStatsUpdate('todosCompleted');
                }
                return updatedTodo;
            }
            return todo;
        }));
    };

    // Delete todo
    const deleteTodo = (id) => {
        setTodos(prev => prev.filter(todo => todo.id !== id));
    };

    // Clear completed todos
    const clearCompleted = () => {
        setTodos(prev => prev.filter(todo => !todo.completed));
    };

    // Filter todos based on current filter
    const filteredTodos = todos.filter(todo => {
        switch (filter) {
            case 'active':
                return !todo.completed;
            case 'completed':
                return todo.completed;
            default:
                return true;
        }
    });

    // Get statistics
    const stats = {
        total: todos.length,
        active: todos.filter(todo => !todo.completed).length,
        completed: todos.filter(todo => todo.completed).length
    };

    return (
        <div>
            {/* Add Todo Form */}
            <form className="todo-form" onSubmit={addTodo}>
                <input
                    type="text"
                    className="todo-input"
                    placeholder="Add a new todo..."
                    value={newTodo}
                    onChange={(e) => setNewTodo(e.target.value)}
                />
                <button type="submit" className="btn btn-primary">
                    Add Todo
                </button>
            </form>

            {/* Filter Buttons */}
            <div style={{ marginBottom: '1rem', display: 'flex', gap: '0.5rem' }}>
                {['all', 'active', 'completed'].map(filterType => (
                    <button
                        key={filterType}
                        className={`btn ${filter === filterType ? 'btn-primary' : 'btn-secondary'}`}
                        onClick={() => setFilter(filterType)}
                        style={{ textTransform: 'capitalize' }}
                    >
                        {filterType} ({filterType === 'all' ? stats.total : stats[filterType]})
                    </button>
                ))}
            </div>

            {/* Clear Completed Button */}
            {stats.completed > 0 && (
                <div style={{ marginBottom: '1rem' }}>
                    <button 
                        className="btn btn-danger" 
                        onClick={clearCompleted}
                    >
                        Clear Completed ({stats.completed})
                    </button>
                </div>
            )}

            {/* Todo List */}
            {filteredTodos.length === 0 ? (
                <div style={{ 
                    textAlign: 'center', 
                    padding: '2rem', 
                    color: '#666',
                    background: 'rgba(255, 255, 255, 0.5)',
                    borderRadius: '0.5rem'
                }}>
                    {todos.length === 0 ? (
                        <p>No todos yet. Add one above! 📝</p>
                    ) : (
                        <p>No {filter} todos found. 🎉</p>
                    )}
                </div>
            ) : (
                <ul className="todo-list">
                    {filteredTodos.map(todo => (
                        <li 
                            key={todo.id} 
                            className={`todo-item ${todo.completed ? 'completed' : ''}`}
                        >
                            <input
                                type="checkbox"
                                className="todo-checkbox"
                                checked={todo.completed}
                                onChange={() => toggleTodo(todo.id)}
                            />
                            <span className="todo-text">{todo.text}</span>
                            <button 
                                className="todo-delete"
                                onClick={() => deleteTodo(todo.id)}
                            >
                                Delete
                            </button>
                        </li>
                    ))}
                </ul>
            )}

            {/* Statistics */}
            <div className="stats" style={{ marginTop: '2rem' }}>
                <div className="stat-item">
                    <div className="stat-value">{stats.total}</div>
                    <div className="stat-label">Total</div>
                </div>
                <div className="stat-item">
                    <div className="stat-value">{stats.active}</div>
                    <div className="stat-label">Active</div>
                </div>
                <div className="stat-item">
                    <div className="stat-value">{stats.completed}</div>
                    <div className="stat-label">Completed</div>
                </div>
            </div>

            {/* Tips */}
            <div style={{ marginTop: '2rem', fontSize: '0.9rem', color: '#666' }}>
                <p>💡 Tips:</p>
                <ul style={{ marginTop: '0.5rem', paddingLeft: '1.5rem' }}>
                    <li>Click the checkbox to mark todos as complete</li>
                    <li>Use the filter buttons to view different todo states</li>
                    <li>Your todos are automatically saved to localStorage</li>
                    <li>Click "Delete" to remove individual todos</li>
                </ul>
            </div>
        </div>
    );
}

module.exports = { default: TodoList };

