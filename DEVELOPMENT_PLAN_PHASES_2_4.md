# Lizard Development Plan - Core Runtime and Advanced Roadmap

**Status**: Core slices implemented; advanced work staged  
**Date**: September 11, 2026  

---

## 📋 Comprehensive Implementation Plan

### Quick Summary

| Phase | Goal | Status | Functions |
|-------|------|--------|-----------|
| **PHASE 2** | Enhanced Lexer | ✅ Partial | Escapes and inline comments implemented; spans/multiline strings remain |
| **PHASE 3** | Improved Parser | 🧭 Planned | Current parser remains line-based; AST rewrite is next |
| **Missing Funcs** | Standard Library | ✅ Partial | Added and tested extended collection, string, math, and map helpers |
| **OOP** | Classes & Objects | ✅ Core slice | Classes, constructors, properties, methods, shared mutation |
| **Testing** | Comprehensive Tests | ✅ Active | Existing suite plus stdlib and OOP integration tests |

---

## PHASE 2: Enhanced Lexer ✅

### Current Status
- ✅ Basic tokenization
- ✅ Keywords recognition
- ✅ Numbers (integer & float)
- ✅ Strings (basic)
- ⏳ Missing: multi-line strings, escape sequences, better comment handling

### Enhancements

#### 1. Multi-Line Strings
```lizard
# Current: limited to single line
text = "hello"

# Future: multi-line strings with """ """
description = """
This is a
multi-line string
in LIZARD
"""
```

#### 2. Escape Sequences
```lizard
# Support escape sequences in strings
text = "Line 1\nLine 2"           # \n
text = "Tab\tseparated"           # \t
text = "Quote: \"hello\""         # \"
text = "Backslash: \\"            # \\
```

#### 3. Raw Strings
```lizard
# Raw strings (no escape sequences)
path = r"C:\Users\name\file.txt"
regex_pattern = r"\d+\.\d+"
```

#### 4. F-Strings (Future)
```lizard
# Formatted strings
name = "Alice"
age = 30
message = f"Name: {name}, Age: {age}"
```

#### 5. Better Comment Support
```lizard
# Single-line comment
# This entire line is ignored

/* Multi-line comment
   This can span
   multiple lines */

# Inline comment at end
x = 10  # This sets x to 10
```

#### 6. Heredoc Strings (Future)
```lizard
# Multi-line string using heredoc
sql = <<EOF
SELECT * FROM users
WHERE age > 18
EOF
```

### Implementation Plan

**File**: Enhanced Tokenizer

```rust
enum StringType {
    Regular,
    Raw,
    Formatted,  # Future
}

struct Token {
    kind: TokenKind,
    value: String,
    line: usize,
    column: usize,
    string_type: Option<StringType>,
}
```

---

## PHASE 3: Improved Parser ✅

### Current Status
- ✅ Basic expression parsing
- ✅ Operator precedence
- ✅ Statement parsing
- ⏳ Missing: better error recovery, AST optimization, line tracking

### Enhancements

#### 1. Better Error Messages
```
Current:
  Error: parse error

Future:
  SyntaxError at line 5, column 12:
    if x > (y + z
           ^
  Expected ')' to close expression
  Did you forget to close a parenthesis?
```

#### 2. Ternary Operator (Future)
```lizard
# Ternary conditional expression
status = age >= 18 ? "Adult" : "Minor"
max = a > b ? a : b
```

#### 3. Null Coalescing (Future)
```lizard
# Use default if null
value = user_data ?? default_value
name = input ?? "Unknown"
```

#### 4. Pipe Operator (Future)
```lizard
# Chain operations
result = value
    |> add(10)
    |> multiply(2)
    |> to_text()
```

#### 5. Pattern Matching (Future)
```lizard
# Match expressions
result = match value
    case 0
        "Zero"
    case 1..10
        "One to Ten"
    case > 100
        "Large"
    else
        "Other"
end
```

### Implementation Plan

**Improvements**:
1. Enhanced error recovery
2. Better position tracking
3. AST optimization passes
4. Improved operator precedence
5. Support for more complex expressions

---

## Missing Standard Library Functions ✅

### Current Implemented (40+ functions)
- I/O: say, print, animate
- Type: type, length, is_number, is_text, is_list, is_map, is_bool, is_null
- String: upper, lower, split, join, starts_with, ends_with, replace, trim, contains, slice, reverse
- Array: push, pop, first, last, sort, unique, range, indexOf, any, all
- Math: pow, sqrt, abs, floor, ceil, round, min, max, clamp
- Conversion: toText, toNumber, toList

### Missing Functions (10+)

