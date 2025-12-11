use extendr_api::prelude::*;
use pca_tree::PcaTree;

/// Run PCA + MST on a list of sequences.
/// 
/// @param seqs Character vector of DNA sequences
/// @param k Number of PCA components
/// @return A list with:
///   - coords: matrix of PCA coordinates
///   - edges: list of (parent, child, dist)
#[extendr]
fn pca_tree_fit(seqs: Vec<String>, k: usize) -> List {
    // Construct and run the Rust model
    let model = PcaTree::new(seqs, k).unwrap_or_else(|e| panic!("Failed to create PcaTree: {e}"));

    // --- Extract PCA coordinates ---
    let coords = model.coords();
    let n = coords.nrows();
    let d = coords.ncols();

    // We know ndarray is row-major, R wants column-major
    let slice = coords.as_slice().unwrap();

    // Make R matrix
    let mut mat = RMatrix::new_matrix(n, d, |_,_| 0.0 );
    for i in 0..n {
        for j in 0..d {
            mat[[i, j]] = slice[i * d + j] as f64;
        }
    }

    // --- Extract MST edges ---
    let edges_r = edges_to_r_df( model.tree() );

    list!(
        coords = mat,
        edges = edges_r
    )
}

fn edges_to_r_df(edges: &Vec<(usize, usize, f32)>) -> List {
    let parents: Vec<i32> = edges.iter().map(|e| e.0 as i32).collect();
    let children: Vec<i32> = edges.iter().map(|e| e.1 as i32).collect();
    let dists: Vec<f64> = edges.iter().map(|e| e.2 as f64).collect();

    list!(
        parent = parents,
        child = children,
        dist = dists
    )
}



extendr_module! {
    mod pcaTreeR;

    fn pca_tree_fit;
}