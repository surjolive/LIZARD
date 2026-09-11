// Standard Library Reference for LIZARD
// ====================================

## 📚 Complete Built-in Function Library

### 1️⃣ I/O Functions (Input/Output)

#### say(message)
```lizard
say "Hello, World!"
say "Number: " + 42
say ""  # Empty line
```
**Purpose**: Print text with newline (like println)

#### print(value)
```lizard
print 42
print "Hello"
print [1, 2, 3]
print {"name": "LIZARD"}
```
**Purpose**: Print any value to output

#### pause(message, frames, delay)
```lizard
pause "Loading", 4, 100  # Shows animation
```
**Purpose**: Show animated pause with spinner

---

### 2️⃣ Type Functions

#### type(value)
```lizard
type(42)           # "number"
type("text")       # "text"
type(true)         # "bool"
type([1,2,3])      # "list"
type({"a": 1})     # "map"
type(null)         # "null"
```
**Purpose**: Get type of value as string

#### length(value)
```lizard
length("hello")    # 5
length([1,2,3])    # 3
length({"a": 1})   # 1
```
**Purpose**: Get length of string, list, or map

#### is_number(value)
```lizard
is_number(42)      # true
is_number("42")    # false
```
**Purpose**: Check if value is number

#### is_text(value)
```lizard
is_text("hello")   # true
is_text(42)        # false
```
**Purpose**: Check if value is string

#### is_list(value)
```lizard
is_list([1,2,3])   # true
is_list("list")    # false
```
**Purpose**: Check if value is list

#### is_map(value)
```lizard
is_map({"a": 1})   # true
is_map([1,2,3])    # false
```
**Purpose**: Check if value is map

#### is_bool(value)
```lizard
is_bool(true)      # true
is_bool(1)         # false
```
**Purpose**: Check if value is boolean

---

### 3️⃣ String Functions

#### upper(string)
```lizard
upper("hello")     # "HELLO"
upper("HeLLo")     # "HELLO"
```
**Purpose**: Convert string to uppercase

#### lower(string)
```lizard
lower("HELLO")     # "hello"
lower("HeLLo")     # "hello"
```
**Purpose**: Convert string to lowercase

#### split(string, delimiter)
```lizard
split("a,b,c", ",")        # ["a", "b", "c"]
split("hello world", " ")  # ["hello", "world"]
```
**Purpose**: Split string by delimiter into array

#### join(array, delimiter)
```lizard
join(["a", "b", "c"], ",")     # "a,b,c"
join(["hello", "world"], " ")  # "hello world"
```
**Purpose**: Join array elements with delimiter

#### starts_with(string, prefix)
```lizard
starts_with("hello", "he")     # true
starts_with("hello", "wo")     # false
```
**Purpose**: Check if string starts with prefix

#### ends_with(string, suffix)
```lizard
ends_with("hello", "lo")       # true
ends_with("hello", "he")       # false
```
**Purpose**: Check if string ends with suffix

#### includes(string, substring)
```lizard
includes("hello", "ell")       # true
includes("hello", "xyz")       # false
```
**Purpose**: Check if string contains substring

#### slice(string, start, end)
```lizard
slice("hello", 0, 3)   # "hel"
slice("hello", 1, 4)   # "ell"
```
**Purpose**: Extract substring from start to end

#### trim(string)
```lizard
trim("  hello  ")     # "hello"
trim("\thello\n")     # "hello"
```
**Purpose**: Remove leading/trailing whitespace

#### replace(string, find, replace)
```lizard
replace("hello", "l", "L")     # "heLLo"
replace("aaa", "a", "b")       # "bbb"
```
**Purpose**: Replace all occurrences

---

### 4️⃣ List/Array Functions

#### push(list, item)
```lizard
arr = [1, 2, 3]
push(arr, 4)
print arr  # [1, 2, 3, 4]
```
**Purpose**: Add item to end of list

#### pop(list)
```lizard
arr = [1, 2, 3]
item = pop(arr)
print item  # 3
print arr   # [1, 2]
```
**Purpose**: Remove and return last item

#### shift(list)
```lizard
arr = [1, 2, 3]
item = shift(arr)
print item  # 1
print arr   # [2, 3]
```
**Purpose**: Remove and return first item

#### unshift(list, item)
```lizard
arr = [2, 3]
unshift(arr, 1)
print arr  # [1, 2, 3]
```
**Purpose**: Add item to beginning of list

#### reverse(list)
```lizard
arr = [1, 2, 3]
reverse(arr)
print arr  # [3, 2, 1]
```
**Purpose**: Reverse list in place

#### sort(list)
```lizard
arr = [3, 1, 2]
sort(arr)
print arr  # [1, 2, 3]
```
**Purpose**: Sort list in ascending order

