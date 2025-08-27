# JetCrab: A Complete Guide for Beginners

## What is JetCrab?

JetCrab is a **JavaScript engine** written in Rust. Think of it as a program that can read, understand, and execute JavaScript code - just like how your web browser runs JavaScript, but as a standalone application.

## Why is this important?

JavaScript engines are fundamental to modern computing. They power:
- Web browsers (Chrome, Firefox, Safari)
- Node.js applications
- Mobile apps
- Server-side applications
- IoT devices

Understanding how they work gives you insight into the foundation of modern software development.

## Computer Science Theory Behind JetCrab

### 1. Compiler Theory

A JavaScript engine is essentially a **compiler** and **interpreter** combined. Here's the theory:

#### What is a Compiler?
A compiler is a program that translates code from one language to another. In JetCrab's case:
- **Input**: JavaScript source code (text)
- **Output**: Bytecode (machine-readable instructions)

#### The Compilation Pipeline
JetCrab follows the classic compiler pipeline:

```mermaid
graph LR
    A[Source Code] --> B[Lexical Analysis]
    B --> C[Syntax Analysis]
    C --> D[Semantic Analysis]
    D --> E[Code Generation]
    E --> F[Execution]
    
    B --> G[Tokens]
    C --> H[AST]
    D --> I[Validated AST]
    E --> J[Bytecode]
    F --> K[Runtime Output]
    
    style A fill:#e1f5fe
    style K fill:#c8e6c9
    style G fill:#fff3e0
    style H fill:#fff3e0
    style I fill:#fff3e0
    style J fill:#fff3e0
```

### 2. Lexical Analysis (Tokenization)

**Theory**: Lexical analysis breaks source code into meaningful units called "tokens."

**Real-world analogy**: Think of reading a sentence. You don't read it letter by letter - you read it word by word. Each word is a "token."

**In JetCrab**:
```javascript
let x = 5 + 3;
```
Becomes these tokens:
- `let` (keyword)
- `x` (identifier)
- `=` (operator)
- `5` (number literal)
- `+` (operator)
- `3` (number literal)
- `;` (punctuation)

**Computer Science Concept**: This is a **finite state machine** that recognizes patterns in text.

### 3. Syntax Analysis (Parsing)

**Theory**: Syntax analysis determines the structure and meaning of the program.

**Real-world analogy**: Just like grammar rules determine how words form sentences, parsing determines how tokens form valid JavaScript code.

**In JetCrab**:
The tokens are organized into an **Abstract Syntax Tree (AST)**:

```mermaid
graph TD
    A[ExpressionStatement] --> B[AssignmentExpression]
    B --> C[Identifier: x]
    B --> D[AssignmentOperator: =]
    B --> E[BinaryExpression]
    E --> F[NumberLiteral: 5]
    E --> G[AdditionOperator: +]
    E --> H[NumberLiteral: 3]
    
    style A fill:#e3f2fd
    style B fill:#e8f5e8
    style C fill:#fff3e0
    style D fill:#fff3e0
    style E fill:#e8f5e8
    style F fill:#fff3e0
    style G fill:#fff3e0
    style H fill:#fff3e0
```

**Computer Science Concept**: This uses **context-free grammar** and **recursive descent parsing**.

### 4. Semantic Analysis

**Theory**: Semantic analysis checks if the program makes logical sense.

**Examples**:
- Are variables declared before use?
- Are function calls valid?
- Are types compatible?

**Computer Science Concept**: This involves **symbol tables** and **scope analysis**.

### 5. Code Generation

**Theory**: Code generation converts the AST into executable instructions.

**In JetCrab**: The AST is converted to **bytecode** - a low-level instruction set that the virtual machine can execute.

**Computer Science Concept**: This is **code generation** and **optimization**.

### 6. Virtual Machine Execution

**Theory**: A virtual machine is a program that simulates a computer.

**In JetCrab**: The VM reads bytecode instructions and executes them step by step.

**Computer Science Concept**: This is **interpretation** and **stack-based execution**.

## JetCrab Architecture Deep Dive

```mermaid
graph TB
    subgraph "JetCrab Architecture"
        A[Source Code] --> B[Lexer Module]
        B --> C[Parser Module]
        C --> D[AST Module]
        D --> E[Semantic Module]
        E --> F[Bytecode Module]
        F --> G[VM Module]
        G --> H[Runtime Module]
        
        I[Memory Module] -.-> G
        I -.-> H
        
        B --> J[Tokens]
        C --> K[Abstract Syntax Tree]
        E --> L[Validated AST]
        F --> M[Bytecode Instructions]
        G --> N[Execution Results]
    end
    
    style A fill:#e1f5fe
    style N fill:#c8e6c9
    style J fill:#fff3e0
    style K fill:#fff3e0
    style L fill:#fff3e0
    style M fill:#fff3e0
```

