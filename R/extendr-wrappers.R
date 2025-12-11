#' Run PCA + MST analysis on sequences
#'
#' @param seqs Character vector of DNA sequences
#' @param k Number of PCA components
#' @return A list with coords (matrix) and edges (data.frame)
#' @export
pca_tree <- function(seqs, k = 3) {

    # --- CALL THE RUST FUNCTION CORRECTLY ---
    res <- pca_tree_fit(seqs, k)

    edges <- do.call(
        rbind,
        lapply(res$edges, as.data.frame)
    )

    list(
        coords = res$coords,
        edges  = edges
    )
}
