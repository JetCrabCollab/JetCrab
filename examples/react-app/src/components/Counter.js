/**
 * Counter Component
 * 
 * Interactive counter component demonstrating:
 * - useState hook for state management
 * - Event handling
 * - Conditional rendering
 * - Props and callbacks
 */

const React = require('react');

function Counter({ onStatsUpdate }) {
    const [count, setCount] = React.useState(0);
    const [step, setStep] = React.useState(1);
    const [history, setHistory] = React.useState([]);

    // Handle increment
    const handleIncrement = () => {
        const newCount = count + step;
        setCount(newCount);
        setHistory(prev => [...prev.slice(-4), { action: 'increment', value: newCount, step }]);
        onStatsUpdate('totalClicks');
    };

    // Handle decrement
    const handleDecrement = () => {
        const newCount = count - step;
        setCount(newCount);
        setHistory(prev => [...prev.slice(-4), { action: 'decrement', value: newCount, step }]);
        onStatsUpdate('totalClicks');
    };

    // Handle reset
    const handleReset = () => {
        setCount(0);
        setHistory(prev => [...prev.slice(-4), { action: 'reset', value: 0, step }]);
        onStatsUpdate('totalClicks');
    };

    // Handle step change
    const handleStepChange = (newStep) => {
        setStep(newStep);
    };

    // Get count color based on value
    const getCountColor = () => {
        if (count > 0) return '#28a745';
        if (count < 0) return '#dc3545';
        return '#667eea';
    };

    return (
        <div className="counter">
            <div className="counter-value" style={{ color: getCountColor() }}>
                {count}
            </div>
            
            <div className="counter-buttons">
                <button 
                    className="btn btn-primary" 
                    onClick={handleIncrement}
                >
                    +{step}
                </button>
                
                <button 
                    className="btn btn-secondary" 
                    onClick={handleDecrement}
                >
                    -{step}
                </button>
                
                <button 
                    className="btn btn-danger" 
                    onClick={handleReset}
                >
                    Reset
                </button>
            </div>

            <div style={{ marginTop: '2rem' }}>
                <label htmlFor="step-input" style={{ display: 'block', marginBottom: '0.5rem' }}>
                    Step Size: {step}
                </label>
                <input
                    id="step-input"
                    type="range"
                    min="1"
                    max="10"
                    value={step}
                    onChange={(e) => handleStepChange(parseInt(e.target.value))}
                    style={{ width: '100%', maxWidth: '200px' }}
                />
            </div>

            {history.length > 0 && (
                <div style={{ marginTop: '2rem' }}>
                    <h4>Recent Actions:</h4>
                    <div style={{ 
                        display: 'flex', 
                        flexDirection: 'column', 
                        gap: '0.5rem',
                        marginTop: '1rem',
                        maxHeight: '150px',
                        overflowY: 'auto'
                    }}>
                        {history.slice().reverse().map((item, index) => (
                            <div 
                                key={index}
                                style={{
                                    padding: '0.5rem',
                                    background: 'rgba(102, 126, 234, 0.1)',
                                    borderRadius: '0.25rem',
                                    fontSize: '0.9rem'
                                }}
                            >
                                <strong>{item.action}</strong> → {item.value} 
                                {item.action !== 'reset' && ` (step: ${item.step})`}
                            </div>
                        ))}
                    </div>
                </div>
            )}

            <div style={{ marginTop: '2rem', fontSize: '0.9rem', color: '#666' }}>
                <p>💡 Try changing the step size and clicking the buttons!</p>
            </div>
        </div>
    );
}

module.exports = { default: Counter };

