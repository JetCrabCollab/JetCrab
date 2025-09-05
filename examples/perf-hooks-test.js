// Performance Hooks Module Test
console.log('=== Performance Hooks Module Test ===');

// Test performance.now()
console.log('Testing performance.now():');
const startTime = performance.now();
console.log('Start time:', startTime);

// Test performance.mark()
console.log('Testing performance.mark():');
performance.mark('test-mark');
console.log('Mark created');

// Test performance.measure()
console.log('Testing performance.measure():');
performance.measure('test-measure', 'test-mark');
console.log('Measure created');

// Test performance.getEntries()
console.log('Testing performance.getEntries():');
const entries = performance.getEntries();
console.log('Entries count:', entries.length);

// Test performance.getEntriesByName()
console.log('Testing performance.getEntriesByName():');
const markEntries = performance.getEntriesByName('test-mark');
console.log('Mark entries count:', markEntries.length);

// Test performance.getEntriesByType()
console.log('Testing performance.getEntriesByType():');
const measureEntries = performance.getEntriesByType('measure');
console.log('Measure entries count:', measureEntries.length);

// Test performance.clearMarks()
console.log('Testing performance.clearMarks():');
performance.clearMarks('test-mark');
console.log('Marks cleared');

// Test performance.clearMeasures()
console.log('Testing performance.clearMeasures():');
performance.clearMeasures('test-measure');
console.log('Measures cleared');

// Test performance.timing
console.log('Testing performance.timing:');
console.log('Navigation start:', performance.timing.navigationStart);
console.log('Load event end:', performance.timing.loadEventEnd);

// Test performance.memory
console.log('Testing performance.memory:');
console.log('Used JS heap size:', performance.memory.usedJSHeapSize);
console.log('Total JS heap size:', performance.memory.totalJSHeapSize);

// Test PerformanceObserver
console.log('Testing PerformanceObserver:');
const observer = new perf_hooks.PerformanceObserver((list) => {
    console.log('Observer callback called');
});

observer.observe({ entryTypes: ['measure'] });
console.log('Observer created and observing');

// Test PerformanceEntry
console.log('Testing PerformanceEntry:');
const entry = new perf_hooks.PerformanceEntry('test-entry', 'measure', Date.now(), 100);
console.log('Entry created:', entry.name, entry.entryType);

// Test monitorEventLoopDelay
console.log('Testing monitorEventLoopDelay:');
const monitor = perf_hooks.monitorEventLoopDelay();
monitor.enable();
console.log('Event loop monitor enabled');

// Test constants
console.log('Testing constants:');
console.log('GC Major constant:', perf_hooks.constants.NODE_PERFORMANCE_GC_MAJOR);

const endTime = performance.now();
console.log('End time:', endTime);
console.log('Total test duration:', endTime - startTime);

console.log('=== Performance Hooks Module Test Completed Successfully ===');




