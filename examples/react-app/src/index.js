#!/usr/bin/env jetcrab

/**
 * React App - JetCrab Example
 * 
 * This is the main entry point for a React application running on JetCrab runtime.
 * It demonstrates how to create interactive web applications using React components.
 */

console.log('🦀 Starting React App with JetCrab Runtime...');

// Import React and ReactDOM (these will be available through JetCrab's module system)
const React = require('react');
const ReactDOM = require('react-dom');

// Import our main App component
const App = require('./App').default;

// Import styles
require('../public/style.css');

// Main application initialization
function initializeApp() {
    console.log('Initializing React application...');
    
    // Get the root element
    const rootElement = document.getElementById('root');
    
    if (!rootElement) {
        console.error('Root element not found!');
        return;
    }
    
    // Render the React app
    ReactDOM.render(React.createElement(App), rootElement);
    
    console.log('✅ React app rendered successfully!');
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

// Export for potential server-side rendering
if (typeof module !== 'undefined' && module.exports) {
    module.exports = { App, initializeApp };
}

