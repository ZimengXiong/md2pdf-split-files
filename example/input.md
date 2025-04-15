\newfile

# CB U3.3 & U3.4: Mathematical Expressions and Strings

## Algorithims

An algorithm is a set of instructions that accomplish a task. Algorithims can be expressed multimodally. Programming languages are languages that construct algorithims through sequencing, selection and iteration.

## Sequencing, Selection, Iteration

Sequencing: executing statements in order, logically Selection: control flow based on making decisions, executing different "arms" conditionally Iteration: repeatedly conditionally continuing the sequencing process

## Operations

A code statement is part of a program that expresses an action to be carried out. Expressions are evaluated and treated like a single value. Evaluations and sequential statements follow a set order of operations to be executed.

Instructions are executed line by line. Variables store the last assigned number.

## Mathematical Expreessions

Basic math operators are implemented in common programming languages.

## Strings

Strings are an ordered sequence of characters. String procedures:

```
len(str)
concact(str1,str2)
substring(str1,start,length)
```

\newfile

# CB U3.5: Boolean Expression

## Boolean Values

Booleans are always evalulated to `True` or `False` statements. Booleans variables are either true or false. Relational operators compare two values. `MOD 2 == 0` -> number is even

## Operators

Boolean expressions can be combined with the `and|or` operators.

```py
A and B # true only when both are true
A or B # true when atleast one is true
```

Boolean expr can be negated with `not`

```py
not False == True
```

\newfile

# CB U3.6 & U3.7 Conditionals & Nested Conditionals

## Conditionals

Algorithms are a finite set of instructions that accomplish a specific task Selection allows parts of an algorithm to execute conditionally based on a condition being satisfied.

They affect the flow of control by executing statements conditionally based on the value of a boolean expression.

```py
if <expr->bool>:
    # executes if condition is true
else:
    # executes if condition is false

if <expr>:
    # same as above
# continues program
```

## Nested Conditionals

Nested conditional statements contain conditional statements within conditional statements

```py
if <expr>:
else:
    if <expr2>:
        # executes if expr is false and expr2
    else:
        # executes if expr is false and expr2 is also false
```

Allows many conditions to be taken into account. \newfile

# CB U3.8 & U3.9: Iteration & Developing Algorithm

## Iteration

Iteration is the repeating of a portion of a algorithm until a condition is met or for a specific # of times.

Iteration repeatedly does something to get closer to the goal until the goal is reached.

Iteration changes the control flow by repeating a set of statements continuously.

`REPEAT UNTIL <expr>` will not execute if the condition is true initially.

## Developing algorithms

Use conditional statements to achieve the desired output from all combinations of possible inputs. Algorithms may be written in various styles and still accomplish the same result. They can be created from existing algorithims or a combination of both existing and new algorithims.

Similar algorithms can produce different outputs.

Existing algorithms can be used as building blocks for creating new algorithms -> abstraction, reducing development time. \newfile

# CB U3.10 & U 3.11: Lists and Binary Search

## Lists

Lists are a collection of data of the same type. Lists are traversed by iteration and loops. Common list algorithims include determining the min or max of a list, and the sum and mean.

List algorithims consist of iteration and operations on each individual element:

```py
keep_track = 0
for x in list:
    #... do operation on x, keep track by modifying another variable

#... do processing on keep_track (or not)
return keep_track
```

## Indexing

Linear and sequential search algorithims check each element in a list until the target value is found or the list has been exhausted. Lists are indexed starting from 1 until length of the list.

```py
n = ['a','b','c']
n[1] = 'a'
n[2] = 'b'
n[3] = 'c'
```

Search on list

```py
for n in list:
    if n==target:
        # exit or etc.
```

## Binary Search

Binary search starts at the middle of sorted set of data and eliminates half of the data repeatedly until the target value is found

Data must be sorted.

Binary search is faster than linear search when sorting the list is negligible.

A list of length $2^n$ will take $n$ operations to sort.

Binary search also works on sorted words. \newfile

# CB U3.12 & U3.13: Calling Procedures & Developing Procedures

## Procedures

A procedure is a block of code that is ran when it is called, taking in 0 or more arguments.

Procedures can be expressions by returning values.

Procedures help abstract and eliminate repetitive code used in multiple places, allowing sharing of code, and helps improve code readability.

```rust
fn procedure_name(args: i32) -> i32{
    // process arguments...
    print!(args);

    // return value
    36
}

println!(procedure_name(2)) //prints 2, 36, as the procedure is an expression
```

Modularity: the subdivision of a computer program into subprograms \newfile

# CB U4: The Internet, Fault Tolerance, Parallel and Distributed Computing

## Internet

The internet responded to computer demands of interconnection. A computer network is a group of interconnected computers capable of sending/receiving data.

Computers send and recieve data in packets.

A packet is a small amt of data sent over a network.

Packets are `routed` through `paths` between two computers.

Bandwidth is the max amount of data that can be sent in a computer network in a fixed time.

### Networks

Network interface cards allow computers to access the internet. Each computer/NIC is uniquely identified by its MAC Addr.

A packet contains metadata for routing information, including its origin and destination.

LAN: A local area network of a few computers (1-100) WAN: the internet links millions of systems together

The internet is scalable because it is decentralized.

### Layers

1. Transport
   1. TCP/UDP faciliate the sending and recieving of packets across the internet.
2. Internet
   1. IP Addr. identifies a computer on a network
   2. Targets
      1. Unicast-specific device
      2. Multicast-range of IPs
      3. Broadcast-all devices
3. Application Layer
   1. DNS resolves domain names into IP addresses to allow communications with servers
   2. HTTP allows sending of website data to webbrowsers
   3. HTTPS enables secure sending of encrypted data

## Fault Tolerance

Fault Tolerance/Redundacy is acheived with multiple ways that result in the same goal.

The internet's many routing options is an example of fault tolerance, it allows the internet to still work even if some devices are down, making it more accessible.

Networks are a connection of many interconnected devices, allowing many routes for data to be transfered.

## Parallel Computing.

Parallel processes involve a parallel and sequential portion.

Parallel computing allows algorithms to scale more efficiently and easily.

Distributed computing allows a large network of computers to solve complex problems by joining resources.

Parallel computing has diminishing returns because it depends on some sequential portions.

Parllel computing: doing computations in parallel on one computer.

Distributed computing: doing computations accross many computers on a network, and joining the results.
