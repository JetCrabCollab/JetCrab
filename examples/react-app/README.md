# React App - JetCrab Example

A modern React application built with JetCrab runtime, demonstrating how to create interactive web applications using React components and JetCrab's JavaScript engine.

## Features Demonstrated

- React component architecture
- State management with hooks
- Event handling
- Component lifecycle
- Props and state
- Modern JavaScript features
- JetCrab runtime integration

## Getting Started

### 1. Initialize the Project

```bash
# Navigate to the example directory
cd examples/react-app

# Initialize a new JetCrab project
claw init

# Install React dependencies
claw install react react-dom
```

### 2. Run the Application

```bash
# Start the React application
jetcrab run src/index.js

# Or use development mode with hot reload
claw dev
```

### 3. Access the Application

Open your browser and navigate to:
- **Main Application**: http://localhost:3000

## Project Structure

```
react-app/
├── README.md           # This file
├── package.json        # Project configuration
├── public/             # Static assets
│   ├── index.html     # HTML template
│   └── style.css      # Global styles
├── src/               # React source code
│   ├── index.js       # Application entry point
│   ├── App.js         # Main App component
│   ├── components/    # React components
│   │   ├── Header.js  # Header component
│   │   ├── Counter.js # Counter component
│   │   └── TodoList.js # Todo list component
│   └── hooks/         # Custom hooks
│       └── useCounter.js
└── tests/             # Test files
    └── App.test.js
```

## What This Example Shows

1. **React Components**: Functional components with hooks
2. **State Management**: useState and useEffect hooks
3. **Event Handling**: Click events and form interactions
4. **Props**: Passing data between components
5. **Custom Hooks**: Reusable stateful logic
6. **Component Composition**: Building complex UIs from simple components

## Components Overview

### App.js
Main application component that orchestrates the entire app.

### Header.js
Navigation header with branding and navigation links.

### Counter.js
Interactive counter component demonstrating state management.

### TodoList.js
Todo list application showing CRUD operations and local storage.

## Development

### Running Tests

```bash
# Run all tests
claw test

# Run specific test suite
jetcrab run tests/App.test.js
```

### Building for Production

```bash
# Build optimized version
claw build

# Start production server
NODE_ENV=production jetcrab run src/index.js
```

## React Features Demonstrated

- **Functional Components**: Modern React with hooks
- **useState**: Local component state management
- **useEffect**: Side effects and lifecycle management
- **Custom Hooks**: Reusable stateful logic
- **Event Handling**: User interactions
- **Conditional Rendering**: Dynamic UI updates
- **Lists and Keys**: Rendering dynamic lists
- **Forms**: Controlled components and form handling

## Next Steps

- Add routing with React Router
- Implement state management with Redux or Context API
- Add styling with CSS-in-JS or styled-components
- Integrate with backend APIs
- Add testing with Jest and React Testing Library
- Deploy to production