### 1. Lexer Module (`src/lexer/`)

**Purpose**: Converts JavaScript source code into tokens.

**Key Components**:
- **Token Recognition**: Identifies keywords, operators, literals
- **Position Tracking**: Keeps track of line/column numbers for error reporting
- **Error Recovery**: Continues parsing even when encountering errors

**Theory**: Uses **regular expressions** and **finite automata** to recognize token patterns.

### 2. Parser Module (`src/parser/`)

**Purpose**: Builds an Abstract Syntax Tree from tokens.

**Key Components**:
- **Expression Parsing**: Handles mathematical expressions, function calls
- **Statement Parsing**: Handles if statements, loops, variable declarations
- **Error Recovery**: Attempts to continue parsing after syntax errors

**Theory**: Implements **recursive descent parsing** with **operator precedence**.

### 3. AST Module (`src/ast/`)

**Purpose**: Represents the program structure in memory.

**Key Components**:
- **Node Types**: Different types for expressions, statements, declarations
- **Visitor Pattern**: Allows traversing the tree structure
- **Serialization**: Converts AST to JSON for debugging

**Theory**: Uses **tree data structures** and **design patterns**.

### 4. Semantic Module (`src/semantic/`)

**Purpose**: Validates program logic and meaning.

**Key Components**:
- **Scope Analysis**: Tracks variable declarations and usage
- **Type Checking**: Ensures operations are valid
- **Error Detection**: Finds logical errors in the code

**Theory**: Implements **symbol tables** and **type systems**.

### 5. Bytecode Module (`src/bytecode/`)

**Purpose**: Generates executable instructions from AST.

**Key Components**:
- **Instruction Set**: Low-level operations the VM can execute
- **Optimization**: Improves code efficiency
- **Constant Pool**: Stores repeated values efficiently

**Theory**: Uses **code generation** and **optimization techniques**.

### 6. VM Module (`src/vm/`)

**Purpose**: Executes bytecode instructions.

**Key Components**:
- **Stack Management**: Handles function calls and local variables
- **Register System**: Stores temporary values
- **Instruction Execution**: Processes bytecode one instruction at a time

**Theory**: Implements **stack-based virtual machines** and **register allocation**.

### 7. Runtime Module (`src/runtime/`)

**Purpose**: Provides JavaScript runtime environment.

**Key Components**:
- **Value System**: Handles different data types (numbers, strings, objects)
- **Object System**: Manages JavaScript objects and their properties
- **Built-in Functions**: Provides standard JavaScript functions

**Theory**: Implements **object-oriented programming** and **dynamic typing**.

### 8. Memory Module (`src/memory/`)

**Purpose**: Manages memory allocation and garbage collection.

**Key Components**:
- **Heap Management**: Allocates and frees memory
- **Garbage Collection**: Automatically cleans up unused memory
- **Memory Safety**: Prevents memory leaks and corruption

**Theory**: Uses **garbage collection algorithms** and **memory management**.

## How JetCrab Processes JavaScript Code

### Step-by-Step Example

Let's trace how JetCrab processes this simple JavaScript code:

```javascript
let x = 10;
let y = 20;
console.log(x + y);
```

#### Step 1: Lexical Analysis
The lexer converts this into tokens:
```
[let, x, =, 10, ;, let, y, =, 20, ;, console, ., log, (, x, +, y, ), ;]
```

#### Step 2: Syntax Analysis
The parser builds an AST:

```mermaid
graph TD
    A[Program] --> B[VariableDeclaration: let x = 10]
    A --> C[VariableDeclaration: let y = 20]
    A --> D[ExpressionStatement]
    D --> E[CallExpression]
    E --> F[MemberExpression: console.log]
    E --> G[Arguments]
    G --> H[BinaryExpression: x + y]
    H --> I[Identifier: x]
    H --> J[AdditionOperator: +]
    H --> K[Identifier: y]
    
    style A fill:#e3f2fd
    style B fill:#e8f5e8
    style C fill:#e8f5e8
    style D fill:#e8f5e8
    style E fill:#fff3e0
    style F fill:#fff3e0
    style G fill:#fff3e0
    style H fill:#fff3e0
    style I fill:#fff3e0
    style J fill:#fff3e0
    style K fill:#fff3e0
```

#### Step 3: Semantic Analysis
The semantic analyzer:
- Checks that `x` and `y` are declared before use
- Validates that `console.log` is a valid function
- Ensures the addition operation is valid

#### Step 4: Bytecode Generation
The bytecode generator creates instructions:

