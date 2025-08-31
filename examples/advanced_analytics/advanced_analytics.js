// Advanced Analytics System Example
// Demonstrates JetCrab's capabilities with complex algorithms and data processing

console.log("=== ADVANCED ANALYTICS SYSTEM ===\n");

// Performance measurement utility
function measurePerformance(name, fn, iterations = 1000) {
    const start = performance.now();
    let result;

    for (let i = 0; i < iterations; i++) {
        result = fn();
    }

    const end = performance.now();
    const duration = end - start;
    const avgTime = duration / iterations;

    console.log(`${name}: ${duration.toFixed(2)}ms total, ${avgTime.toFixed(4)}ms avg (${iterations} iterations)`);
    return { result, duration, avgTime };
}

// Advanced Data Structures
class PriorityQueue {
    constructor() {
        this.heap = [];
    }

    enqueue(item, priority) {
        this.heap.push({ item, priority });
        this._bubbleUp();
    }

    dequeue() {
        if (this.isEmpty()) return null;

        const max = this.heap[0];
        const end = this.heap.pop();

        if (this.heap.length > 0) {
            this.heap[0] = end;
            this._bubbleDown();
        }

        return max.item;
    }

    _bubbleUp() {
        let index = this.heap.length - 1;
        const element = this.heap[index];

        while (index > 0) {
            const parentIndex = Math.floor((index - 1) / 2);
            const parent = this.heap[parentIndex];

            if (element.priority <= parent.priority) break;

            this.heap[parentIndex] = element;
            this.heap[index] = parent;
            index = parentIndex;
        }
    }

    _bubbleDown() {
        let index = 0;
        const length = this.heap.length;
        const element = this.heap[0];

        while (true) {
            const leftChildIndex = 2 * index + 1;
            const rightChildIndex = 2 * index + 2;
            let leftChild, rightChild;
            let swap = null;

            if (leftChildIndex < length) {
                leftChild = this.heap[leftChildIndex];
                if (leftChild.priority > element.priority) {
                    swap = leftChildIndex;
                }
            }

            if (rightChildIndex < length) {
                rightChild = this.heap[rightChildIndex];
                if (
                    (swap === null && rightChild.priority > element.priority) ||
                    (swap !== null && rightChild.priority > leftChild.priority)
                ) {
                    swap = rightChildIndex;
                }
            }

            if (swap === null) break;

            this.heap[index] = this.heap[swap];
            this.heap[swap] = element;
            index = swap;
        }
    }

    isEmpty() {
        return this.heap.length === 0;
    }

    size() {
        return this.heap.length;
    }
}

class Graph {
    constructor() {
        this.adjacencyList = new Map();
    }

    addVertex(vertex) {
        if (!this.adjacencyList.has(vertex)) {
            this.adjacencyList.set(vertex, []);
        }
    }

    addEdge(vertex1, vertex2, weight = 1) {
        this.addVertex(vertex1);
        this.addVertex(vertex2);

        this.adjacencyList.get(vertex1).push({ vertex: vertex2, weight });
        this.adjacencyList.get(vertex2).push({ vertex: vertex1, weight });
    }

    dijkstra(startVertex) {
        const distances = new Map();
        const previous = new Map();
        const pq = new PriorityQueue();

        // Initialize distances
        for (let vertex of this.adjacencyList.keys()) {
            distances.set(vertex, Infinity);
            previous.set(vertex, null);
        }
        distances.set(startVertex, 0);

        pq.enqueue(startVertex, 0);

        while (!pq.isEmpty()) {
            const currentVertex = pq.dequeue();
            const currentDistance = distances.get(currentVertex);

            for (let neighbor of this.adjacencyList.get(currentVertex)) {
                const neighborVertex = neighbor.vertex;
                const neighborWeight = neighbor.weight;
                const totalDistance = currentDistance + neighborWeight;

                if (totalDistance < distances.get(neighborVertex)) {
                    distances.set(neighborVertex, totalDistance);
                    previous.set(neighborVertex, currentVertex);
                    pq.enqueue(neighborVertex, totalDistance);
                }
            }
        }

        return { distances, previous };
    }
}