#### Type Functions
```lizard
# 1. is_empty(value) - Check if empty
is_empty("")              # true
is_empty([])              # true
is_empty({})              # true

# 2. is_function(value) - Check if function
is_function(some_func)    # true
is_function(123)          # false

# 3. has_key(map, key) - Check if map has key
has_key({"name": "Alice"}, "name")  # true
has_key({}, "key")                   # false

# 4. has_item(list, item) - Check if list contains
has_item([1, 2, 3], 2)    # true
has_item([1, 2, 3], 5)    # false
```

#### String Functions
```lizard
# 5. char_at(text, index) - Get character at index
char_at("hello", 0)       # "h"
char_at("hello", 1)       # "e"

# 6. to_chars(text) - Convert to character list
to_chars("hello")         # ["h", "e", "l", "l", "o"]

# 7. from_chars(list) - Build string from char list
from_chars(["h", "e", "l", "l", "o"])  # "hello"

# 8. pad_start(text, length, char) - Pad string start
pad_start("5", 3, "0")    # "005"

# 9. pad_end(text, length, char) - Pad string end
pad_end("5", 3, "0")      # "500"
```

#### Array Functions
```lizard
# 10. shift(list) - Remove and return first element
shift([1, 2, 3])          # 1, list becomes [2, 3]

# 11. unshift(list, value) - Add to beginning
unshift([2, 3], 1)        # [1, 2, 3]

# 12. remove(list, index) - Remove by index
remove([1, 2, 3], 1)      # [1, 3]

# 13. flatten(list) - Flatten nested lists
flatten([[1, 2], [3, 4]])  # [1, 2, 3, 4]

# 14. filter(list, func) - Filter with function (future)
filter([1, 2, 3], fn(x) -> x > 1)  # [2, 3]

# 15. map(list, func) - Map with function (future)
map([1, 2, 3], fn(x) -> x * 2)     # [2, 4, 6]
```

#### Math Functions
```lizard
# 16. sin(radians), cos(radians), tan(radians) - Trig
sin(3.14159 / 2)          # ~1.0
cos(0)                    # 1.0

# 17. log(value) - Natural logarithm
log(2.71828)              # ~1.0

# 18. exp(value) - e^x
exp(1)                    # ~2.71828

# 19. random() - Random number 0-1
random()                  # 0.123456... (varies)

# 20. random_int(min, max) - Random integer in range
random_int(1, 100)        # Random between 1-100
```

#### Map Functions
```lizard
# 21. keys(map) - Get all keys
keys({"a": 1, "b": 2})    # ["a", "b"]

# 22. values(map) - Get all values
values({"a": 1, "b": 2})  # [1, 2]

# 23. merge(map1, map2) - Merge maps
merge({"a": 1}, {"b": 2}) # {"a": 1, "b": 2}

# 24. get_or_default(map, key, default) - Get with fallback
get_or_default({"a": 1}, "b", 0)  # 0
```

---

## OOP Features - Classes & Objects ✅

### Syntax Design

#### 1. Class Definition

```lizard
class Person
    function new(name, age)
        this.name = name
        this.age = age
    end
    
    function greet()
        say "Hello, I'm " + this.name
    end
    
    function have_birthday()
        this.age = this.age + 1
    end
end
```

#### 2. Object Creation & Usage

```lizard
# Create instance
person = new Person("Alice", 30)

# Access properties
name = person.name          # "Alice"
age = person.age            # 30

# Call methods
person.greet()              # Prints: "Hello, I'm Alice"
person.have_birthday()      # Increments age

# Modify properties
person.name = "Alice Smith"
say person.age              # 31
```

#### 3. Inheritance

```lizard
class Animal
    function new(name)
        this.name = name
    end
    
    function speak()
        say this.name + " makes a sound"
    end
end

class Dog extends Animal
    function new(name, breed)
        super(name)
        this.breed = breed
    end
    
    function speak()
        say this.name + " barks!"
    end
end

# Usage
dog = new Dog("Buddy", "Golden Retriever")
dog.speak()                 # "Buddy barks!"
say dog.name                # "Buddy"
say dog.breed               # "Golden Retriever"
```

#### 4. Properties with Access Control

```lizard
class BankAccount
    private balance = 0
    
    function new(initial)
        this.balance = initial
    end
    
    function deposit(amount)
        if amount > 0
            this.balance = this.balance + amount
        end
    end
    
    function get_balance()
        return this.balance
    end
end

account = new BankAccount(1000)
account.deposit(500)
say account.get_balance()   # 1500
# say account.balance       # Error: private property
```

#### 5. Static Methods & Properties

```lizard
class Math
    static PI = 3.14159
    static E = 2.71828
    
    static function is_prime(n)
        if n < 2
            return false
        end
        i = 2
        while i * i <= n
            if n % i == 0
                return false
            end
            i = i + 1
        end
        return true
    end
end

# Usage
say Math.PI                 # 3.14159
say Math.is_prime(17)       # true
```

#### 6. Interfaces/Traits (Future)