```mermaid
graph LR
    subgraph "Bytecode Instructions"
        A[LOAD_CONSTANT 10] --> B[STORE_VARIABLE x]
        B --> C[LOAD_CONSTANT 20]
        C --> D[STORE_VARIABLE y]
        D --> E[LOAD_VARIABLE x]
        E --> F[LOAD_VARIABLE y]
        F --> G[ADD]
        G --> H[CALL_FUNCTION console.log]
    end
    
    style A fill:#e8f5e8
    style B fill:#e8f5e8
    style C fill:#e8f5e8
    style D fill:#e8f5e8
    style E fill:#fff3e0
    style F fill:#fff3e0
    style G fill:#fff3e0
    style H fill:#fff3e0
```

#### Step 5: Virtual Machine Execution
The VM executes each instruction:

```mermaid
graph TD
    subgraph "VM Execution Steps"
        A[1. Load 10 into memory] --> B[2. Store as variable x]
        B --> C[3. Load 20 into memory]
        C --> D[4. Store as variable y]
        D --> E[5. Load value of x (10)]
        E --> F[6. Load value of y (20)]
        F --> G[7. Add them together (30)]
        G --> H[8. Call console.log with result 30]
    end
    
    subgraph "Memory State"
        I[Variable x = 10]
        J[Variable y = 20]
        K[Stack: 30]
    end
    
    style A fill:#e8f5e8
    style B fill:#e8f5e8
    style C fill:#e8f5e8
    style D fill:#e8f5e8
    style E fill:#fff3e0
    style F fill:#fff3e0
    style G fill:#fff3e0
    style H fill:#fff3e0
    style I fill:#e1f5fe
    style J fill:#e1f5fe
    style K fill:#e1f5fe
```

#### Step 6: Runtime Output
The runtime environment outputs: `30`

## Key Computer Science Concepts in JetCrab

### 1. Data Structures

```mermaid
graph TB
    subgraph "Data Structures in JetCrab"
        A[Trees<br/>AST Representation] --> A1[Binary Trees]
        A --> A2[Expression Trees]
        
        B[Stacks<br/>VM Execution Stack] --> B1[Function Call Stack]
        B --> B2[Operand Stack]
        
        C[Hash Tables<br/>Symbol Tables] --> C1[Variable Lookup]
        C --> C2[Object Properties]
        
        D[Arrays<br/>Token Streams] --> D1[Token Lists]
        D --> D2[Instruction Lists]
    end
    
    style A fill:#e3f2fd
    style B fill:#e8f5e8
    style C fill:#fff3e0
    style D fill:#fce4ec
```

### 2. Algorithms

```mermaid
graph LR
    subgraph "Algorithms in JetCrab"
        A[Parsing<br/>Recursive Descent] --> A1[Operator Precedence]
        A --> A2[Error Recovery]
        
        B[Memory Management<br/>Garbage Collection] --> B1[Mark & Sweep]
        B --> B2[Memory Allocation]
        
        C[Optimization<br/>Code Generation] --> C1[Constant Folding]
        C --> C2[Dead Code Elimination]
        
        D[Search<br/>Symbol Resolution] --> D1[Scope Lookup]
        D --> D2[Variable Resolution]
    end
    
    style A fill:#e3f2fd
    style B fill:#e8f5e8
    style C fill:#fff3e0
    style D fill:#fce4ec
```

### 3. Theory of Computation

```mermaid
graph TB
    subgraph "Computational Theory in JetCrab"
        A[Regular Languages<br/>Token Recognition] --> A1[Finite State Machines]
        A --> A2[Regular Expressions]
        
        B[Context-Free Languages<br/>Syntax Parsing] --> B1[Context-Free Grammars]
        B --> B2[Recursive Descent]
        
        C[Turing Machines<br/>VM Execution] --> C1[Universal Computation]
        C --> C2[Program Execution]
        
        D[Automata<br/>Lexical Analysis] --> D1[State Transitions]
        D --> D2[Pattern Recognition]
    end
    
    style A fill:#e3f2fd
    style B fill:#e8f5e8
    style C fill:#fff3e0
    style D fill:#fce4ec
```

### 4. Programming Language Theory

```mermaid
graph LR
    subgraph "Language Theory in JetCrab"
        A[Type Systems<br/>Dynamic Typing] --> A1[Type Checking]
        A --> A2[Type Coercion]
        
        B[Scope<br/>Lexical Scoping] --> B1[Variable Scope]
        B --> B2[Closures]
        
        C[Evaluation<br/>Call by Value] --> C1[Parameter Passing]
        C --> C2[Lazy Evaluation]
        
        D[Memory Models<br/>Heap vs Stack] --> D1[Memory Allocation]
        D --> D2[Garbage Collection]
    end
    
    style A fill:#e3f2fd
    style B fill:#e8f5e8
    style C fill:#fff3e0
    style D fill:#fce4ec
```

## Performance Considerations

### Why Rust?