#### has(list, item)
```lizard
has([1, 2, 3], 2)      # true
has([1, 2, 3], 5)      # false
```
**Purpose**: Check if list contains item

#### get(list, index)
```lizard
get([10, 20, 30], 0)   # 10
get([10, 20, 30], 2)   # 30
```
**Purpose**: Get item at index

#### remove(list, index)
```lizard
arr = [1, 2, 3]
remove(arr, 1)
print arr  # [1, 3]
```
**Purpose**: Remove item at index

---

### 5️⃣ Math Functions

#### pow(base, exponent)
```lizard
pow(2, 3)      # 8
pow(5, 2)      # 25
```
**Purpose**: Calculate power (base^exponent)

#### sqrt(number)
```lizard
sqrt(16)       # 4
sqrt(2)        # 1.414...
```
**Purpose**: Calculate square root

#### floor(number)
```lizard
floor(3.7)     # 3
floor(3.2)     # 3
```
**Purpose**: Round down to nearest integer

#### ceil(number)
```lizard
ceil(3.2)      # 4
ceil(3.7)      # 4
```
**Purpose**: Round up to nearest integer

#### round(number)
```lizard
round(3.4)     # 3
round(3.5)     # 4
round(3.7)     # 4
```
**Purpose**: Round to nearest integer

#### abs(number)
```lizard
abs(-5)        # 5
abs(3)         # 3
```
**Purpose**: Absolute value

#### min(a, b)
```lizard
min(5, 3)      # 3
min(-10, -5)   # -10
```
**Purpose**: Return smaller value

#### max(a, b)
```lizard
max(5, 3)      # 5
max(-10, -5)   # -5
```
**Purpose**: Return larger value

#### clamp(value, minimum, maximum)
```lizard
clamp(5, 0, 10)    # 5
clamp(-5, 0, 10)   # 0
clamp(15, 0, 10)   # 10
```
**Purpose**: Constrain value between min and max

---

### 6️⃣ Map/Dictionary Functions

#### has(map, key)
```lizard
obj = {"name": "LIZARD", "age": 1}
has(obj, "name")   # true
has(obj, "color")  # false
```
**Purpose**: Check if map has key

#### get(map, key)
```lizard
obj = {"name": "LIZARD"}
get(obj, "name")   # "LIZARD"
```
**Purpose**: Get value by key

#### keys(map)
```lizard
obj = {"a": 1, "b": 2}
keys(obj)  # ["a", "b"]
```
**Purpose**: Get all keys

#### values(map)
```lizard
obj = {"a": 1, "b": 2}
values(obj)  # [1, 2]
```
**Purpose**: Get all values

---

### 7️⃣ Conversion Functions

#### to_text(value)
```lizard
to_text(42)        # "42"
to_text(true)      # "true"
to_text([1,2,3])   # "[1, 2, 3]"
```
**Purpose**: Convert value to string

#### to_number(value)
```lizard
to_number("42")    # 42
to_number("3.14")  # 3.14
to_number("abc")   # Error or 0
```
**Purpose**: Convert value to number

---

## 📊 Library Function Count

| Category | Count | Functions |
|----------|-------|-----------|
| I/O | 3 | say, print, pause |
| Type | 8 | type, length, is_number, is_text, is_list, is_map, is_bool, is_null |
| String | 10 | upper, lower, split, join, starts_with, ends_with, includes, slice, trim, replace |
| Array | 10 | push, pop, shift, unshift, reverse, sort, has, get, remove, length |
| Math | 9 | pow, sqrt, floor, ceil, round, abs, min, max, clamp |
| Map | 4 | has, get, keys, values |
| Conversion | 2 | to_text, to_number |
| **TOTAL** | **46** | **Complete built-in function library** |

---

## 🎯 Quick Reference

### Most Used Functions
```lizard
say "Hello"                    # Output
print value                    # Display value
length(arr)                    # Get size
type(x)                        # Get type
upper("text")                  # String to uppercase
push(list, item)               # Add to list
sqrt(16)                       # Math calculation
to_text(42)                    # Type conversion
```

### String Operations
```lizard
str = "hello world"
str = upper(str)               # "HELLO WORLD"
arr = split(str, " ")          # ["HELLO", "WORLD"]
result = join(arr, "-")        # "HELLO-WORLD"
```

### List Operations
```lizard
arr = [1, 2, 3]
push(arr, 4)                   # [1, 2, 3, 4]
item = pop(arr)                # item = 4
sort(arr)                      # [1, 2, 3]
reverse(arr)                   # [3, 2, 1]
```

### Math Operations
```lizard
result = pow(2, 8)             # 256
result = sqrt(144)             # 12
result = clamp(50, 0, 100)     # 50
```

---

**LIZARD v0.1.0 Standard Library - 46 Built-in Functions**
**Ready for production use!**
