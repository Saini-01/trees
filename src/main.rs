use rand::Rng;

#[derive(Debug, Clone)]
struct TreeNode {
    val: i32,
    children: Vec<Box<TreeNode>>,
}

#[derive(Debug, Clone)]
struct BinaryTreeNode {
    val: i32,
    left: Option<Box<BinaryTreeNode>>,
    right: Option<Box<BinaryTreeNode>>,
}

// General tree generator
struct TreeGenerator {
    rng: rand::rngs::ThreadRng,
}

impl TreeGenerator {
    fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }

    // Generate a random general tree with n nodes
    fn generate_tree(&mut self, n: usize) -> Option<Box<TreeNode>> {
        if n == 0 {
            return None;
        }

        self.generate_tree_recursive(1, n)
    }

    fn generate_tree_recursive(&mut self, start_val: i32, n: usize) -> Option<Box<TreeNode>> {
        if n == 0 {
            return None;
        }

        let root = Box::new(TreeNode {
            val: start_val,
            children: Vec::new(),
        });

        if n == 1 {
            return Some(root);
        }

        // Randomly decide how many children this node should have (max 5)
        let max_children = (n - 1).min(5);
        let num_children = if max_children > 0 {
            self.rng.gen_range(1..=max_children)
        } else {
            0
        };

        if num_children == 0 {
            return Some(root);
        }

        // Distribute remaining nodes (n - 1) among children
        let remaining = n - 1;
        let mut children = Vec::new();
        let mut current_val = start_val + 1;
        let mut remaining_nodes = remaining;

        // Distribute remaining nodes among children
        // Ensure at least one child gets nodes, and all nodes are used
        for i in 0..num_children {
            let is_last = i == num_children - 1;
            let child_size = if is_last {
                // Last child gets all remaining nodes
                remaining_nodes
            } else if remaining_nodes > 0 {
                // Randomly assign nodes, but leave at least 1 for remaining children
                let max_for_this = remaining_nodes.saturating_sub(num_children - i - 1);
                if max_for_this > 0 {
                    self.rng.gen_range(1..=max_for_this)
                } else {
                    0
                }
            } else {
                0
            };

            if child_size > 0 {
                if let Some(child) = self.generate_tree_recursive(current_val, child_size) {
                    current_val += child_size as i32;
                    remaining_nodes -= child_size;
                    children.push(child);
                }
            }
        }

        Some(Box::new(TreeNode {
            val: start_val,
            children,
        }))
    }

    // Generate a random binary tree with n nodes
    fn generate_binary_tree(&mut self, n: usize) -> Option<Box<BinaryTreeNode>> {
        if n == 0 {
            return None;
        }

        self.generate_binary_tree_recursive(1, n)
    }

    fn generate_binary_tree_recursive(&mut self, start_val: i32, n: usize) -> Option<Box<BinaryTreeNode>> {
        if n == 0 {
            return None;
        }

        let root = Box::new(BinaryTreeNode {
            val: start_val,
            left: None,
            right: None,
        });

        if n == 1 {
            return Some(root);
        }

        // Randomly distribute remaining nodes between left and right subtrees
        let remaining = n - 1;
        let left_size = if remaining > 0 {
            self.rng.gen_range(0..=remaining)
        } else {
            0
        };
        let right_size = remaining - left_size;

        let left = if left_size > 0 {
            self.generate_binary_tree_recursive(start_val + 1, left_size)
        } else {
            None
        };

        let right = if right_size > 0 {
            self.generate_binary_tree_recursive(start_val + 1 + left_size as i32, right_size)
        } else {
            None
        };

        Some(Box::new(BinaryTreeNode {
            val: start_val,
            left,
            right,
        }))
    }
}

// Tree visualization and DFS implementations
impl TreeNode {
    fn print_tree(&self, prefix: &str, is_last: bool) {
        println!("{}{}{}", prefix, if is_last { "└── " } else { "├── " }, self.val);
        let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
        for (i, child) in self.children.iter().enumerate() {
            child.print_tree(&new_prefix, i == self.children.len() - 1);
        }
    }

    fn dfs_preorder(&self, result: &mut Vec<i32>) {
        result.push(self.val);
        for child in &self.children {
            child.dfs_preorder(result);
        }
    }

    fn dfs_postorder(&self, result: &mut Vec<i32>) {
        for child in &self.children {
            child.dfs_postorder(result);
        }
        result.push(self.val);
    }

    fn dfs_inorder(&self, result: &mut Vec<i32>) {
        if !self.children.is_empty() {
            self.children[0].dfs_inorder(result);
        }
        result.push(self.val);
        for child in self.children.iter().skip(1) {
            child.dfs_inorder(result);
        }
    }
}

impl BinaryTreeNode {
    fn print_tree(&self, prefix: &str, is_left: bool) {
        if let Some(right) = &self.right {
            right.print_tree(
                &format!("{}{}", prefix, if is_left { "│   " } else { "    " }),
                false,
            );
        }
        println!("{}{}{}", prefix, if is_left { "└── " } else { "┌── " }, self.val);
        if let Some(left) = &self.left {
            left.print_tree(
                &format!("{}{}", prefix, if is_left { "    " } else { "│   " }),
                true,
            );
        }
    }

    fn dfs_preorder(&self, result: &mut Vec<i32>) {
        result.push(self.val);
        if let Some(left) = &self.left {
            left.dfs_preorder(result);
        }
        if let Some(right) = &self.right {
            right.dfs_preorder(result);
        }
    }

    fn dfs_inorder(&self, result: &mut Vec<i32>) {
        if let Some(left) = &self.left {
            left.dfs_inorder(result);
        }
        result.push(self.val);
        if let Some(right) = &self.right {
            right.dfs_inorder(result);
        }
    }

    fn dfs_postorder(&self, result: &mut Vec<i32>) {
        if let Some(left) = &self.left {
            left.dfs_postorder(result);
        }
        if let Some(right) = &self.right {
            right.dfs_postorder(result);
        }
        result.push(self.val);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    let num_nodes = if args.len() > 1 {
        args[1].parse().unwrap_or(10)
    } else {
        10
    };

    let tree_type = if args.len() > 2 {
        args[2].as_str()
    } else {
        "general"
    };

    let mut generator = TreeGenerator::new();

    match tree_type {
        "binary" | "b" => {
            println!("Generating random binary tree with {} nodes:\n", num_nodes);
            if let Some(root) = generator.generate_binary_tree(num_nodes) {
                root.print_tree("", false);
                
                println!("\n--- DFS Traversals ---");
                let mut preorder = Vec::new();
                root.dfs_preorder(&mut preorder);
                println!("Preorder:  {:?}", preorder);
                
                let mut inorder = Vec::new();
                root.dfs_inorder(&mut inorder);
                println!("Inorder:   {:?}", inorder);
                
                let mut postorder = Vec::new();
                root.dfs_postorder(&mut postorder);
                println!("Postorder: {:?}", postorder);
            }
        }
        _ => {
            println!("Generating random general tree with {} nodes:\n", num_nodes);
            if let Some(root) = generator.generate_tree(num_nodes) {
                root.print_tree("", true);
                
                println!("\n--- DFS Traversals ---");
                let mut preorder = Vec::new();
                root.dfs_preorder(&mut preorder);
                println!("Preorder:  {:?}", preorder);
                
                let mut postorder = Vec::new();
                root.dfs_postorder(&mut postorder);
                println!("Postorder: {:?}", postorder);
                
                let mut inorder = Vec::new();
                root.dfs_inorder(&mut inorder);
                println!("Inorder:   {:?}", inorder);
            }
        }
    }
}
