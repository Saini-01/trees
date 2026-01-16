### THIS IS VIBECODED

# Random Tree Generator

A Rust program to generate random trees for practicing recursive DFS algorithms.

## Features

- Generate random **general trees** (nodes can have any number of children)
- Generate random **binary trees** (each node has at most 2 children)
- Specify the number of nodes
- Visual tree printing
- Built-in DFS traversal implementations (preorder, inorder, postorder)

## Usage

### Build the project

```bash
cargo build --release
```

### Run with default settings (10 nodes, general tree)

```bash
cargo run
```

### Specify number of nodes

```bash
cargo run -- 15
```

### Generate a binary tree

```bash
cargo run -- 10 binary
# or
cargo run -- 10 b
```

### Generate a general tree

```bash
cargo run -- 10 general
# or just
cargo run -- 10
```

## Examples

```bash
# Generate a binary tree with 7 nodes
cargo run -- 7 binary

# Generate a general tree with 20 nodes
cargo run -- 20 general
```

## Output

The program will:
1. Display the tree structure in a visual format
2. Show the results of DFS traversals:
   - **Preorder**: Visit root, then children
   - **Inorder**: Visit left subtree, root, right subtree (for binary trees)
   - **Postorder**: Visit children, then root

## Practice Exercises

Try implementing your own DFS functions:
- Count the number of nodes
- Find the maximum depth
- Find the sum of all node values
- Find a specific value in the tree
- Print all leaf nodes
