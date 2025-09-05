// Worker Threads Example - Multi-threaded Application
// This example demonstrates how to use the Worker Threads module for parallel execution

console.log('🧵 Starting Worker Threads Example...');

// Check if this is the main thread
if (worker_threads.isMainThread) {
    console.log('🏠 Main thread started');

    // Create a worker thread
    const worker = new worker_threads.Worker('./worker-script.js', {
        workerData: {
            message: 'Hello from main thread!',
            iterations: 1000000
        }
    });

    console.log('✅ Worker created with ID:', worker.id);

    // Set up event listeners
    worker.on('message', (data) => {
        console.log('📨 Received message from worker:', data);

        if (data.type === 'result') {
            console.log('🎯 Worker result:', data.result);
            console.log('⏱️ Processing time:', data.processingTime, 'ms');
        }
    });

    worker.on('error', (error) => {
        console.error('❌ Worker error:', error);
    });

    worker.on('exit', (code) => {
        console.log('🚪 Worker exited with code:', code);
    });

    // Send messages to worker
    worker.postMessage({
        type: 'start',
        data: { task: 'calculate_primes', limit: 10000 }
    });

    // Send another message after a delay
    setTimeout(() => {
        worker.postMessage({
            type: 'update',
            data: { newLimit: 20000 }
        });
    }, 2000);

    // Terminate worker after some time
    setTimeout(() => {
        console.log('🛑 Terminating worker...');
        worker.terminate();
    }, 5000);

} else {
    console.log('👷 Worker thread started');
    console.log('📊 Worker data:', worker_threads.workerData);

    // Simulate some CPU-intensive work
    function calculatePrimes(limit) {
        const start = Date.now();
        const primes = [];

        for (let i = 2; i <= limit; i++) {
            let isPrime = true;
            for (let j = 2; j <= Math.sqrt(i); j++) {
                if (i % j === 0) {
                    isPrime = false;
                    break;
                }
            }
            if (isPrime) {
                primes.push(i);
            }
        }

        const end = Date.now();
        return {
            primes: primes.length,
            processingTime: end - start
        };
    }

    // Listen for messages from main thread
    if (worker_threads.parentPort) {
        worker_threads.parentPort.on('message', (data) => {
            console.log('📨 Worker received message:', data);

            if (data.type === 'start') {
                console.log('🚀 Starting calculation...');
                const result = calculatePrimes(data.data.limit);

                // Send result back to main thread
                worker_threads.parentPort.postMessage({
                    type: 'result',
                    result: result,
                    processingTime: result.processingTime
                });
            } else if (data.type === 'update') {
                console.log('🔄 Updating calculation...');
                const result = calculatePrimes(data.data.newLimit);

                worker_threads.parentPort.postMessage({
                    type: 'result',
                    result: result,
                    processingTime: result.processingTime
                });
            }
        });
    }

    // Send online message
    if (worker_threads.parentPort) {
        worker_threads.parentPort.postMessage({
            type: 'online',
            message: 'Worker is ready!'
        });
    }
}

// Test MessageChannel
console.log('🔗 Testing MessageChannel...');
const { port1, port2 } = new worker_threads.MessageChannel();

port1.on('message', (data) => {
    console.log('📨 Port1 received:', data);
    port1.postMessage({ response: 'Hello from port1!' });
});

port2.on('message', (data) => {
    console.log('📨 Port2 received:', data);
    port2.postMessage({ response: 'Hello from port2!' });
});

// Send messages between ports
port1.postMessage({ message: 'Hello from port1 to port2!' });
port2.postMessage({ message: 'Hello from port2 to port1!' });

// Test MessagePort
console.log('📡 Testing MessagePort...');
const messagePort = new worker_threads.MessagePort();

messagePort.on('message', (data) => {
    console.log('📨 MessagePort received:', data);
});

messagePort.postMessage({ message: 'Hello MessagePort!' });

// Test environment data
console.log('🌍 Testing environment data...');
worker_threads.setEnvironmentData('test_key', 'test_value');
const envData = worker_threads.getEnvironmentData('test_key');
console.log('📊 Environment data:', envData);

// Test resource limits
console.log('📊 Resource limits:', worker_threads.resourceLimits);

console.log('✅ Worker Threads example completed');



