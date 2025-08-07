mod binary_search_tree;

fn main() {
    let mut tree = binary_search_tree::BSTree::new();

    tree.insert(10);
    tree.insert(4);
    tree.insert(13);
    tree.insert(3);
    tree.insert(5);
    tree.insert(6);
    tree.insert(12);
    tree.insert(20);

    tree.preorder();
    tree.inorder();
    tree.postorder();
    tree.dfs();
    tree.bfs();
    tree.bfs_on_levels();
    if let Some(_) = tree.search(21) {
        println!("FOUND!");
    } else {
        println!("NOT FOUND!");
    }

    tree.delete(10);
    tree.bfs_on_levels();
}
