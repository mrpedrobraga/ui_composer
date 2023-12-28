pub fn weighted_division_with_minima(total_share: f64, el_weights: Vec<f64>, el_minima: Vec<f64>) -> Vec<f64> {
    let mut indices = (0..el_weights.len()).collect::<Vec<usize>>();
    indices
        .sort_by(|&i_a, &i_b| (el_minima[i_b] / el_weights[i_b])
            .partial_cmp(&(el_minima[i_a] / el_weights[i_a])).unwrap()
        );
    
    let wsum = el_weights.iter().sum::<f64>();
    
    let rsizes: Vec<_> = indices.iter()
        .scan(wsum, |acc, i| {
            let result = Some(*acc);
            *acc -= el_weights[*i];
            result
        })
        .collect();

    let sizes: Vec<_> = indices.iter()
        .scan((total_share, 0), |(space_left, it_index), i| {
            let proportion = el_weights[*i] * *space_left / rsizes[*it_index];
            let size = el_minima[*i].max(proportion);
            *space_left -= size;
            *it_index += 1;
            Some(size)
        })
        .collect();
    
    indices.iter()
        .map(|i| { sizes[*i] })
        .collect()
}