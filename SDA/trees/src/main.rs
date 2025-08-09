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
    tree_avl.insert_recursive_wrapper(10);
    tree_avl.insert_recursive_wrapper(25);
    tree_avl.insert_recursive_wrapper(24);
    tree_avl.insert_recursive_wrapper(30);
    tree_avl.insert_recursive_wrapper(29);
    tree_avl.insert_recursive_wrapper(31);
    tree_avl.bfs_print_on_levels();
    println!();

    tree_avl.delete(30);
    tree_avl.delete(29);
    tree_avl.delete(31);
    tree_avl.bfs_print_on_levels();
    println!();
}
