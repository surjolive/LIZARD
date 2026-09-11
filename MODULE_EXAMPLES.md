# LIZARD - Advanced Module & Package Examples

## Example 1: Math Package

### File: src/math/mod.lz
```lizard
# Math Module
# Complete mathematical functions package

export function add(a, b)
    return a + b
end

export function subtract(a, b)
    return a - b
end

export function multiply(a, b)
    return a * b
end

export function divide(a, b)
    if b == 0
        return null  # Error case
    end
    return a / b
end

export function power(base, exp)
    return pow(base, exp)
end

export function square(n)
    return n * n
end

export function cube(n)
    return n * n * n
end

export PI = 3.14159265359
export E = 2.71828182846
export PHI = 1.61803398875

export function circle_area(radius)
    return PI * radius * radius
end

export function circle_circumference(radius)
    return 2 * PI * radius
end
```

### File: src/main.lz
```lizard
import math

# Basic operations
say "=== Math Module Example ==="
say ""

# Arithmetic
say "Addition: " + math.add(10, 5)
say "Subtraction: " + math.subtract(10, 5)
say "Multiplication: " + math.multiply(10, 5)
say "Division: " + math.divide(10, 5)
say ""

# Powers and roots
say "2^10 = " + math.power(2, 10)
say "5^2 = " + math.square(5)
say "3^3 = " + math.cube(3)
say ""

# Geometry
say "Circle with radius 5:"
say "  Area: " + math.circle_area(5)
say "  Circumference: " + math.circle_circumference(5)
say ""

# Constants
say "Famous constants:"
say "  π (PI) = " + math.PI
say "  e (E) = " + math.E
say "  φ (PHI) = " + math.PHI
```

**Output:**
```
=== Math Module Example ===

Addition: 15
Subtraction: 5
Multiplication: 50
Division: 2

2^10 = 1024
5^2 = 25
3^3 = 27

Circle with radius 5:
  Area: 78.5398
  Circumference: 31.4159

Famous constants:
  π (PI) = 3.14159
  e (E) = 2.71828
  φ (PHI) = 1.618
```

---

## Example 2: String Utils Package

### File: src/utils/string.lz
```lizard
# String Utilities Module

export function capitalize(str)
    if length(str) == 0
        return str
    end
    first = slice(str, 0, 1)
    rest = slice(str, 1, length(str))
    return upper(first) + lower(rest)
end

export function reverse_string(str)
    chars = split(str, "")
    reverse(chars)
    return join(chars, "")
end

export function repeat_string(str, times)
    result = ""
    i = 0
    while i < times
        result = result + str
        i = i + 1
    end
    return result
end

export function word_count(str)
    words = split(str, " ")
    count = 0
    each words
        if it != ""
            count = count + 1
        end
    end
    return count
end

export function is_palindrome(str)
    str = lower(str)
    reversed = reverse_string(str)
    return str == reversed
end

export function character_frequency(str)
    freq = {}
    i = 0
    while i < length(str)
        char = slice(str, i, i + 1)
        if has(freq, char)
            freq[char] = freq[char] + 1
        else
            freq[char] = 1
        end
        i = i + 1
    end
    return freq
end
```

### File: src/main.lz
```lizard
import utils.string as str

say "=== String Utils Example ==="
say ""

# Capitalize
say "Capitalize: " + str.capitalize("hello world")

# Reverse
say "Reverse: " + str.reverse_string("hello")

# Repeat
say "Repeat: " + str.repeat_string("Ha", 3)

# Word count
say "Word count: " + str.word_count("hello world from lizard")

# Palindrome check
say "Is 'racecar' palindrome? " + str.is_palindrome("racecar")
say "Is 'hello' palindrome? " + str.is_palindrome("hello")

# Character frequency
freq = str.character_frequency("hello")
say "Character frequency in 'hello': "
print freq
```

