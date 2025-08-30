// Data Structures Examples
// Implementation of common data structures in JavaScript

console.log("=== DATA STRUCTURES EXAMPLES ===\n");

// Stack Implementation
class Stack {
    constructor() {
        this.items = [];
    }
    
    push(element) {
        this.items.push(element);
    }
    
    pop() {
        if (this.isEmpty()) {
            return "Stack is empty";
        }
        return this.items.pop();
    }
    
    peek() {
        if (this.isEmpty()) {
            return "Stack is empty";
        }
        return this.items[this.items.length - 1];
    }
    
    isEmpty() {
        return this.items.length === 0;
    }
    
    size() {
        return this.items.length;
    }
    
    clear() {
        this.items = [];
    }
    
    toString() {
        return this.items.toString();
    }
}

// Queue Implementation
class Queue {
    constructor() {
        this.items = [];
    }
    
    enqueue(element) {
        this.items.push(element);
    }
    
    dequeue() {
        if (this.isEmpty()) {
            return "Queue is empty";
        }
        return this.items.shift();
    }
    
    front() {
        if (this.isEmpty()) {
            return "Queue is empty";
        }
        return this.items[0];
    }
    
    isEmpty() {
        return this.items.length === 0;
    }
    
    size() {
        return this.items.length;
    }
    
    clear() {
        this.items = [];
    }
    
    toString() {
        return this.items.toString();
    }
}

// Linked List Implementation
class Node {
    constructor(data) {
        this.data = data;
        this.next = null;
    }
}

class LinkedList {
    constructor() {
        this.head = null;
        this.size = 0;
    }
    
    add(data) {
        let node = new Node(data);
        
        if (this.head === null) {
            this.head = node;
        } else {
            let current = this.head;
            while (current.next) {
                current = current.next;
            }
            current.next = node;
        }
        this.size++;
    }
    
    remove(data) {
        let current = this.head;
        let previous = null;
        
        while (current !== null) {
            if (current.data === data) {
                if (previous === null) {
                    this.head = current.next;
                } else {
                    previous.next = current.next;
                }
                this.size--;
                return true;
            }
            previous = current;
            current = current.next;
        }
        return false;
    }
    
    find(data) {
        let current = this.head;
        let index = 0;
        
        while (current !== null) {
            if (current.data === data) {
                return index;
            }
            current = current.next;
            index++;
        }
        return -1;
    }
    
    toString() {
        let current = this.head;
        let str = "";
        
        while (current) {
            str += current.data + " -> ";
            current = current.next;
        }
        str += "null";
        return str;
    }
}

// Binary Search Tree Implementation
class TreeNode {
    constructor(data) {
        this.data = data;
        this.left = null;
        this.right = null;
    }
}

class BinarySearchTree {
    constructor() {
        this.root = null;
    }
    
    insert(data) {
        let newNode = new TreeNode(data);
        
        if (this.root === null) {
            this.root = newNode;
        } else {
            this.insertNode(this.root, newNode);
        }
    }
    
    insertNode(node, newNode) {
        if (newNode.data < node.data) {
            if (node.left === null) {
                node.left = newNode;
            } else {
                this.insertNode(node.left, newNode);
            }
        } else {
            if (node.right === null) {
                node.right = newNode;
            } else {
                this.insertNode(node.right, newNode);
            }
        }
    }
    
    search(data) {
        return this.searchNode(this.root, data);
    }
    
    searchNode(node, data) {
        if (node === null) {
            return false;
        }
        
        if (data < node.data) {
            return this.searchNode(node.left, data);
        } else if (data > node.data) {
            return this.searchNode(node.right, data);
        } else {
            return true;
        }
    }
    
    inOrderTraversal(node = this.root, result = []) {
        if (node !== null) {
            this.inOrderTraversal(node.left, result);
            result.push(node.data);
            this.inOrderTraversal(node.right, result);
        }
        return result;
    }
}

// Hash Table Implementation
class HashTable {
    constructor(size = 53) {
        this.keyMap = new Array(size);
    }
    