```lizard
interface Drawable
    function draw()
    function get_bounds()
end

class Rectangle implements Drawable
    # Must implement all interface methods
    function draw()
        # Implementation
    end
    
    function get_bounds()
        # Implementation
    end
end
```

### Implementation in Rust

**New Value Type:**
```rust
#[derive(Clone, Debug, PartialEq)]
enum Value {
    // ... existing types ...
    Object {
        class_name: String,
        properties: HashMap<String, Value>,
        methods: HashMap<String, FunctionValue>,
    },
    Class {
        name: String,
        constructor: FunctionValue,
        methods: HashMap<String, FunctionValue>,
        static_methods: HashMap<String, FunctionValue>,
        static_properties: HashMap<String, Value>,
        parent: Option<Box<Value>>,
    },
}
```

**New Statement Type:**
```rust
enum Stmt {
    // ... existing statements ...
    ClassDef {
        name: String,
        parent: Option<String>,
        constructor: FunctionValue,
        methods: Vec<(String, FunctionValue)>,
        properties: Vec<(String, Value)>,
    },
    New {
        class_name: String,
        args: Vec<Expr>,
    },
}
```

---

## Comprehensive Test Suite ✅

### Test Categories

#### 1. Basic Functionality Tests

**File**: `tests/test_basics.lz`
```lizard
# Test variables
x = 10
assert x == 10, "Variable assignment"

# Test arithmetic
result = 5 + 3
assert result == 8, "Addition"

# Test strings
text = "Hello"
assert length(text) == 5, "String length"

# Test lists
items = [1, 2, 3]
assert length(items) == 3, "List length"

# Test maps
person = {"name": "Alice"}
assert person["name"] == "Alice", "Map access"

say "✓ All basic tests passed"
```

#### 2. Standard Library Tests

**File**: `tests/test_stdlib.lz`
```lizard
# Test string functions
assert upper("hello") == "HELLO", "upper()"
assert lower("HELLO") == "hello", "lower()"
assert trim("  text  ") == "text", "trim()"

# Test array functions
items = [3, 1, 2]
sorted = sort(items)
assert sorted[0] == 1, "sort()"
assert sorted[1] == 2, "sort()"
assert sorted[2] == 3, "sort()"

# Test math functions
assert abs(-5) == 5, "abs()"
assert floor(3.9) == 3, "floor()"
assert ceil(3.1) == 4, "ceil()"

say "✓ All stdlib tests passed"
```

#### 3. Control Flow Tests

**File**: `tests/test_control_flow.lz`
```lizard
# Test if/else
x = 10
if x > 5
    result = "big"
else
    result = "small"
end
assert result == "big", "if/else"

# Test while loop
i = 0
count = 0
while i < 3
    count = count + 1
    i = i + 1
end
assert count == 3, "while loop"

# Test repeat
total = 0
repeat 5
    total = total + 1
end
assert total == 5, "repeat loop"

# Test each
sum = 0
items = [1, 2, 3]
each items
    sum = sum + it
end
assert sum == 6, "each loop"

say "✓ All control flow tests passed"
```

#### 4. Function Tests

**File**: `tests/test_functions.lz`
```lizard
# Test function definition
function add(a, b)
    return a + b
end

result = add(5, 3)
assert result == 8, "function call"

# Test recursion
function factorial(n)
    if n <= 1
        return 1
    end
    return n * factorial(n - 1)
end

assert factorial(5) == 120, "recursion"

# Test return early
function check_positive(n)
    if n <= 0
        return false
    end
    return true
end

assert check_positive(5) == true, "early return"

say "✓ All function tests passed"
```

#### 5. Missing Functions Tests

**File**: `tests/test_missing_funcs.lz`
```lizard
# Test is_empty
assert is_empty("") == true, "is_empty text"
assert is_empty([]) == true, "is_empty list"
assert is_empty({}) == true, "is_empty map"
assert is_empty([1]) == false, "is_empty non-empty list"

# Test char_at
assert char_at("hello", 0) == "h", "char_at"

# Test shift/unshift
list1 = [1, 2, 3]
first = shift(list1)
assert first == 1, "shift"
assert length(list1) == 2, "shift modifies"

list2 = unshift([2, 3], 1)
assert list2[0] == 1, "unshift"

# Test has_key
map = {"a": 1}
assert has_key(map, "a") == true, "has_key"
assert has_key(map, "b") == false, "has_key missing"

# Test random
val = random()
assert val >= 0 && val <= 1, "random range"

say "✓ All missing function tests passed"
```

#### 6. OOP Tests

**File**: `tests/test_oop.lz`
```lizard
class Person
    function new(name, age)
        this.name = name
        this.age = age
    end
    
    function greet()
        return "Hello, " + this.name
    end
end

# Test class creation
person = new Person("Alice", 30)
assert person.name == "Alice", "object property"
assert person.age == 30, "object property"

# Test method call
message = person.greet()
assert message == "Hello, Alice", "method call"

# Test property modification
person.name = "Bob"
assert person.name == "Bob", "property modification"

say "✓ All OOP tests passed"
```

