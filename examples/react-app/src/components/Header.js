/**
 * Header Component
 * 
 * Navigation header component demonstrating:
 * - Props and event handling
 * - Conditional styling
 * - Component composition
 */

const React = require('react');

function Header({ currentPage, onNavigate }) {
    const navItems = [
        { id: 'home', label: 'Home', icon: '🏠' },
        { id: 'counter', label: 'Counter', icon: '🔢' },
        { id: 'todos', label: 'Todos', icon: '✅' }
    ];

    const handleNavClick = (pageId) => {
        onNavigate(pageId);
    };

    return (
        <header className="header">
            <div className="header-content">
                <a href="#" className="logo jetcrab-brand" onClick={(e) => {
                    e.preventDefault();
                    onNavigate('home');
                }}>
                    JetCrab React App
                </a>
                
                <nav className="nav">
                    {navItems.map(item => (
                        <a
                            key={item.id}
                            href="#"
                            className={currentPage === item.id ? 'active' : ''}
                            onClick={(e) => {
                                e.preventDefault();
                                handleNavClick(item.id);
                            }}
                            style={{
                                background: currentPage === item.id ? 'rgba(255, 255, 255, 0.3)' : 'transparent'
                            }}
                        >
                            <span style={{ marginRight: '0.5rem' }}>{item.icon}</span>
                            {item.label}
                        </a>
                    ))}
                </nav>
            </div>
        </header>
    );
}

module.exports = { default: Header };

