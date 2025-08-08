mod binary_search_tree;
mod avl_tree;

fn main() {
    // let mut tree = binary_search_tree::BSTree::new();

    // tree.insert(10);
    // tree.insert(4);
    // tree.insert(13);
    // tree.insert(3);
    // tree.insert(5);
    // tree.insert(6);
    // tree.insert(12);
    // tree.insert(20);

    // tree.preorder();
    // tree.inorder();
    // tree.postorder();
    // tree.dfs();
    // tree.bfs();
    // tree.bfs_on_levels();
    // if let Some(_) = tree.search(21) {
    //     println!("FOUND!");
    // } else {
    //     println!("NOT FOUND!");
    // }

    // tree.delete(10);
    // tree.bfs_on_levels();


    let mut tree_avl = avl_tree::AVLTree::new();

    // tree_avl.insert(30);
    // tree_avl.insert(10);
    // tree_avl.insert(20);
    // tree_avl.insert(40);
    // tree_avl.bfs_print_on_levels();

    tree_avl.insert_recursive_wrapper(20);
    tree_avl.insert_recursive_wrapper(1);
    tree_avl.insert_recursive_wrapper(25);
    tree_avl.insert_recursive_wrapper(22);
    tree_avl.insert_recursive_wrapper(26);
    tree_avl.insert_recursive_wrapper(21);
    tree_avl.insert_recursive_wrapper(23);
    tree_avl.bfs_print_on_levels();
    println!();

    // let x = tree_avl.root.clone().unwrap();
    // tree_avl.root = Some(tree_avl.LL(x));
    // tree_avl.bfs_print_on_levels();
    
    // let y = tree_avl.root.clone().unwrap();
    // tree_avl.root = Some(tree_avl.RR(y));
    // tree_avl.bfs_print_on_levels();

    // let z = tree_avl.root.clone().unwrap();
    // tree_avl.root = Some(tree_avl.LR(z));
    // tree_avl.bfs_print_on_levels();

    let p = tree_avl.root.clone().unwrap();
    tree_avl.root = Some(tree_avl.RL(p));
    tree_avl.bfs_print_on_levels();
}