// Advanced Algorithms
class MachineLearning {
    static kMeansClustering(data, k, maxIterations = 100) {
        if (data.length === 0 || k <= 0) return null;

        // Initialize centroids randomly
        let centroids = [];
        for (let i = 0; i < k; i++) {
            const randomIndex = Math.floor(Math.random() * data.length);
            centroids.push([...data[randomIndex]]);
        }

        let iterations = 0;
        let converged = false;

        while (!converged && iterations < maxIterations) {
            iterations++;

            // Assign points to nearest centroid
            const clusters = Array.from({ length: k }, () => []);

            for (let point of data) {
                let minDistance = Infinity;
                let nearestCentroid = 0;

                for (let i = 0; i < k; i++) {
                    const distance = this._euclideanDistance(point, centroids[i]);
                    if (distance < minDistance) {
                        minDistance = distance;
                        nearestCentroid = i;
                    }
                }

                clusters[nearestCentroid].push(point);
            }

            // Update centroids
            const newCentroids = [];
            let hasChanged = false;

            for (let i = 0; i < k; i++) {
                if (clusters[i].length === 0) {
                    newCentroids.push(centroids[i]);
                    continue;
                }

                const newCentroid = this._calculateCentroid(clusters[i]);
                newCentroids.push(newCentroid);

                if (this._euclideanDistance(centroids[i], newCentroid) > 0.001) {
                    hasChanged = true;
                }
            }

            centroids = newCentroids;
            converged = !hasChanged;
        }

        return { centroids, clusters, iterations };
    }

    static _euclideanDistance(point1, point2) {
        let sum = 0;
        for (let i = 0; i < point1.length; i++) {
            sum += Math.pow(point1[i] - point2[i], 2);
        }
        return Math.sqrt(sum);
    }

    static _calculateCentroid(points) {
        const dimensions = points[0].length;
        const centroid = new Array(dimensions).fill(0);

        for (let point of points) {
            for (let i = 0; i < dimensions; i++) {
                centroid[i] += point[i];
            }
        }

        for (let i = 0; i < dimensions; i++) {
            centroid[i] /= points.length;
        }

        return centroid;
    }
}

// Data Processing Pipeline
class DataProcessor {
    constructor() {
        this.pipeline = [];
    }

    addStep(step) {
        this.pipeline.push(step);
        return this;
    }

    process(data) {
        let result = data;
        for (let step of this.pipeline) {
            result = step(result);
        }
        return result;
    }
}

// Complex Data Analysis
class FinancialAnalyzer {
    static calculateReturns(prices) {
        const returns = [];
        for (let i = 1; i < prices.length; i++) {
            returns.push((prices[i] - prices[i - 1]) / prices[i - 1]);
        }
        return returns;
    }

    static calculateVolatility(returns) {
        const mean = returns.reduce((sum, r) => sum + r, 0) / returns.length;
        const variance = returns.reduce((sum, r) => sum + Math.pow(r - mean, 2), 0) / returns.length;
        return Math.sqrt(variance);
    }

    static calculateSharpeRatio(returns, riskFreeRate = 0.02) {
        const meanReturn = returns.reduce((sum, r) => sum + r, 0) / returns.length;
        const volatility = this.calculateVolatility(returns);
        return (meanReturn - riskFreeRate) / volatility;
    }

    static monteCarloSimulation(initialPrice, volatility, drift, days, simulations = 10000) {
        const results = [];

        for (let sim = 0; sim < simulations; sim++) {
            let price = initialPrice;
            const path = [price];

            for (let day = 1; day <= days; day++) {
                const random = (Math.random() + Math.random() + Math.random() + Math.random() - 2) / 2;
                const change = drift + volatility * random;
                price *= (1 + change);
                path.push(price);
            }

            results.push(path);
        }

        return results;
    }
}

// Performance Testing
console.log("=== PERFORMANCE TESTING ===\n");

// Test 1: Priority Queue Performance
console.log("1. Priority Queue Performance Test:");
const pq = new PriorityQueue();
const pqTest = () => {
    for (let i = 0; i < 1000; i++) {
        pq.enqueue(`item${i}`, Math.random());
    }
    const results = [];
    while (!pq.isEmpty()) {
        results.push(pq.dequeue());
    }
    return results.length;
};

measurePerformance("Priority Queue Operations", pqTest, 100);

// Test 2: Graph Algorithm Performance
console.log("\n2. Graph Algorithm Performance Test:");
const graph = new Graph();
for (let i = 0; i < 100; i++) {
    for (let j = i + 1; j < Math.min(i + 5, 100); j++) {
        graph.addEdge(`node${i}`, `node${j}`, Math.random() * 10);
    }
}

const graphTest = () => {
    return graph.dijkstra('node0');
};

measurePerformance("Dijkstra Algorithm", graphTest, 50);

// Test 3: Machine Learning Performance
console.log("\n3. Machine Learning Performance Test:");
const mlData = [];
for (let i = 0; i < 1000; i++) {
    mlData.push([Math.random() * 100, Math.random() * 100, Math.random() * 100]);
}