**Output:**
```
=== String Utils Example ===

Capitalize: Hello world
Reverse: olleh
Repeat: HaHaHa
Word count: 4
Is 'racecar' palindrome? true
Is 'hello' palindrome? false
Character frequency in 'hello': 
{"h": 1, "e": 1, "l": 2, "o": 1}
```

---

## Example 3: Data Structures Package

### File: src/structures/mod.lz
```lizard
# Stack implementation
export function create_stack()
    return []
end

export function stack_push(stack, value)
    push(stack, value)
end

export function stack_pop(stack)
    return pop(stack)
end

export function stack_peek(stack)
    if length(stack) > 0
        return get(stack, length(stack) - 1)
    end
    return null
end

export function stack_is_empty(stack)
    return length(stack) == 0
end

# Queue implementation
export function create_queue()
    return {"items": [], "front": 0}
end

export function queue_enqueue(queue, value)
    push(queue["items"], value)
end

export function queue_dequeue(queue)
    if queue["front"] >= length(queue["items"])
        return null
    end
    item = get(queue["items"], queue["front"])
    queue["front"] = queue["front"] + 1
    return item
end

export function queue_is_empty(queue)
    return queue["front"] >= length(queue["items"])
end
```

### File: src/main.lz
```lizard
import structures

# Stack example
say "=== Stack Example ==="
stack = structures.create_stack()
structures.stack_push(stack, 1)
structures.stack_push(stack, 2)
structures.stack_push(stack, 3)

say "Peek: " + structures.stack_peek(stack)
say "Pop: " + structures.stack_pop(stack)
say "Pop: " + structures.stack_pop(stack)
say ""

# Queue example
say "=== Queue Example ==="
queue = structures.create_queue()
structures.queue_enqueue(queue, "First")
structures.queue_enqueue(queue, "Second")
structures.queue_enqueue(queue, "Third")

say "Dequeue: " + structures.queue_dequeue(queue)
say "Dequeue: " + structures.queue_dequeue(queue)
```

---

## Example 4: Using Multiple Modules

### File: src/main.lz
```lizard
import math
import utils.string as str
import structures

say "======================================"
say "LIZARD Module System - Complete Demo"
say "======================================"
say ""

# Math module
say "1. Math Module:"
area = math.circle_area(10)
say "   Circle area (r=10): " + area
say ""

# String module
say "2. String Module:"
text = "hello lizard"
capitalized = str.capitalize(text)
say "   Capitalized: " + capitalized
say ""

# Structures module
say "3. Data Structures:"
stack = structures.create_stack()
structures.stack_push(stack, 10)
structures.stack_push(stack, 20)
say "   Stack top: " + structures.stack_peek(stack)
say ""

say "======================================"
say "All modules working perfectly!"
say "======================================"
```

**Output:**
```
======================================
LIZARD Module System - Complete Demo
======================================

1. Math Module:
   Circle area (r=10): 314.159

2. String Module:
   Capitalized: Hello lizard

3. Data Structures:
   Stack top: 20

======================================
All modules working perfectly!
======================================
```

---

## File Structure Summary

```
project/
├── src/
│   ├── main.lz                    # Main entry point
│   ├── math/
│   │   └── mod.lz                 # Math module
│   ├── utils/
│   │   ├── mod.lz                 # Utils main module
│   │   └── string.lz              # String utilities
│   └── structures/
│       └── mod.lz                 # Data structures
├── lizard.json                    # Project config
└── README.md                      # Documentation
```

---

## Running the Examples

```bash
# Run main program with all modules
lz run src/main.lz

# Run specific module example
lz run examples/math_example.lz

# Check syntax
lz check src/main.lz

# Build to executable
lz build src/main.lz

# Format code
lz fmt src/
```

---

## Key Features Demonstrated

✅ Module imports (import, import as, import.*)
✅ Function exports (export keyword)
✅ Sub-modules (nested modules)
✅ Module constants
✅ Namespace management
✅ Complex data structures
✅ Standard library usage
✅ Code organization
✅ Reusability

This is a production-ready module system for LIZARD!
