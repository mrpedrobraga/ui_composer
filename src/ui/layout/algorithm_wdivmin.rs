/// Divides a total number of shares for n elements, where the elements can be biased with a weight, or have a minimum share.
/// It does three dynamic allocations, and runs in O(n * log(n)).
pub fn wdivmin(
    total: f64,
    el_weights: &[f64],
    el_minima: &[f64]
) -> Vec<f64> {
    let el_count = el_weights.len();
    // Imagine a container with size x on the lim x -> Infinity.
    // In such a container, minimum size doesn't matter.
    // If you shrink this container, eventually *some* element will hit its
    // minimum size. The elements need to be addressed in the order they hit the minimum size.
    let mut indices = (0..el_count).collect::<Vec<usize>>();
    indices.sort_by(| &i_a, &i_b | {
        let ratio_a = el_minima[i_a] * el_weights[i_a];
        let ratio_b = el_minima[i_b] * el_weights[i_b];
        ratio_b.partial_cmp(&ratio_a).unwrap()
    });
    let total_weight_count = el_weights.iter().sum::<f64>();
    // After that, we know the characteristics of which elements
    // will be taken off the total, so we can pre-calculate the sums of the weights
    // of the remaining objects.
    let remaining_weight_sums = indices.iter()
        .scan(total_weight_count, |acc, i| {
           let result = Some(*acc);
           *acc -= el_weights[*i];
           result
        });
    // Then, each element will calculate how much they take from the total
    // which will either be their minimum size, or a calculated fraction of the
    // remaining space;
    let sizes = indices.iter()
        .zip(remaining_weight_sums)
        .scan(total, |space_left, (i, remaining_weight_sum)| {
            let el_share_count = *space_left * el_weights[*i] / remaining_weight_sum;
            let size = el_minima[*i].max(el_share_count);
            *space_left -= size;
            Some(size)
        });
    // On the end, you need to return the sizes in the original order.
    let mut result = vec![0.0; el_count];
    for (index, size) in indices.iter().zip(sizes) {
        result[*index] = size;
    }
    result
}