    hash(key) {
        let total = 0;
        let WEIRD_PRIME = 31;
        
        for (let i = 0; i < Math.min(key.length, 100); i++) {
            let char = key[i];
            let value = char.charCodeAt(0) - 96;
            total = (total * WEIRD_PRIME + value) % this.keyMap.length;
        }
        return total;
    }
    
    set(key, value) {
        let index = this.hash(key);
        
        if (!this.keyMap[index]) {
            this.keyMap[index] = [];
        }
        
        this.keyMap[index].push([key, value]);
    }
    
    get(key) {
        let index = this.hash(key);
        
        if (this.keyMap[index]) {
            for (let i = 0; i < this.keyMap[index].length; i++) {
                if (this.keyMap[index][i][0] === key) {
                    return this.keyMap[index][i][1];
                }
            }
        }
        return undefined;
    }
    
    keys() {
        let keysArr = [];
        
        for (let i = 0; i < this.keyMap.length; i++) {
            if (this.keyMap[i]) {
                for (let j = 0; j < this.keyMap[i].length; j++) {
                    if (!keysArr.includes(this.keyMap[i][j][0])) {
                        keysArr.push(this.keyMap[i][j][0]);
                    }
                }
            }
        }
        return keysArr;
    }
    
    values() {
        let valuesArr = [];
        
        for (let i = 0; i < this.keyMap.length; i++) {
            if (this.keyMap[i]) {
                for (let j = 0; j < this.keyMap[i].length; j++) {
                    if (!valuesArr.includes(this.keyMap[i][j][1])) {
                        valuesArr.push(this.keyMap[i][j][1]);
                    }
                }
            }
        }
        return valuesArr;
    }
}

// Test Stack
console.log("1. Stack Implementation:");
let stack = new Stack();
stack.push(1);
stack.push(2);
stack.push(3);
console.log("Stack:", stack.toString());
console.log("Pop:", stack.pop());
console.log("Peek:", stack.peek());
console.log("Size:", stack.size());

// Test Queue
console.log("\n2. Queue Implementation:");
let queue = new Queue();
queue.enqueue("Alice");
queue.enqueue("Bob");
queue.enqueue("Charlie");
console.log("Queue:", queue.toString());
console.log("Dequeue:", queue.dequeue());
console.log("Front:", queue.front());
console.log("Size:", queue.size());

// Test Linked List
console.log("\n3. Linked List Implementation:");
let linkedList = new LinkedList();
linkedList.add(10);
linkedList.add(20);
linkedList.add(30);
linkedList.add(40);
console.log("Linked List:", linkedList.toString());
console.log("Find 20 at index:", linkedList.find(20));
linkedList.remove(20);
console.log("After removing 20:", linkedList.toString());

// Test Binary Search Tree
console.log("\n4. Binary Search Tree Implementation:");
let bst = new BinarySearchTree();
bst.insert(15);
bst.insert(10);
bst.insert(20);
bst.insert(8);
bst.insert(12);
bst.insert(18);
bst.insert(25);
console.log("In-order traversal:", bst.inOrderTraversal());
console.log("Search for 12:", bst.search(12));
console.log("Search for 99:", bst.search(99));

// Test Hash Table
console.log("\n5. Hash Table Implementation:");
let hashTable = new HashTable();
hashTable.set("name", "John");
hashTable.set("age", 30);
hashTable.set("city", "New York");
hashTable.set("country", "USA");
console.log("Get name:", hashTable.get("name"));
console.log("Get age:", hashTable.get("age"));
console.log("Keys:", hashTable.keys());
console.log("Values:", hashTable.values());

// Performance comparison
console.log("\n=== PERFORMANCE COMPARISON ===");

// Stack vs Array for push/pop operations
let array = [];
let stack2 = new Stack();

console.log("\nStack vs Array Performance:");
let start = Date.now();
for (let i = 0; i < 10000; i++) {
    array.push(i);
}
for (let i = 0; i < 10000; i++) {
    array.pop();
}
let arrayTime = Date.now() - start;

start = Date.now();
for (let i = 0; i < 10000; i++) {
    stack2.push(i);
}
for (let i = 0; i < 10000; i++) {
    stack2.pop();
}
let stackTime = Date.now() - start;

console.log("Array operations:", arrayTime, "ms");
console.log("Stack operations:", stackTime, "ms");