```mermaid
graph TB
    subgraph "Rust Benefits for JetCrab"
        A[Memory Safety] --> A1[No Buffer Overflows]
        A --> A2[No Null Pointer Errors]
        
        B[Performance] --> B1[Near C/C++ Speed]
        B --> B2[Modern Abstractions]
        
        C[Concurrency] --> C1[Safe Multi-threading]
        C --> C2[Data Race Prevention]
        
        D[Zero-Cost Abstractions] --> D1[High-level Features]
        D --> D2[No Runtime Overhead]
    end
    
    style A fill:#e3f2fd
    style B fill:#e8f5e8
    style C fill:#fff3e0
    style D fill:#fce4ec
```

### Optimization Techniques

```mermaid
graph LR
    subgraph "Optimization Strategies"
        A[Bytecode Optimization<br/>Reduce Instructions] --> A1[Constant Folding]
        A --> A2[Dead Code Elimination]
        
        B[Memory Pooling<br/>Reuse Allocations] --> B1[Object Pooling]
        B --> B2[Memory Reuse]
        
        C[Lazy Evaluation<br/>Compute on Demand] --> C1[Delayed Computation]
        C --> C2[Conditional Execution]
        
        D[Caching<br/>Store Frequent Data] --> D1[Result Caching]
        D --> D2[Lookup Tables]
    end
    
    style A fill:#e3f2fd
    style B fill:#e8f5e8
    style C fill:#fff3e0
    style D fill:#fce4ec
```

## Learning Path for Beginners

### 1. Start with the Basics

```mermaid
graph TD
    subgraph "Learning Foundation"
        A[Learn Rust Fundamentals] --> A1[Ownership & Borrowing]
        A --> A2[Memory Management]
        
        B[Data Structures] --> B1[Arrays & Linked Lists]
        B --> B2[Trees & Graphs]
        
        C[Algorithms] --> C1[Searching & Sorting]
        C --> C2[Parsing Algorithms]
    end
    
    style A fill:#e3f2fd
    style B fill:#e8f5e8
    style C fill:#fff3e0
```

### 2. Study Compiler Theory

```mermaid
graph LR
    subgraph "Compiler Education"
        A[Dragon Book<br/>Compilers: Principles] --> A1[Lexical Analysis]
        A --> A2[Syntax Analysis]
        
        B[Code Generation] --> B1[Optimization]
        B --> B2[Target Code]
        
        C[Language Design] --> C1[Grammar Design]
        C --> C2[Semantic Analysis]
    end
    
    style A fill:#e3f2fd
    style B fill:#e8f5e8
    style C fill:#fff3e0
```

### 3. Explore JetCrab Code

```mermaid
graph TB
    subgraph "Hands-on Learning"
        A[Start with Examples] --> A1[basic_usage.rs]
        A --> A2[vm_demo.rs]
        
        B[Read Implementation] --> B1[Lexer Module]
        B --> B2[Parser Module]
        
        C[Trace Execution] --> C1[Token Generation]
        C --> C2[AST Construction]
        
        D[Experiment] --> D1[Modify Examples]
        D --> D2[Add Features]
    end
    
    style A fill:#e3f2fd
    style B fill:#e8f5e8
    style C fill:#fff3e0
    style D fill:#fce4ec
```

### 4. Build Your Own

```mermaid
graph LR
    subgraph "Project Progression"
        A[Simple Calculator] --> B[Basic Parser]
        B --> C[Small VM]
        C --> D[Add JetCrab Features]
        
        A1[Arithmetic Operations]
        B1[Expression Parsing]
        C1[Bytecode Execution]
        D1[Language Extensions]
    end
    
    style A fill:#e3f2fd
    style B fill:#e8f5e8
    style C fill:#fff3e0
    style D fill:#fce4ec
```

## Common Questions

### Q: Why not use an existing JavaScript engine?
A: Building from scratch teaches fundamental concepts and allows customization for specific use cases.

### Q: How does this compare to V8 or SpiderMonkey?
A: JetCrab is educational and experimental, while V8/SpiderMonkey are production engines with years of optimization.

### Q: Can I use JetCrab in production?
A: Currently, it's designed for learning and experimentation. Production use would require more testing and optimization.

### Q: How do I contribute to JetCrab?
A: Start by reading the code, running tests, fixing bugs, and adding small features. Check the CONTRIBUTING.md file.

## Conclusion

JetCrab is more than just a JavaScript engine - it's a comprehensive learning tool that demonstrates fundamental computer science concepts in practice. By understanding how JetCrab works, you'll gain deep insights into:

- How programming languages are designed
- How compilers and interpreters work
- How virtual machines execute code
- How memory management affects performance
- How to build large, complex software systems

This knowledge is valuable whether you're building web applications, system software, or learning about computer science fundamentals.

Remember: Every complex system is built from simple components. JetCrab shows how these components work together to create something powerful and useful. 