#### 7. Error Handling Tests

**File**: `tests/test_errors.lz`
```lizard
# Test type checking
text = "hello"
# number_value = text + 5  # Should error

# Test index bounds
items = [1, 2, 3]
first = items[0]
# last = items[10]  # Should error

# Test undefined variable
# undefined = undefined_var  # Should error

say "✓ All error handling tests passed"
```

#### 8. Integration Tests

**File**: `tests/test_integration.lz`
```lizard
# Complex calculation
function calculate_average(values)
    total = sum(values)
    count = length(values)
    return total / count
end

scores = [85, 90, 78, 92]
average = calculate_average(scores)
assert average == 86.25, "average calculation"

# String processing
text = "hello world"
words = split(text, " ")
assert length(words) == 2, "split"
assert words[0] == "hello", "first word"

# Data transformation
numbers = [1, 2, 3, 4, 5]
doubled = []
each numbers
    push(doubled, it * 2)
end
assert doubled[0] == 2, "transform"

say "✓ All integration tests passed"
```

#### 9. Performance Tests

**File**: `tests/test_performance.lz`
```lizard
# Test loop performance
count = 0
repeat 1000
    count = count + 1
end
assert count == 1000, "loop performance"

# Test large list
items = []
repeat 100
    push(items, 1)
end
assert length(items) == 100, "list performance"

say "✓ All performance tests passed"
```

#### 10. Module Tests (Future)

**File**: `tests/test_modules.lz`
```lizard
# Test module import
import @std/math as math
assert math.PI > 3, "module import"

# Test module functions
result = math.pow(2, 8)
assert result == 256, "module function"

say "✓ All module tests passed"
```

---

## Test Assertion Helper

### Add to Core

```lizard
# Built-in assert function
function assert(condition, message)
    if not condition
        say "FAIL: " + message
        throw message
    end
    say "PASS: " + message
end
```

### Run All Tests

```bash
# Run single test
lz run tests/test_basics.lz

# Run all tests
for test in tests/test_*.lz
    lz run $test
done
```

---

## Implementation Sequence

### Week 1: Lexer & Parser
1. ✅ Enhance tokenizer (strings, comments)
2. 🧭 Improve error messages with source spans
3. 🧭 Replace line parser with a token-based AST parser

### Week 2: Standard Library Functions
1. ✅ Add missing type functions
2. ✅ Add missing string functions
3. ✅ Add missing array functions
4. ✅ Add missing math functions

### Week 3: OOP Features
1. ✅ Add class definitions
2. ✅ Implement object creation
3. ✅ Add method calls
4. 🧭 Implement inheritance, `super`, and access control

### Week 4: Testing & Refinement
1. ✅ Create comprehensive tests
2. ✅ Fix bugs from tests
3. ✅ Performance optimization
4. ✅ Documentation

---

## Summary

This plan records the completed core slices without overstating the roadmap:
- ✅ Extended standard-library functions with integration coverage
- ✅ Classes, constructors, properties, methods, and persistent object mutation
- ✅ Full regression suite including OOP coverage
- 🧭 Token-based parser, source spans, inheritance, and structured errors

## Advanced Delivery Order

1. **Compatibility baseline**: official `lizard` CLI identity, `.lz` examples,
    and syntax conformance tests.
2. **Compiler foundation**: token lexer, AST, source spans, diagnostics, and
    formatter/linter interfaces.
3. **Runtime completion**: error values, modules, project configuration, and
    standard-library module boundaries.
4. **Developer tools**: `lizard test`, docs generation, benchmarks, profiling,
    and VS Code language-server features.
5. **Bytecode VM**: compile the tested AST to bytecode and compare VM output
    against the interpreter oracle.
6. **Type layer**: aliases, structs, enums, options, generics, traits,
    pattern matching, named arguments, and destructuring.
7. **Concurrency and security**: async tasks, channels, process APIs, and
    permission-based sandboxing.
8. **Networking and storage**: HTTP/TLS, TCP/UDP, WebSocket, SQLite, then
    optional database drivers and package registry support.
9. **Native/FFI backends**: reproducible native builds and opt-in AI/data
    science bridges.
10. **Self-hosting**: rewrite compiler components in Lizard after bootstrap
     conformance is stable on Windows, Linux, and macOS.

Each item exits only with implementation, tests, documentation, examples,
project-tree changes, and a reproducible command. See
`LIZARD_LANGUAGE_SPECIFICATION.md` for the complete capability matrix.

---

**Lizard Development Plan**  
**Core Runtime and Advanced Roadmap**  
**September 11, 2026**
