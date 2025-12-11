# pcaTreeR

High-performance PCA + MST clonal geometry analysis for BCR/IG sequences using Rust.

`pcaTreeR` provides an R interface to the Rust crate `pca_tree`, enabling extremely fast
PCA embeddings and minimum-spanning-tree (MST) geometry analysis for antibody repertoires.

## Features

- Fast PCA embeddings using Rust + ndarray
- MST topology extraction for clonal structure exploration
- Support for thousands to tens of thousands of sequences
- Seamless R interface via extendr
- Optional Rust-side plotting (PNG output)
- Ideal for exploratory immunoglobulin lineage analysis

## Installation

You need a working Rust toolchain (Rust >= 1.70).

```r
# Install dependencies
install.packages(c("devtools", "rextendr"))

# Clone the repository
# git clone https://github.com/stela2502/pcaTreeR.git

# Install locally
devtools::install("pcaTreeR")
```

## Example

```r
library(pcaTreeR)

seqs <- readLines("example_sequences.txt")

res <- pca_tree(seqs, k = 3)

coords <- res$coords
edges  <- res$edges

print(head(coords))
print(head(edges))
```

## Plotting (Rust-based)

```r
pca_tree_plot_pca(seqs, 3, "pca.png")
pca_tree_plot_mst(seqs, 3, "mst.png")
```

## Rust Integration

This package wraps the Rust crate:

https://github.com/stela2502/pca_tree

The Rust engine performs:

- sequence encoding
- PCA computation
- MST construction
- optional plotting

## License

MIT License
