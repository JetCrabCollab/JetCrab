// Sorting Algorithms Examples
// Multiple sorting implementations to demonstrate different approaches

console.log("=== SORTING ALGORITHMS EXAMPLES ===\n");

// Bubble Sort
function bubbleSort(arr) {
    let n = arr.length;
    let swapped;
    
    do {
        swapped = false;
        for (let i = 0; i < n - 1; i++) {
            if (arr[i] > arr[i + 1]) {
                let temp = arr[i];
                arr[i] = arr[i + 1];
                arr[i + 1] = temp;
                swapped = true;
            }
        }
        n--;
    } while (swapped);
    
    return arr;
}

// Selection Sort
function selectionSort(arr) {
    let n = arr.length;
    
    for (let i = 0; i < n - 1; i++) {
        let minIndex = i;
        
        for (let j = i + 1; j < n; j++) {
            if (arr[j] < arr[minIndex]) {
                minIndex = j;
            }
        }
        
        if (minIndex !== i) {
            let temp = arr[i];
            arr[i] = arr[minIndex];
            arr[minIndex] = temp;
        }
    }
    
    return arr;
}

// Insertion Sort
function insertionSort(arr) {
    let n = arr.length;
    
    for (let i = 1; i < n; i++) {
        let key = arr[i];
        let j = i - 1;
        
        while (j >= 0 && arr[j] > key) {
            arr[j + 1] = arr[j];
            j--;
        }
        
        arr[j + 1] = key;
    }
    
    return arr;
}

// Quick Sort
function quickSort(arr) {
    if (arr.length <= 1) {
        return arr;
    }
    
    let pivot = arr[Math.floor(arr.length / 2)];
    let left = [];
    let middle = [];
    let right = [];
    
    for (let i = 0; i < arr.length; i++) {
        if (arr[i] < pivot) {
            left.push(arr[i]);
        } else if (arr[i] === pivot) {
            middle.push(arr[i]);
        } else {
            right.push(arr[i]);
        }
    }
    
    return [...quickSort(left), ...middle, ...quickSort(right)];
}

// Merge Sort
function mergeSort(arr) {
    if (arr.length <= 1) {
        return arr;
    }
    
    let mid = Math.floor(arr.length / 2);
    let left = mergeSort(arr.slice(0, mid));
    let right = mergeSort(arr.slice(mid));
    
    return merge(left, right);
}

function merge(left, right) {
    let result = [];
    let i = 0, j = 0;
    
    while (i < left.length && j < right.length) {
        if (left[i] <= right[j]) {
            result.push(left[i]);
            i++;
        } else {
            result.push(right[j]);
            j++;
        }
    }
    
    return result.concat(left.slice(i)).concat(right.slice(j));
}

// Test data
let testArray = [64, 34, 25, 12, 22, 11, 90, 88, 76, 54, 32, 21, 19, 8, 5, 3, 1];

console.log("Original array:", testArray);

// Test different sorting algorithms
console.log("\n1. Bubble Sort:");
let bubbleResult = bubbleSort([...testArray]);
console.log("Result:", bubbleResult);

console.log("\n2. Selection Sort:");
let selectionResult = selectionSort([...testArray]);
console.log("Result:", selectionResult);

console.log("\n3. Insertion Sort:");
let insertionResult = insertionSort([...testArray]);
console.log("Result:", insertionResult);

console.log("\n4. Quick Sort:");
let quickResult = quickSort([...testArray]);
console.log("Result:", quickResult);

console.log("\n5. Merge Sort:");
let mergeResult = mergeSort([...testArray]);
console.log("Result:", mergeResult);

// Performance comparison
console.log("\n=== PERFORMANCE COMPARISON ===");

function measureSortingTime(sortFunction, arr, name) {
    let start = Date.now();
    let result = sortFunction([...arr]);
    let end = Date.now();
    console.log(`${name}: ${end - start}ms`);
    return result;
}

let largeArray = [];
for (let i = 0; i < 1000; i++) {
    largeArray.push(Math.floor(Math.random() * 1000));
}

console.log("\nSorting 1000 random numbers:");
measureSortingTime(bubbleSort, largeArray, "Bubble Sort");
measureSortingTime(selectionSort, largeArray, "Selection Sort");
measureSortingTime(insertionSort, largeArray, "Insertion Sort");
measureSortingTime(quickSort, largeArray, "Quick Sort");
measureSortingTime(mergeSort, largeArray, "Merge Sort");

// Utility functions
function isSorted(arr) {
    for (let i = 1; i < arr.length; i++) {
        if (arr[i] < arr[i - 1]) {
            return false;
        }
    }
    return true;
}

console.log("\n=== VALIDATION ===");
console.log("Bubble sort result is sorted:", isSorted(bubbleResult));
console.log("Quick sort result is sorted:", isSorted(quickResult));
console.log("Merge sort result is sorted:", isSorted(mergeResult));