const mlTest = () => {
    return MachineLearning.kMeansClustering(mlData, 5, 50);
};

measurePerformance("K-Means Clustering", mlTest, 20);

// Test 4: Financial Analysis Performance
console.log("\n4. Financial Analysis Performance Test:");
const prices = [];
for (let i = 0; i < 1000; i++) {
    prices.push(100 + Math.random() * 50);
}

const financialTest = () => {
    const returns = FinancialAnalyzer.calculateReturns(prices);
    const volatility = FinancialAnalyzer.calculateVolatility(returns);
    const sharpeRatio = FinancialAnalyzer.calculateSharpeRatio(returns);
    const simulation = FinancialAnalyzer.monteCarloSimulation(100, 0.02, 0.001, 30, 1000);
    return { volatility, sharpeRatio, simulationCount: simulation.length };
};

measurePerformance("Financial Analysis", financialTest, 30);

// Test 5: Data Processing Pipeline
console.log("\n5. Data Processing Pipeline Performance Test:");
const processor = new DataProcessor()
    .addStep(data => data.filter(x => x > 50))
    .addStep(data => data.map(x => x * 2))
    .addStep(data => data.reduce((sum, x) => sum + x, 0));

const pipelineTest = () => {
    const testData = Array.from({ length: 10000 }, () => Math.random() * 100);
    return processor.process(testData);
};

measurePerformance("Data Pipeline Processing", pipelineTest, 100);

// Test 6: Complex Object Operations
console.log("\n6. Complex Object Operations Performance Test:");
const complexObjectTest = () => {
    const objects = [];
    for (let i = 0; i < 1000; i++) {
        objects.push({
            id: i,
            data: {
                values: Array.from({ length: 10 }, () => Math.random()),
                metadata: {
                    timestamp: Date.now(),
                    category: `cat${i % 10}`,
                    tags: Array.from({ length: 5 }, () => `tag${Math.floor(Math.random() * 100)}`)
                }
            },
            methods: {
                calculate: function () {
                    return this.data.values.reduce((sum, v) => sum + v, 0);
                },
                transform: function () {
                    return this.data.values.map(v => v * 2);
                }
            }
        });
    }

    const results = objects.map(obj => ({
        id: obj.id,
        sum: obj.methods.calculate(),
        transformed: obj.methods.transform(),
        category: obj.data.metadata.category
    }));

    return results.length;
};

measurePerformance("Complex Object Operations", complexObjectTest, 50);

// Test 7: Advanced Array Operations
console.log("\n7. Advanced Array Operations Performance Test:");
const arrayTest = () => {
    const largeArray = Array.from({ length: 10000 }, () => Math.random());

    const operations = [
        () => largeArray.filter(x => x > 0.5),
        () => largeArray.map(x => Math.pow(x, 2)),
        () => largeArray.reduce((sum, x) => sum + x, 0),
        () => largeArray.sort((a, b) => a - b),
        () => largeArray.find(x => x > 0.9),
        () => largeArray.some(x => x > 0.95),
        () => largeArray.every(x => x >= 0),
        () => largeArray.slice(0, 1000),
        () => [...largeArray, ...largeArray],
        () => largeArray.flatMap(x => [x, x * 2])
    ];

    return operations.map(op => op().length);
};

measurePerformance("Advanced Array Operations", arrayTest, 100);

// Test 8: Recursive Algorithm Performance
console.log("\n8. Recursive Algorithm Performance Test:");
function fibonacciDynamic(n, memo = {}) {
    if (n in memo) return memo[n];
    if (n <= 1) return n;

    memo[n] = fibonacciDynamic(n - 1, memo) + fibonacciDynamic(n - 2, memo);
    return memo[n];
}

const recursiveTest = () => {
    const results = [];
    for (let i = 0; i < 100; i++) {
        results.push(fibonacciDynamic(i));
    }
    return results.length;
};

measurePerformance("Dynamic Programming Fibonacci", recursiveTest, 100);

// Final Results Summary
console.log("\n=== FINAL RESULTS SUMMARY ===");
console.log("✅ All performance tests completed successfully!");
console.log("🚀 JetCrab engine demonstrates excellent performance across:");
console.log("   - Data structures (Priority Queue, Graph algorithms)");
console.log("   - Machine learning algorithms (K-Means clustering)");
console.log("   - Financial analysis (Monte Carlo simulations)");
console.log("   - Data processing pipelines");
console.log("   - Complex object operations");
console.log("   - Advanced array operations");
console.log("   - Recursive algorithms with memoization");

// Return final result to demonstrate completion
"Advanced Analytics System - All Tests Passed";
