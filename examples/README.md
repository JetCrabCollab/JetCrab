# JetCrab Examples

This directory contains practical examples demonstrating the capabilities of the JetCrab JavaScript engine. Each example includes both JavaScript code and a Rust program that uses the JetCrab API to interpret and execute the JavaScript.

## Examples Overview

### 1. Fibonacci (`fibonacci/`)
- **JavaScript File**: `fibonacci.js`
- **Rust File**: `src/main.rs`
- **Features Demonstrated**:
  - Recursive Fibonacci implementation
  - Iterative Fibonacci implementation
  - Memoized Fibonacci with performance optimization
  - Error handling and input validation
  - Performance comparison between different approaches

### 2. Sorting Algorithms (`sorting/`)
- **JavaScript File**: `sorting.js`
- **Rust File**: `src/main.rs`
- **Features Demonstrated**:
  - Bubble Sort
  - Selection Sort
  - Insertion Sort
  - Quick Sort
  - Merge Sort
  - Performance benchmarking
  - Array validation utilities

### 3. Data Structures (`data_structures/`)
- **JavaScript File**: `data_structures.js`
- **Rust File**: `src/main.rs`
- **Features Demonstrated**:
  - Stack implementation
  - Queue implementation
  - Linked List implementation
  - Binary Search Tree implementation
  - Hash Table implementation
  - Performance comparisons

## How to Run the Examples

### Prerequisites
- Rust installed on your system
- JetCrab engine built and available

### Running an Example

1. **Navigate to the example directory**:
   ```bash
   cd examples/fibonacci
   ```

2. **Run the example**:
   ```bash
   cargo run
   ```

### Example Output

Each example will:
1. Load and parse the JavaScript file
2. Execute the JavaScript code using JetCrab
3. Test individual functions and operations
4. Display performance metrics
5. Show execution results

## Example Structure

Each example follows this structure:
```
example_name/
├── Cargo.toml          # Rust dependencies and configuration
├── example_name.js     # JavaScript code to be interpreted
└── src/
    └── main.rs         # Rust program using JetCrab API
```

## JavaScript Features Demonstrated

The examples showcase JetCrab's support for:

- **Basic Syntax**: Variables, functions, loops, conditionals
- **Advanced Features**: Classes, arrow functions, template literals
- **Data Types**: Numbers, strings, booleans, arrays, objects
- **Control Flow**: If statements, loops, switch statements
- **Error Handling**: Try-catch blocks, input validation
- **Performance**: Benchmarking and optimization techniques

## API Usage

Each Rust program demonstrates how to:

1. **Create a JetCrab Engine**:
   ```rust
   use jetcrab::api::Engine;
   let mut engine = Engine::new();
   ```

2. **Execute JavaScript Code**:
   ```rust
   let result = engine.evaluate(&js_code)?;
   ```

3. **Handle Results and Errors**:
   ```rust
   match engine.evaluate(code) {
       Ok(result) => println!("✅ Result: {:?}", result),
       Err(error) => println!("❌ Error: {}", error),
   }
   ```

## Performance Considerations

- **Fibonacci**: Demonstrates recursive vs iterative performance
- **Sorting**: Shows algorithm complexity differences
- **Data Structures**: Compares different implementation approaches

## Troubleshooting

### Common Issues

1. **File Not Found**: Ensure you're running from the correct directory
2. **Compilation Errors**: Check that JetCrab is properly built
3. **Runtime Errors**: Verify JavaScript syntax is supported by JetCrab

### Debugging

- Check console output for detailed error messages
- Verify file paths in the Rust code
- Ensure JavaScript code follows supported syntax

## Contributing

To add new examples:

1. Create a new directory in `examples/`
2. Add JavaScript file with your code
3. Create Rust file using JetCrab API
4. Add appropriate Cargo.toml
5. Test thoroughly
6. Update this README

## Next Steps

After running these examples, you can:

- Modify the JavaScript code to test different scenarios
- Add new algorithms or data structures
- Experiment with JetCrab's API features
- Create your own examples
- Contribute improvements to the JetCrab engine
