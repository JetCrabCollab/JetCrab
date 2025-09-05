// Cluster Example - Multi-process Application
// This example demonstrates how to use the Cluster module for load balancing

console.log('🚀 Starting Cluster Example...');

// Check if this is the master process
if (cluster.isMaster) {
    console.log('👑 Master process started');

    // Setup master with configuration
    cluster.setupMaster({
        maxWorkers: 4,
        minWorkers: 2,
        restartDelay: 1000,
        heartbeatInterval: 5000,
        loadBalanceStrategy: 'RoundRobin'
    }).then(() => {
        console.log('✅ Master setup complete');

        // Fork workers
        const worker1 = cluster.fork('worker-script.js');
        const worker2 = cluster.fork('worker-script.js');
        const worker3 = cluster.fork('worker-script.js');

        console.log('🔄 Forked workers:', worker1, worker2, worker3);

        // Get worker count
        const workerCount = cluster.getWorkerCount();
        console.log('👥 Total workers:', workerCount);

        // Get all workers info
        const allWorkers = cluster.getAllWorkers();
        console.log('📊 All workers info:', allWorkers);

        // Test load balancing
        const target = cluster.getLoadBalanceTarget();
        console.log('🎯 Load balance target:', target);

        // Send message to specific worker
        cluster.send(worker1, { type: 'greeting', message: 'Hello from master!' });

        // Broadcast message to all workers
        cluster.broadcast({ type: 'broadcast', message: 'Hello all workers!' });

        // Simulate some work
        setTimeout(() => {
            console.log('⏰ Master work completed');

            // Kill a worker
            cluster.killWorker(worker1);
            console.log('💀 Killed worker 1');

            // Restart the worker
            setTimeout(() => {
                cluster.restartWorker(worker1, 'worker-script.js');
                console.log('🔄 Restarted worker 1');
            }, 2000);

        }, 3000);

    }).catch(err => {
        console.error('❌ Master setup failed:', err);
    });

} else {
    console.log('👷 Worker process started');

    // Setup worker
    const workerId = process.env.CLUSTER_WORKER_ID || 'unknown';
    cluster.setupWorker(workerId).then(() => {
        console.log('✅ Worker setup complete, ID:', workerId);

        // Simulate worker work
        setInterval(() => {
            console.log('🔄 Worker', workerId, 'is working...');
        }, 1000);

    }).catch(err => {
        console.error('❌ Worker setup failed:', err);
    });
}

console.log('✅ Cluster example completed');



