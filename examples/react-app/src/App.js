/**
 * App Component - Main React Application
 * 
 * This is the root component of our React application, demonstrating:
 * - Component composition
 * - State management with hooks
 * - Event handling
 * - Conditional rendering
 */

const React = require('react');
const Header = require('./components/Header').default;
const Counter = require('./components/Counter').default;
const TodoList = require('./components/TodoList').default;

function App() {
    const [currentPage, setCurrentPage] = React.useState('home');
    const [appStats, setAppStats] = React.useState({
        totalClicks: 0,
        todosCreated: 0,
        todosCompleted: 0
    });

    // Update app statistics
    const updateStats = React.useCallback((type, value = 1) => {
        setAppStats(prev => ({
            ...prev,
            [type]: prev[type] + value
        }));
    }, []);

    // Handle navigation
    const handleNavigation = (page) => {
        setCurrentPage(page);
        console.log(`Navigating to: ${page}`);
    };

    // Render current page content
    const renderPageContent = () => {
        switch (currentPage) {
            case 'counter':
                return (
                    <div className="card fade-in">
                        <h2>Interactive Counter</h2>
                        <Counter onStatsUpdate={updateStats} />
                    </div>
                );
            case 'todos':
                return (
                    <div className="card fade-in">
                        <h2>Todo List</h2>
                        <TodoList onStatsUpdate={updateStats} />
                    </div>
                );
            default:
                return (
                    <div className="card fade-in">
                        <h2>Welcome to JetCrab React App! 🦀</h2>
                        <p>
                            This is a modern React application running on JetCrab runtime.
                            Explore the different features using the navigation above.
                        </p>
                        
                        <div className="stats">
                            <div className="stat-item">
                                <div className="stat-value">{appStats.totalClicks}</div>
                                <div className="stat-label">Total Clicks</div>
                            </div>
                            <div className="stat-item">
                                <div className="stat-value">{appStats.todosCreated}</div>
                                <div className="stat-label">Todos Created</div>
                            </div>
                            <div className="stat-item">
                                <div className="stat-value">{appStats.todosCompleted}</div>
                                <div className="stat-label">Todos Completed</div>
                            </div>
                        </div>

                        <div style={{ marginTop: '2rem' }}>
                            <h3>Features Demonstrated:</h3>
                            <ul style={{ marginTop: '1rem', paddingLeft: '2rem' }}>
                                <li>React functional components with hooks</li>
                                <li>State management with useState</li>
                                <li>Event handling and user interactions</li>
                                <li>Component composition and props</li>
                                <li>Conditional rendering</li>
                                <li>Local storage integration</li>
                                <li>Responsive design</li>
                                <li>Modern JavaScript features</li>
                            </ul>
                        </div>
                    </div>
                );
        }
    };

    return (
        <div className="app">
            <Header 
                currentPage={currentPage} 
                onNavigate={handleNavigation}
            />
            
            <main className="main">
                {renderPageContent()}
            </main>
        </div>
    );
}

// Export the component
module.exports = { default: App };

