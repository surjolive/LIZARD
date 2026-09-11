# Lizard Programming Language - Language Specification

**Version**: 0.1.0 (design baseline)  
**Status**: Core interpreter implemented; advanced capabilities specified and staged  
**Date**: September 11, 2026  
**Author**: Surjo Live  
**License**: MIT

---

## 📋 Table of Contents

1. [Language Philosophy](#language-philosophy)
2. [Quick Overview](#quick-overview)
3. [Type System](#type-system)
4. [Syntax Specification](#syntax-specification)
5. [Keywords & Identifiers](#keywords--identifiers)
6. [Data Types](#data-types)
7. [Operators](#operators)
8. [Control Flow](#control-flow)
9. [Functions](#functions)
10. [Data Structures](#data-structures)
11. [Module System](#module-system)
12. [Error Handling](#error-handling)
13. [Standard Library](#standard-library)
14. [File Format & Extensions](#file-format--extensions)
15. [Execution Model](#execution-model)
16. [Grammar Overview](#grammar-overview)
17. [Example Programs](#example-programs)
18. [Implementation Roadmap](#implementation-roadmap)
19. [Advanced Capability Design](#advanced-capability-design)
20. [Realistic Delivery Roadmap](#realistic-delivery-roadmap)
21. [Master Prompt Coverage](#master-prompt-coverage)

---

## 1. Language Philosophy

### Core Principles

Lizard is designed with these core principles:

1. **Simplicity First** - Syntax should be clean, readable, and intuitive
2. **Beginner-Friendly** - Easy to learn, especially for those new to programming
3. **Powerful Yet Simple** - Can be used for simple scripts to complex applications
4. **Native Performance** - Compiled to native code for fast execution
5. **Fast Startup** - No runtime bloat, instant execution
6. **Consistent Syntax** - Original keywords and a distinct runtime model
7. **Clear Intent** - Code should express intent explicitly

### Design Decisions

- **Gradually Typed**: Dynamic execution first, optional static checks later
- **Interpreted First**: Tree-walking runtime is the compatibility baseline
- **Rust Runtime**: Rust provides the initial safe implementation platform
- **Multi-Platform**: Windows, Linux, macOS support
- **Minimal Dependencies**: No external crates for core functionality
- **Beginner-Focused Standard Library**: Clean, intuitive function names

---

## 2. Quick Overview

### What is Lizard?

Lizard is a general-purpose programming language that combines:
- **Simple Syntax**: Clean and familiar without being a Python clone
- **Native Performance**: Written in Rust, compiled to fast binaries
- **Rich Ecosystem**: Standard library with 46+ built-in functions
- **Module System**: Import/export for code organization
- **CLI Tools**: REPL, formatter, linter, build commands
- **Multi-Purpose**: Suitable for automation, web, data science, games, etc.

### File Extension

```
.lz
```

Examples:
- `hello.lz`
- `calculator.lz`
- `web_server.lz`
- `game.lz`

### Hello World

```lizard
say "Hello, LIZARD!"
```

### Run LIZARD Programs

```bash
# Run a file
lizard run hello.lz

# Run inline code
lz -e "say 42"

# REPL
lizard repl

# Check syntax
lz check hello.lz

# Format code
lz fmt hello.lz --write

# Build to executable
lz build hello.lz --output hello.exe

# Check system
lz doctor

# Version
lz version
```

---

## 3. Type System

### Type Philosophy

LIZARD uses **dynamic typing** with **runtime type checking**:

- Variables can hold any type
- Type checking occurs at runtime
- Type conversion is automatic where safe
- Operations on incompatible types produce errors
- Built-in type checking functions available

### Benefits of Dynamic Typing in LIZARD

✅ Flexible and powerful  
✅ Easy for beginners  
✅ Quick prototyping  
✅ Less boilerplate code  
✅ Duck typing support  

### Alternative: Optional Static Typing (Future)

```lizard
# Future enhancement: optional type hints
function add(a: number, b: number) -> number
    return a + b
end
```

---

## 4. Syntax Specification

### Code Structure

#### Comments

```lizard
# Single-line comment

# This is a comment
# spanning multiple lines

# Comments are ignored by the interpreter
```

#### Statements

LIZARD uses newlines to separate statements (semicolons optional):

```lizard
x = 5
y = 10
say x + y
```

Semicolons allowed but not required:

```lizard
x = 5; y = 10; say x + y
```

Line continuation with backslash:

```lizard
long_result = \
    value1 + value2 + \
    value3 + value4
```

#### Indentation

LIZARD uses **significant indentation** (like Python):

```lizard
if x > 0
    say "Positive"
    say "Greater than zero"
else
    say "Non-positive"
end
```

**Indentation Rules:**
- Use spaces (tab = 4 spaces internally)
- Consistent indentation within block
- Block start: keyword followed by newline
- Block end: `end` keyword

---

## 5. Keywords & Identifiers

### Reserved Keywords

```
# Variable & Constants
set, assign, const

# Control Flow
if, else, elif, when
while, repeat, each, for
break, continue, pass

# Functions
function, return, call
lambda, do

# Data Structures
list, map, tuple, set

# OOP
class, object, new, this, super
property, method
public, private, protected

# Modules
import, export, module, from

# Error Handling
try, catch, throw, raise, finally

# Other
true, false, null, void
and, or, not
in, is, has
yield, await, async

# Literals
this, super, self
```

### Naming Conventions

**Variables & Constants:**
```
variable_name        # snake_case recommended
VariableName         # PascalCase for constants
_private_var         # Leading underscore for private
```

**Functions & Methods:**
```
function_name        # snake_case
MethodName           # PascalCase for methods (OOP)
```

**Classes:**
```
MyClass              # PascalCase
PersonRecord         # Descriptive PascalCase
```

**Identifiers:**
- Start with letter or underscore
- Followed by letters, digits, underscores
- Case-sensitive
- No spaces or special characters (except underscore)

Valid identifiers:
```
name
_private
variable_123
calculateTotal
MyVariable
```

Invalid identifiers:
```
123name              # Starts with digit
my-variable          # Hyphen not allowed
my variable          # Space not allowed
class                # Reserved keyword
```

---

## 6. Data Types

### Primitive Types

#### 1. Number

Represents integers and floating-point numbers:

```lizard
x = 42              # Integer
y = 3.14            # Float
z = -100            # Negative
a = 1.5e10          # Scientific notation
```

Operations:
```lizard
result = 10 + 5     # Addition
result = 10 - 5     # Subtraction
result = 10 * 5     # Multiplication
result = 10 / 5     # Division
result = 10 % 3     # Modulo
result = 2 ** 8     # Exponent
```

Type checking:
```lizard
is_number(42)       # true
type(42)            # "number"
```

#### 2. Text

Represents strings (sequences of characters):

```lizard
name = "Alice"
message = "Hello, World!"
empty = ""
multiline = "Line 1
Line 2
Line 3"
```

String operations:
```lizard
text = "Hello" + " " + "World"     # Concatenation
repeated = "Ha" * 3                # "HaHaHa"
char = text[0]                      # Indexing
substring = text[0:5]              # Slicing
length(text)                        # 11
```

String functions:
```lizard
upper("hello")                      # "HELLO"
lower("HELLO")                      # "hello"
trim("  text  ")                    # "text"
split("a,b,c", ",")                # ["a", "b", "c"]
join(["a", "b"], "-")              # "a-b"
replace("hello", "l", "L")         # "heLLo"
starts_with("hello", "he")         # true
ends_with("hello", "lo")           # true
includes("hello", "ll")            # true
slice("hello", 1, 3)               # "el"
```

Type checking:
```lizard
is_text("hello")    # true
type("hello")       # "text"
```

#### 3. Boolean

Represents true/false values:

```lizard
active = true
ready = false
condition = 5 > 3   # true
```

Boolean operations:
```lizard
result = true and true        # true
result = true or false        # true
result = not false            # true
```

Truthiness:
```lizard
# Truthy values: any non-zero number, non-empty text, non-empty list, etc.
# Falsy values: 0, false, null, empty text, empty list, etc.

if "text"           # true (non-empty string)
    say "Yes"
end

if 0                # false (zero)
    say "No"
end
```

Type checking:
```lizard
is_bool(true)       # true
type(true)          # "bool"
```

#### 4. Null

Represents absence of value:

```lizard
x = null
nothing = null
```

Operations:
```lizard
is_null(null)       # true
is_null(0)          # false
type(null)          # "null"
```

Null propagation:
```lizard
x = null
# Accessing properties of null is an error
result = x + 5      # Error
```

### Collection Types

#### 5. List

Ordered collection of values (array):

```lizard
numbers = [1, 2, 3, 4, 5]
mixed = [1, "text", true, null]
empty = []
```

Operations:
```lizard
list = [1, 2, 3]
item = list[0]              # 1 (0-indexed)
list[1] = 10                # Modify element
push(list, 4)               # Add to end: [1, 10, 3, 4]
pop(list)                   # Remove and return last
shift(list)                 # Remove and return first
unshift(list, 0)            # Add to start
slice(list, 0, 2)           # Sublist
length(list)                # Number of elements
reverse(list)               # Reverse in place
sort(list)                  # Sort in place
```

Iteration:
```lizard
items = [1, 2, 3]

# Each loop
each items
    say it          # it = current item
end

# While loop
i = 0
while i < length(items)
    say items[i]
    i = i + 1
end
```

Type checking:
```lizard
is_list([1, 2, 3])  # true
type([1, 2, 3])     # "list"
```

#### 6. Map

Key-value collection (dictionary/object):

```lizard
person = {
    "name": "Alice",
    "age": 30,
    "city": "NYC"
}

empty = {}

# Keys can be text or numbers
data = {
    "key1": "value1",
    "key2": "value2"
}
```

Operations:
```lizard
person = {"name": "Bob", "age": 25}
name = person["name"]              # "Bob"
person["age"] = 26                 # Modify value
person["city"] = "LA"              # Add new key
has(person, "name")                # true
keys(person)                       # ["name", "age", "city"]
values(person)                     # ["Bob", 26, "LA"]
length(person)                     # 3
```

Iteration:
```lizard
person = {"name": "Alice", "age": 30}

# Each loop
each person
    say it          # it = current value
end

# Access keys
keys_list = keys(person)
each keys_list
    key = it
    value = person[key]
    say key + ": " + to_text(value)
end
```

Type checking:
```lizard
is_map({"a": 1})    # true
type({"a": 1})      # "map"
```

#### 7. Tuple (Future)

Immutable ordered collection:

```lizard
# Planned for future release
point = (x: 10, y: 20)
color = (255, 128, 0)  # RGB
```

#### 8. Set (Future)

Unordered unique collection:

```lizard
# Planned for future release
unique_numbers = {1, 2, 3, 4, 5}
```

### Type Checking Functions

```lizard
is_number(value)        # Check if number
is_text(value)          # Check if text
is_bool(value)          # Check if boolean
is_null(value)          # Check if null
is_list(value)          # Check if list
is_map(value)           # Check if map
type(value)             # Get type as text
```

### Type Conversion

```lizard
to_text(42)             # "42"
to_text(3.14)           # "3.14"
to_text(true)           # "true"
to_number("42")         # 42
to_number("3.14")       # 3.14
to_number("abc")        # Error
```

---

## 7. Operators

### Arithmetic Operators

| Operator | Name | Example | Result |
|----------|------|---------|--------|
| `+` | Addition | `5 + 3` | `8` |
| `-` | Subtraction | `5 - 3` | `2` |
| `*` | Multiplication | `5 * 3` | `15` |
| `/` | Division | `15 / 3` | `5` |
| `%` | Modulo | `17 % 5` | `2` |
| `**` | Exponent | `2 ** 3` | `8` |

### String Operators

| Operator | Name | Example | Result |
|----------|------|---------|--------|
| `+` | Concatenation | `"Hello" + " World"` | `"Hello World"` |
| `*` | Repetition | `"Ha" * 3` | `"HaHaHa"` |
| `[i]` | Indexing | `"hello"[0]` | `"h"` |
| `[i:j]` | Slicing | `"hello"[1:4]` | `"ell"` |

### Comparison Operators

| Operator | Name | Example | Result |
|----------|------|---------|--------|
| `==` | Equal | `5 == 5` | `true` |
| `!=` | Not equal | `5 != 3` | `true` |
| `<` | Less than | `3 < 5` | `true` |
| `>` | Greater than | `5 > 3` | `true` |
| `<=` | Less or equal | `3 <= 3` | `true` |
| `>=` | Greater or equal | `5 >= 3` | `true` |

### Logical Operators

| Operator | Name | Example | Result |
|----------|------|---------|--------|
| `and` | Logical AND | `true and false` | `false` |
| `or` | Logical OR | `true or false` | `true` |
| `not` | Logical NOT | `not false` | `true` |

### Assignment Operators

| Operator | Example | Equivalent |
|----------|---------|------------|
| `=` | `x = 5` | Assign |
| `+=` | `x += 3` | `x = x + 3` |
| `-=` | `x -= 3` | `x = x - 3` |
| `*=` | `x *= 3` | `x = x * 3` |
| `/=` | `x /= 3` | `x = x / 3` |
| `%=` | `x %= 3` | `x = x % 3` |

### Other Operators

| Operator | Name | Example | Meaning |
|----------|------|---------|---------|
| `[index]` | Indexing | `list[0]` | Access element |
| `[start:end]` | Slicing | `list[0:3]` | Sublist |
| `in` | Membership | `x in list` | Check membership |
| `is` | Identity | `x is y` | Same object |
| `has` | Contains (Map) | `has(map, "key")` | Key exists |

### Operator Precedence

Highest to lowest:

1. `[]`, `.` (indexing, member access)
2. `**` (exponent)
3. `-x`, `+x`, `not` (unary)
4. `*`, `/`, `%` (multiplication, division, modulo)
5. `+`, `-` (addition, subtraction)
6. `<`, `>`, `<=`, `>=`, `==`, `!=` (comparison)
7. `and` (logical AND)
8. `or` (logical OR)
9. `=`, `+=`, `-=`, etc. (assignment)

---

## 8. Control Flow

### If / Else / Elif

```lizard
if condition
    # code
end

if condition
    # true branch
else
    # false branch
end

if condition1
    # branch 1
elif condition2
    # branch 2
elif condition3
    # branch 3
else
    # default
end
```

Example:

```lizard
age = 18

if age >= 18
    say "Adult"
elif age >= 13
    say "Teenager"
else
    say "Child"
end
```

### While Loop

```lizard
while condition
    # body
    break       # Exit loop
    continue    # Skip to next
end
```

Example:

```lizard
count = 0
while count < 5
    say count
    count = count + 1
end
```

### Repeat Loop

```lizard
repeat n
    # body (executed n times)
    break
    continue
end
```

Example:

```lizard
repeat 3
    say "Hello"         # Printed 3 times
end
```

### Each Loop

```lizard
each collection
    # body
    # it = current item
    break
    continue
end
```

Example:

```lizard
items = [10, 20, 30]
each items
    say it              # 10, 20, 30
end

person = {"name": "Alice", "age": 30}
each person
    say it              # Values: "Alice", 30
end
```

### Pass Statement

```lizard
if condition
    pass                # Do nothing
end

while true
    pass                # Infinite loop
end
```

### Break & Continue

```lizard
# Break - exit loop
while true
    if condition
        break
    end
end

# Continue - skip to next iteration
repeat 10
    if is_odd
        continue
    end
    say it              # Only even numbers
end
```

---

## 9. Functions

### Function Declaration

```lizard
function name()
    # body
    return value
end
```

### Parameters & Arguments

```lizard
function greet(name, age)
    say "Hello, " + name
    say "Age: " + to_text(age)
end

greet("Alice", 30)
```

### Return Values

```lizard
function add(a, b)
    return a + b
end

result = add(5, 3)          # 8
```

Multiple returns:

```lizard
function get_person()
    name = "Alice"
    age = 30
    city = "NYC"
    return [name, age, city]    # Return list
end

person = get_person()
```

### Default Parameters (Future)

```lizard
# Planned feature
function greet(name, greeting = "Hello")
    say greeting + ", " + name
end

greet("Alice")              # Hello, Alice
greet("Bob", "Hi")          # Hi, Bob
```

### Variable-Length Arguments (Future)

```lizard
# Planned feature
function sum(args...)
    total = 0
    each args
        total = total + it
    end
    return total
end

result = sum(1, 2, 3, 4, 5)    # 15
```

### Lambda Functions (Future)

```lizard
# Planned feature
add = lambda(a, b) -> a + b
result = add(5, 3)              # 8

square = lambda(x) -> x * x
```

### Function Scope

Functions have local scope:

```lizard
x = 10                  # Global

function modify()
    x = 20              # Local x
    return x
end

say x                   # 10 (global unchanged)
say modify()            # 20 (local x)
```

### Recursion

```lizard
function factorial(n)
    if n <= 1
        return 1
    end
    return n * factorial(n - 1)
end

say factorial(5)        # 120
```

### Calling Functions

```lizard
# Simple call
result = function_name(arg1, arg2)

# Call with no arguments
result = function_name()

# Call with variable arguments
values = [1, 2, 3]
# (Pass array elements as arguments - future feature)
```

---

## 10. Data Structures

### Lists

Operations on lists:

```lizard
list = [1, 2, 3]
length(list)                    # 3
get(list, 0)                    # 1
push(list, 4)                   # [1, 2, 3, 4]
pop(list)                       # 4, list = [1, 2, 3]
shift(list)                     # 1, list = [2, 3]
unshift(list, 0)                # list = [0, 2, 3]
reverse(list)                   # [3, 2, 0]
sort(list)                      # [0, 2, 3]
has(list, 2)                    # true
slice(list, 0, 2)              # [0, 2] (sublist)
```

### Maps

Operations on maps:

```lizard
person = {"name": "Alice", "age": 30}
has(person, "name")             # true
get(person, "name")             # "Alice"
keys(person)                    # ["name", "age"]
values(person)                  # ["Alice", 30]
length(person)                  # 2
person["city"] = "NYC"          # Add key
```

### Nested Structures

```lizard
data = {
    "users": [
        {"name": "Alice", "age": 30},
        {"name": "Bob", "age": 25}
    ],
    "count": 2
}

first_user = data["users"][0]   # {"name": "Alice", "age": 30}
name = first_user["name"]       # "Alice"
```

---

## 11. Module System

### Import Statement

```lizard
# Import entire module
import math

# Use as
import math as m

# Import from standard library
import @std/string

# Selective import (future)
import math.add
import math.sqrt
```

### Export Statement

```lizard
# Export function
export function add(a, b)
    return a + b
end

# Export constant
export PI = 3.14159

# Export variable
export version = "1.0.0"
```

### Module Structure

```
project/
├── src/
│   ├── main.lz              # Entry point
│   ├── math/
│   │   └── mod.lz           # Math module
│   ├── utils/
│   │   ├── mod.lz           # Utils main
│   │   └── string.lz        # String utilities
│   └── structures/
│       └── mod.lz           # Data structures
└── lizard.json              # Project config
```

### Using Modules

```lizard
import math

result = math.add(5, 3)
area = math.circle_area(10)
```

### Standard Library Modules

Available modules:

- `@std/io` - Input/output
- `@std/math` - Mathematical functions
- `@std/string` - String manipulation
- `@std/array` - Array operations
- `@std/map` - Map/dictionary operations
- `@std/file` - File I/O
- `@std/json` - JSON parsing
- `@std/time` - Time functions
- `@std/random` - Random numbers
- `@std/regex` - Regular expressions
- `@std/http` - HTTP requests
- `@std/database` - Database connections

### Package Manager

```bash
lz package init                         # Initialize project
lz package install                      # Install dependencies
lz package install @http                # Install specific
lz package remove @http                 # Remove
lz package list                         # List installed
lz package update                       # Update all
lz package search keyword               # Search
lz package info @http                   # Get info
```

### Lizard.json

```json
{
  "name": "my_project",
  "version": "1.0.0",
  "description": "My LIZARD project",
  "author": "Your Name",
  "license": "MIT",
  "dependencies": {
    "@http": "1.0.0",
    "@database": "2.1.0"
  },
  "scripts": {
    "main": "src/main.lz",
    "test": "tests/run.lz"
  }
}
```

---

## 12. Error Handling

### Try / Catch / Finally (Future)

```lizard
# Planned feature
try
    result = 10 / 0     # Error
catch error
    say "Error: " + error
finally
    say "Done"
end
```

### Throw / Raise (Future)

```lizard
# Planned feature
if value < 0
    throw "Value must be positive"
end

if not valid
    raise ErrorMessage("Invalid input")
end
```

### Current Error Handling

For now, errors are reported and stop execution:

```lizard
x = to_number("abc")    # Error: Cannot convert "abc" to number
```

---

## 13. Standard Library

### Overview

LIZARD includes 46+ built-in functions across 7 categories:

#### I/O Functions (3)
- `say(text)` - Print with newline
- `print(value)` - Print any value
- `pause(msg, frames, delay)` - Animated pause

#### Type Functions (8)
- `type(value)` - Get type
- `length(value)` - Get size
- `is_number()`, `is_text()`, `is_list()`, `is_map()`, `is_bool()`, `is_null()`

#### String Functions (10)
- `upper()`, `lower()`, `split()`, `join()`, `starts_with()`, `ends_with()`, `includes()`, `slice()`, `trim()`, `replace()`

#### Array Functions (10)
- `push()`, `pop()`, `shift()`, `unshift()`, `reverse()`, `sort()`, `has()`, `get()`, `remove()`, `length()`

#### Math Functions (9)
- `pow()`, `sqrt()`, `floor()`, `ceil()`, `round()`, `abs()`, `min()`, `max()`, `clamp()`

#### Map Functions (4)
- `has()`, `get()`, `keys()`, `values()`

#### Conversion Functions (2)
- `to_text()`, `to_number()`

See [STANDARD_LIBRARY.md](STANDARD_LIBRARY.md) for complete reference.

---

## 14. File Format & Extensions

### LIZARD File Extension

```
.lz
```

### File Structure

A LIZARD file is a sequence of statements:

```lizard
# Comments

# Variables
name = "Alice"
age = 30

# Functions
function greet(person_name)
    say "Hello, " + person_name
end

# Main code
greet(name)

# Imports
import math
result = math.add(1, 2)
```

### File Encoding

- UTF-8 recommended
- UTF-16, Latin-1 supported
- BOM (Byte Order Mark) allowed but not recommended

### Line Endings

- Windows: `\r\n` (CRLF)
- Unix/Linux/macOS: `\n` (LF)
- Both supported transparently

---

## 15. Execution Model

### Interpretation

LIZARD uses a **tree-walking interpreter**:

```
LIZARD Source Code (.lz)
        ↓
Lexer (Tokenization)
        ↓
Parser (Syntax Analysis)
        ↓
AST (Abstract Syntax Tree)
        ↓
Semantic Analysis
        ↓
Interpreter (Tree Walking)
        ↓
Output / Side Effects
```

### Scope & Binding

Variables follow lexical scoping:

```lizard
x = 10                  # Global scope

function outer()
    y = 20              # Outer scope
    
    function inner()
        z = 30          # Inner scope
        say x           # Can access global
        say y           # Can access outer
    end
    
    inner()
end

# Variables are looked up in order:
# 1. Local scope
# 2. Enclosing scope(s)
# 3. Global scope
# 4. Built-in scope
```

### Execution Order

LIZARD executes code top-to-bottom:

1. Import statements (if any)
2. Function definitions (hoisted)
3. Statements in order

### Memory Model

LIZARD uses automatic memory management:
- Garbage collection for unreachable objects
- Reference counting for efficiency
- Automatic cleanup on exit

### Runtime Errors

Runtime errors stop execution:

```lizard
result = 10 / 0                 # Error: Division by zero
name = to_number("abc")        # Error: Cannot convert
x = unknown_variable            # Error: Undefined variable
result = [1, 2][10]            # Error: Index out of bounds
```

---

## 16. Grammar Overview

### EBNF-Style Specification

```ebnf
program         = statement*

statement       = (assignment | expression | control_flow | function_def | import | export) newline*

assignment      = identifier "=" expression

expression      = or_expr

or_expr         = and_expr ("or" and_expr)*
and_expr        = not_expr ("and" not_expr)*
not_expr        = "not" not_expr | comparison
comparison      = additive ((("==" | "!=" | "<" | ">" | "<=" | ">=") additive)*)
additive        = multiplicative (("+" | "-") multiplicative)*
multiplicative  = unary (("*" | "/" | "%") unary)*
unary           = ("-" | "+") unary | power
power           = postfix ("**" postfix)*
postfix         = primary ("[" expression "]" | "." identifier)*
primary         = number | string | boolean | null
                | identifier
                | "(" expression ")"
                | list_literal
                | map_literal
                | function_call

control_flow    = if_stmt | while_stmt | repeat_stmt | each_stmt | break | continue | pass

if_stmt         = "if" expression newline+ block ("elif" expression newline+ block)* ("else" newline+ block)? "end"
while_stmt      = "while" expression newline+ block "end"
repeat_stmt     = "repeat" expression newline+ block "end"
each_stmt       = "each" expression newline+ block "end"
break           = "break"
continue        = "continue"
pass            = "pass"

function_def    = "function" identifier "(" parameters? ")" newline+ block "end"
parameters      = identifier ("," identifier)*

block           = statement*

function_call   = identifier "(" arguments? ")"
arguments       = expression ("," expression)*

list_literal    = "[" (expression ("," expression)*)? "]"
map_literal     = "{" (string ":" expression ("," string ":" expression)*)? "}"

import          = "import" identifier ("as" identifier)?
export          = "export" (function_def | assignment | expression)
```

---

## 17. Example Programs

### Example 1: Hello World

```lizard
say "Hello, LIZARD!"
```

### Example 2: Variables & Arithmetic

```lizard
x = 10
y = 20
sum = x + y
product = x * y

say "Sum: " + to_text(sum)
say "Product: " + to_text(product)
```

### Example 3: Functions

```lizard
function factorial(n)
    if n <= 1
        return 1
    end
    return n * factorial(n - 1)
end

result = factorial(5)
say "5! = " + to_text(result)
```

### Example 4: Lists & Loops

```lizard
numbers = [1, 2, 3, 4, 5]
sum = 0

each numbers
    sum = sum + it
end

say "Sum: " + to_text(sum)
```

### Example 5: Maps & Conditionals

```lizard
person = {
    "name": "Alice",
    "age": 30,
    "city": "NYC"
}

if person["age"] >= 18
    say person["name"] + " is an adult"
end
```

### Example 6: String Operations

```lizard
text = "hello world"
upper_text = upper(text)
words = split(text, " ")

say "Original: " + text
say "Upper: " + upper_text
say "First word: " + words[0]
```

### Example 7: Modules (Future)

```lizard
import math

say "π (PI) = " + to_text(math.PI)
area = math.circle_area(10)
say "Circle area (r=10): " + to_text(area)
```

---

## 18. Implementation Roadmap

### PHASE 1: Language Specification ✅

**Goals:**
- Define syntax and keywords
- Specify type system
- Document execution model
- Create grammar
- Establish design principles

**Deliverables:**
- This specification document
- Grammar reference
- Example programs
- Design rationale

---

### PHASE 2: Lexer

**Goals:**
- Tokenize LIZARD source code
- Handle keywords, identifiers, operators
- Support comments and string literals
- Track line/column information

**Deliverables:**
- Complete lexer implementation
- Token definitions
- Error reporting
- Lexer tests

---

### PHASE 3: Parser & AST

**Goals:**
- Parse tokens into Abstract Syntax Tree
- Handle operator precedence
- Validate syntax
- Generate AST nodes

**Deliverables:**
- Parser implementation
- AST node definitions
- Parser tests
- Error recovery

---

### PHASE 4: Interpreter/Runtime

**Goals:**
- Interpret AST
- Implement evaluation engine
- Memory management
- Error handling

**Deliverables:**
- Interpreter implementation
- Variable storage
- Function calling
- Built-in operations
- Runtime tests

---

### PHASE 5: Variables, Expressions, Conditions, Loops

**Goals:**
- Variable assignment and lookup
- Arithmetic and logical expressions
- if/else/elif statements
- while/repeat/each loops
- break/continue

**Deliverables:**
- Variable management
- Expression evaluation
- Control flow implementation
- Integration tests

---

### PHASE 6: Functions

**Goals:**
- Function declaration
- Parameters and arguments
- Return values
- Local scope
- Recursion

**Deliverables:**
- Function implementation
- Scope management
- Parameter binding
- Function tests

---

### PHASE 7: Data Structures

**Goals:**
- Lists and indexing
- Maps and key access
- Slicing
- Collection operations
- Nested structures

**Deliverables:**
- List implementation
- Map implementation
- Collection functions
- Data structure tests

---

### PHASE 8: OOP (Object-Oriented Programming)

**Goals:**
- Class definitions
- Objects and instances
- Methods and properties
- Constructors
- Inheritance (optional for v0.1)

**Deliverables:**
- Class implementation
- Object system
- Method calls
- OOP tests

---

### PHASE 9: Modules & Package Manager

**Goals:**
- Module import/export
- Module loading
- Package management CLI
- Standard library modules
- Package registry (future)

**Deliverables:**
- Module system
- Package manager CLI
- Module tests
- Example packages

---

### PHASE 10: Standard Library

**Goals:**
- Implement 46+ built-in functions
- Organize into modules
- Complete documentation
- Tests for each function

**Deliverables:**
- Complete standard library
- Function implementations
- Documentation
- Library tests

---

### PHASE 11: Async/Concurrency

**Goals:**
- async/await syntax
- Promises (if applicable)
- Concurrency support
- Async tests

**Deliverables:**
- Async implementation
- Concurrency primitives
- Example programs
- Tests

---

### PHASE 12: Database & Networking

**Goals:**
- Database connections
- SQL queries
- HTTP requests
- Socket programming

**Deliverables:**
- Database APIs
- HTTP client
- Network functions
- Tests

---

### PHASE 13: Web Framework

**Goals:**
- HTTP server
- Routing
- Request/response handling
- Middleware
- Templates

**Deliverables:**
- Web framework
- Example server
- Routing system
- Tests

---

### PHASE 14: Developer Tools

**Goals:**
- Formatter (linter)
- Debugger support
- REPL enhancement
- Documentation generator

**Deliverables:**
- Formatter tool
- Linter tool
- Enhanced REPL
- Doc generator

---

### PHASE 15: VS Code Extension

**Goals:**
- Syntax highlighting
- IntelliSense
- Debugging integration
- Commands and shortcuts

**Deliverables:**
- VS Code extension
- Grammar file
- Extension tests

---

### PHASE 16: Compiler/VM Optimization

**Goals:**
- Bytecode compilation
- VM implementation
- Performance optimization
- Profiling

**Deliverables:**
- Bytecode compiler
- VM implementation
- Benchmarks
- Optimization report

---

### PHASE 17: Self-Hosting Roadmap

**Goals:**
- Rewrite lexer in LIZARD
- Rewrite parser in LIZARD
- Bootstrap compiler
- Maintain Rust runtime for efficiency

**Deliverables:**
- LIZARD-written components
- Self-hosting roadmap
- Bootstrap procedure

---

## 19. Advanced Capability Design

The following features are part of the Lizard design, but are **planned unless
the implementation status explicitly says implemented**. A feature becomes
official only after its parser/runtime implementation, tests, examples, and
documentation land together.

### 19.1 Custom Types, Generics, and Pattern Matching

The type layer will be gradual: ordinary `.lz` code remains dynamic, while
annotations enable checking and tooling.

```lizard
type UserId = number

struct Pair<T, U>
        first: T
        second: U
end

enum Result<T>
        ok(value: T)
        error(message: text)
end

trait Printable
        fn display() -> text
end

fn unwrap<T>(value: Option<T>, fallback: T) -> T
        match value
                some(item) => item
                none => fallback
        end
end
```

Planned type features include `Option<T>` (`some`/`none`), type aliases,
custom errors, interface/trait conformance, named arguments, destructuring,
and multiple return values:

```lizard
fn divide(left: number, right: number) -> (number, number)
        return left / right, left % right
end

(quotient, remainder) = divide(left: 10, right: 3)
```

Operator overloading will be explicit and bounded by a trait, never an
arbitrary runtime hook. Extension methods will be imported explicitly so they
cannot silently change a type's behavior.

### 19.2 Runtime and Performance Architecture

The delivery order is:

```text
.lz source -> lexer -> parser -> typed AST -> bytecode -> Lizard VM
                                                                            \-> interpreter fallback
```

- The current Rust tree-walking interpreter remains the compatibility oracle.
- A register or stack bytecode VM will be added only after AST behavior is
    covered by conformance tests.
- Native compilation will use a stable intermediate representation.
- JIT compilation is an optional later backend, not a v0.1 promise.
- Memory management starts with Rust ownership and reference-counted runtime
    handles; a tracing garbage collector is considered only for cyclic managed
    objects.
- Async tasks use structured cancellation. Threads, processes, and parallel
    iterators require explicit APIs and safe message passing.
- SIMD/GPU execution is library/backend-specific and must have a portable
    fallback.

### 19.3 Networking and Data Formats

The standard library will expose capability-based APIs for `lz.http`, `lz.net`,
`lz.websocket`, `lz.dns`, and `lz.tls`. HTTP clients will validate URLs,
timeouts, redirects, and certificate verification by default. REST, GraphQL,
and gRPC belong in versioned packages built on those primitives.

Planned format modules are `lz.json`, `lz.xml`, `lz.yaml`, `lz.csv`,
`lz.toml`, and `lz.binary`. Parsers must impose size/depth limits and return
structured errors instead of silently accepting malformed input.

### 19.4 Database and AI Interoperability

Database adapters will share one parameterized interface:

```lizard
db = database.open("sqlite", "app.db")
rows = db.query("select * from users where id = ?", [user_id])
db.close()
```

SQLite is the first adapter. MySQL, PostgreSQL, and Redis are optional drivers;
an ORM is a separate package and never replaces parameterized queries.

NumPy, Pandas, PyTorch, TensorFlow, and Scikit-learn interoperability will be
implemented through explicit FFI/bridge packages. Tensor operations, model
loading, GPU acceleration, and local inference are future capabilities with
isolated native dependencies and serialization limits.

### 19.5 Application Targets

The platform roadmap supports CLI tools, desktop bindings, web/API servers,
automation, games, data processing, and system scripting. Each target is a
library or toolchain layer, not a promise that the core language embeds every
framework. Desktop support will use platform adapters for Windows, Linux, and
macOS; WebAssembly and ARM are later targets.

### 19.6 Security Model

Lizard programs run with least privilege when launched in sandbox mode:

```bash
lizard run app.lz --sandbox=default
lizard run app.lz --allow=net:api.example,read:./data
```

The permission model covers filesystem, network, subprocess, environment, and
FFI access. Secure defaults include TLS verification, bounded resource use,
parameterized database queries, safe subprocess argument arrays, cryptographic
hashing, and OS-backed secure random generation. The project explicitly does
not provide malware, credential theft, unauthorized access, or exploit tooling.

### 19.7 Package Ecosystem and Project File

The official executable is `lizard`; source files use `.lz`. Projects use
`lizard.toml`:

```toml
[project]
name = "weather_app"
version = "0.1.0"
entry = "src/main.lz"
requires_lizard = ">=0.1.0"

[dependencies]
http = "1.0.0"

[build]
backend = "bytecode"
```

The future registry is `packages.lizard.dev`. Package metadata includes name,
version, license, source, dependencies, supported targets, checksums, and
maintainer identity. The CLI contract is:

```text
lizard install package
lizard remove package
lizard update
lizard search package
lizard list
lizard publish package
```

Lockfiles, checksum verification, dependency resolution, yanked versions, and
offline cache behavior are required before publishing is considered stable.

### 19.8 Developer Tools and Documentation

The official command surface is:

```text
lizard run app.lz       lizard build app.lz
lizard test             lizard format
lizard lint             lizard debug
lizard docs             lizard repl
lizard benchmark        lizard profile
lizard version          lizard package
```

`lizard test` discovers `*_test.lz` and `tests/` files, provides assertions,
integration tests, benchmarks, and later coverage. `lizard format` defines
stable indentation, quote, spacing, and trailing-newline rules. `lizard lint`
reports diagnostics without changing files. Documentation comments use `##`
and `lizard docs` emits Markdown or a static site for public functions,
classes, types, modules, and packages.

The VS Code extension will provide syntax highlighting first, then completion,
diagnostics, formatting, definition/reference navigation, hover docs,
debugging, and LSP transport as the compiler exposes stable source spans.

### 19.9 Error System

Errors are structured values with a kind, message, source span, cause, and
optional suggestion:

```text
Error[NameNotFound]: Undefined variable "name"
    --> hello.lz:5:9
    suggestion: did you mean "username"?
```

Future `try`, `catch`, `finally`, and `throw` syntax will preserve error
chains and stack traces. Runtime errors must never expose secrets in messages
or logs.

### 19.10 Self-Hosting and Cross-Compilation

The Rust implementation is the bootstrap compiler/runtime. Once the lexer,
parser, formatter, and bytecode VM have conformance tests, those components
can be rewritten in Lizard. A three-stage bootstrap is required:

```text
Rust compiler -> Lizard compiler 1 -> Lizard compiler 2
```

Each stage must compile the next stage and produce equivalent conformance-test
results on Windows, Linux, and macOS.

## 20. Realistic Delivery Roadmap

This roadmap is ordered by dependency and risk, not by feature count.

| Stage | Deliverable | Exit condition |
| --- | --- | --- |
| 0 | Identity and compatibility baseline | `lizard`, `.lz`, CLI smoke tests, documented syntax |
| 1 | Lexer and source spans | strings/comments/operators have lexer tests |
| 2 | AST parser | parser tests cover every core statement and expression |
| 3 | Diagnostics | errors include file, line, column, and suggestions where possible |
| 4 | Core runtime | variables, functions, collections, control flow, and OOP conformance suite |
| 5 | Standard library modules | `lz.io`, `lz.math`, `lz.string`, `lz.collections`, `lz.json` tested |
| 6 | Testing/formatting/linting/docs tools | commands work on a sample project |
| 7 | Modules and `lizard.toml` | local imports, lockfile, and project builds are reproducible |
| 8 | Bytecode VM | VM passes the interpreter conformance suite |
| 9 | Optional typing and custom types | aliases, structs, enums, options, traits, and pattern matching tested |
| 10 | Async and concurrency | cancellation, channels, threads, and processes have safe APIs |
| 11 | Security sandbox | permissions and resource limits are enforced on all supported hosts |
| 12 | Networking and databases | HTTP/TLS, TCP/UDP, SQLite, and parameterized queries tested |
| 13 | Package registry | signed metadata, checksums, dependency solving, and publishing workflow |
| 14 | Native backend | reproducible native builds with a portable fallback |
| 15 | FFI and AI bridges | optional NumPy/Pandas/ML adapters with isolated dependencies |
| 16 | LSP and debugger | editor diagnostics, navigation, hover, and stepping work |
| 17 | Self-hosting | Lizard compiler builds itself on Windows, Linux, and macOS |

Every stage requires a specification update, implementation, focused tests,
documentation, examples, project-tree update, and a reproducible run command.
No stage is marked complete merely because its syntax has been designed.

## Summary

This specification defines Lizard's current core and its staged future. The
working v0.1 implementation is a Rust interpreter with a growing standard
library, functions, collections, loops, modules by design, and basic classes
and objects. The advanced capabilities above are design commitments and
roadmap items, not claims of current implementation.

✅ **Clear Syntax** - Intuitive and beginner-friendly  
✅ **Rich Type System** - Dynamic typing with type checking  
✅ **Complete Keywords** - Original LIZARD keywords  
✅ **Control Flow** - if/else, loops, functions  
✅ **Data Structures** - Lists, maps, and more  
✅ **Standard Library** - 46+ built-in functions  
✅ **Execution Model** - Tree-walking interpreter  
🧭 **Advanced Features** - Specified and staged, not yet complete  
🧭 **Implementation Roadmap** - Dependency-ordered delivery plan  

LIZARD is designed to be:
- 🎯 Beginner-friendly
- 🚀 Powerful
- 💻 General-purpose
- 🔧 Well-documented
- 🎓 Educational
- 🏗️ Designed for incremental production use

---

**Lizard Programming Language v0.1.0**
**Language Specification and Advanced Roadmap**
**September 11, 2026**

*For questions, contributions, or issues, please visit the [LIZARD GitHub repository](https://github.com/surjolive/LIZARD).*

## 21. Master Prompt Coverage

This section records the remaining requirements from the Lizard master plan.
These are specifications and acceptance criteria; they are not claims that all
of the features already exist in the v0.1 interpreter.

### 21.1 First-Party Library Map

The stable namespace is `lz.*`; short imports such as `import array` are a
future convenience that resolve to the same first-party modules.

| Module | Scope | Initial acceptance test |
| --- | --- | --- |
| `lz.io` | input, output, streams | deterministic text I/O |
| `lz.math` | numeric functions and constants | numeric conformance tests |
| `lz.string` | Unicode-safe text operations | Unicode cases |
| `lz.collections` | list, map, set, tuple utilities | mutation and iteration tests |
| `lz.array` | multidimensional arrays, broadcasting, linear algebra | shape and arithmetic tests |
| `lz.data` | Series/DataFrame, CSV/JSON, filter/group/join | table transformation tests |
| `lz.plot` | line, bar, scatter, histogram, pie, heatmap, basic 3D | artifact and snapshot tests |
| `lz.json` / `lz.csv` | bounded structured-data parsing | malformed-input tests |
| `lz.regex` | safe regular expressions | timeout/limit tests |
| `lz.http` / `lz.net` | HTTP(S), TCP, UDP, DNS, WebSocket | local test-server tests |
| `lz.os` / `lz.path` | platform paths and process environment | sandboxed platform tests |
| `lz.time` / `lz.random` | clocks, timers, secure and non-secure random | deterministic seeded tests |
| `lz.crypto` | hashes, signatures, secure primitives | known-answer tests |
| `lz.database` | SQLite first, then MySQL/PostgreSQL/Redis drivers | prepared-query tests |
| `lz.process` | safe subprocess execution | argument and permission tests |
| `lz.async` | tasks, futures, cancellation, channels | scheduler tests |
| `lz.testing` | assertions, discovery, benchmarks, coverage | self-tests |
| `lz.ml` / `lz.tensor` / `lz.nn` / `lz.vision` | tensors, models, neural networks, images | optional adapter tests |
| `lz.gui` / `lz.game` | windows/widgets and 2D game primitives | platform smoke tests |

`lz.array` is Lizard's NumPy-like API and must define shapes, dtypes, slicing,
broadcasting, matrix multiplication, statistics, and memory layout. `lz.data`
is a Pandas-like API for Series/DataFrame operations, missing values, cleaning,
grouping, aggregation, merge, and join. `lz.plot` owns an original plotting
API and may use native rendering backends without copying their source code.

### 21.2 Web, GUI, Game, and WASM Contracts

The web layer will provide routes, request/response values, JSON responses,
templates, middleware, REST endpoints, and WebSocket handlers. The final route
syntax must be approved in the stable syntax specification before being
implemented.

```lizard
web route "/hello"
    return web.text("Hello from Lizard")
end
```

`lz.gui` will define windows, buttons, labels, inputs, tables, menus, dialogs,
and events through Windows/Linux/macOS adapters. `lz.game` will define a game
loop, sprites, animation, audio, keyboard/mouse input, physics, and collision
interfaces. These are separate libraries, not core-language keywords.

The WebAssembly target is invoked by:

```text
lizard build hello.lz --target wasm
```

The output is `hello.wasm`, subject to a documented host-API permission model.
ARM and other architectures use the same backend contract after the portable
bytecode target is stable.

### 21.3 Tooling and Project Generation

The command contract includes:

```text
lizard create app myapp
lizard create web myweb
lizard create lib mylib
lizard run app.lz
lizard build app.lz
lizard test --coverage
lizard format
lizard lint
lizard debug app.lz
lizard docs
lizard benchmark
lizard profile
lizard repl
lizard package
```

The REPL prompt is `Lizard >` and must support history, multiline blocks,
imports, and source locations in diagnostics. The formatter owns official
indentation, quoting, spacing, and newline rules. The linter detects syntax,
unused names, unreachable code, applicable type problems, style issues, and
unsafe API use. The debugger exposes breakpoints, stepping, variables, call
stack, and watch expressions. The LSP exposes completion, diagnostics,
definition, references, hover, rename, and formatting.

### 21.4 Repository and Release Identity

The planned repository layout is:

```text
Lizard/
├── compiler/{lexer,parser,ast,semantic,ir,optimizer,codegen}/
├── runtime/
├── vm/
├── stdlib/{io,math,array,data,http}/
├── packages/
├── tests/
├── examples/
├── benchmarks/
├── docs/
├── tools/
├── lsp/
├── debugger/
├── vscode-extension/
├── README.md
├── LICENSE
├── AUTHORS
├── NOTICE
├── CHANGELOG.md
├── CONTRIBUTING.md
└── lizard.toml
```

Project metadata identifies **Surjo Deb Nath** as creator/maintainer and uses
the project target `https://github.com/SURJO99exe/lizard`. Metadata must not
claim legal exclusivity. `AUTHORS`, `NOTICE`, README attribution, release
history, and the selected license must be kept consistent. Releases use
semantic versioning (`v0.1.0`, `v0.2.0`, `v1.0.0`) and `CHANGELOG.md` records
features, fixes, breaking changes, and measured performance changes.

### 21.5 CI/CD, Benchmarks, and Deployment

GitHub Actions will run formatting, linting, unit/integration tests, examples,
builds, and release jobs on Windows, Linux, and macOS. Release artifacts will
include standalone executables and documented Docker/server deployment paths.

`lizard benchmark` and `lizard profile` must measure startup, execution,
memory, array operations, file operations, HTTP behavior, and compilation
time. Baselines and machine details are recorded; unsupported performance
claims are prohibited.

### 21.6 Phase Completion Contract

Every implementation phase must provide:

1. Specification changes.
2. Source files created or modified.
3. A project tree update.
4. Working implementation or an explicit `NOT IMPLEMENTED` status.
5. Unit, integration, and example tests appropriate to the feature.
6. Documentation and runnable examples.
7. Installation and execution commands.
8. Known limitations and a TODO list.
9. The next phase and compatibility impact.

The project must remain runnable after every phase. Stable syntax cannot be
silently changed; breaking changes require a versioning entry and migration
notes. No placeholder, design sketch, benchmark, interoperability claim, or
self-hosting claim may be presented as complete implementation.

### 21.7 Self-Hosting Acceptance Test

Self-hosting is complete only when a reproducible bootstrap test can:

```text
bootstrap Rust compiler
        -> build Lizard compiler
        -> build the Lizard compiler again
        -> compare outputs and conformance results
```

The process must be documented and repeatable on Windows, Linux, and macOS.
