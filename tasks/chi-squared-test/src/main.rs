use statrs::function::gamma::gamma_li;

fn chi_distance(dataset: &[u32]) -> f64 {
    let expected = f64::from(dataset.iter().sum::<u32>()) / dataset.len() as f64;
    dataset
        .iter()
        .fold(0., |acc, &elt| acc + (elt as f64 - expected).powf(2.))
        / expected
}

fn chi2_probability(dof: f64, distance: f64) -> f64 {
    1. - gamma_li(dof * 0.5, distance * 0.5)
}

fn chi2_uniform(dataset: &[u32], significance: f64) -> bool {
    let d = chi_distance(&dataset);
    chi2_probability(dataset.len() as f64 - 1., d) > significance
}

fn main() {
    let dsets = vec![
        vec![199809, 200665, 199607, 200270, 199649],
        vec![522573, 244456, 139979, 71531, 21461],
    ];

    for ds in dsets {
        println!("Data set: {:?}", ds);
        let d = chi_distance(&ds);
        print!("Distance: {:.6} ", d);
        print!(
            "Chi2 probability: {:.6} ",
            chi2_probability(ds.len() as f64 - 1., d)
        );
        print!("Uniform? {}\n", chi2_uniform(&ds, 0.05));
    }
}
