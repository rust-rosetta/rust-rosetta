struct IIRFilter<'f>(&'f [f32], &'f [f32]);

impl<'f> IIRFilter<'f> {
    pub fn with_coefficients(a: &'f [f32], b: &'f [f32]) -> IIRFilter<'f> {
        IIRFilter(a, b)
    }

    // Performs the calculation as an iterator chain.
    pub fn apply<I: Iterator<Item = &'f f32> + 'f>(
        &self,
        samples: I,
    ) -> impl Iterator<Item = f32> + 'f {
        // Name some things for readability
        let a_coeff = self.0;
        let b_coeff = self.1;

        let mut prev_results = Vec::<f32>::new();
        let mut prev_samples = Vec::<f32>::new();

        // The actual calculation, done one number at a time
        samples.enumerate() // (i, sample[i])
            .map(move |(i, sample)| { // for each sample, apply this function
                prev_samples.push(*sample);
                prev_results.push(0f32); // the initial version of the previous result

                let sum_b: f32 = b_coeff.iter() // for each coefficient in b
                    .enumerate() // (j, b_coeff[j])
                    .map(|(j, c)| { // calculate the weight of the coefficient
                        if i >= j {
                            (*c) * prev_samples[i-j]
                        } else {
                            0f32
                        }
                    })
                    .sum(); // add them all together

                let sum_a: f32 = a_coeff.iter() // for each coefficient in a
                    .enumerate() // (j, a_coeff[j])
                    .map(|(j, c)| { // calculate the weight of the coefficient
                        if i >= j {
                            (*c) * prev_results[i-j]
                        } else {
                            0f32
                        }
                    })
                    .sum(); // add them all together

                // perform the final calculation
                let result = (sum_b - sum_a) / a_coeff[0];

                // update the previous result for the next iteration
                prev_results[i] = result;

                // return the current result in this iteration
                result
            }
        )
    }
}

fn main() {
    let a: &[f32] = &[1.00000000, -2.77555756e-16, 3.33333333e-01, -1.85037171e-17];
    let b: &[f32] = &[0.16666667, 0.5, 0.5, 0.16666667];

    let samples: Vec<f32> = vec![
        -0.917843918645,
        0.141984778794,
        1.20536903482,
        0.190286794412,
        -0.662370894973,
        -1.00700480494,
        -0.404707073677,
        0.800482325044,
        0.743500089861,
        1.01090520172,
        0.741527555207,
        0.277841675195,
        0.400833448236,
        -0.2085993586,
        -0.172842103641,
        -0.134316096293,
        0.0259303398477,
        0.490105989562,
        0.549391221511,
        0.9047198589,
    ];

    for (i, result) in IIRFilter::with_coefficients(a, b)
        .apply(samples.iter())
        .enumerate()
    {
        print!("{:.8}", result);
        if (i + 1) % 5 != 0 {
            print!(", ");
        } else {
            println!();
        }
    }
    println!();
}

#[test]
fn test() {
    use std::cmp::Ordering;

    let a: &[f32] = &[1.00000000, -2.77555756e-16, 3.33333333e-01, -1.85037171e-17];
    let b: &[f32] = &[0.16666667, 0.5, 0.5, 0.16666667];

    let samples: Vec<f32> = vec![
        -0.917843918645,
        0.141984778794,
        1.20536903482,
        0.190286794412,
        -0.662370894973,
        -1.00700480494,
        -0.404707073677,
        0.800482325044,
        0.743500089861,
        1.01090520172,
        0.741527555207,
        0.277841675195,
        0.400833448236,
        -0.2085993586,
        -0.172842103641,
        -0.134316096293,
        0.0259303398477,
        0.490105989562,
        0.549391221511,
        0.9047198589,
    ];

    let expected = vec![
        -0.15297399,
        -0.43525785,
        -0.13604343,
        0.69750333,
        0.65644467,
        -0.43548250,
        -1.08923948,
        -0.53767651,
        0.51705003,
        1.05224979,
        0.96185434,
        0.69568992,
        0.42435625,
        0.19626230,
        -0.02783510,
        -0.21172196,
        -0.17474557,
        0.06925842,
        0.38544586,
        0.65177077,
    ]
    .into_iter();

    let actual = IIRFilter::with_coefficients(a, b).apply(samples.iter());

    let failed = actual
        .zip(expected)
        .inspect(|(act, exp)| {
            eprintln!("{} <=> {}: {:?}", act, exp, act.partial_cmp(&exp));
        })
        .any(|(act, exp)| act.partial_cmp(&exp) != Some(Ordering::Equal));
    assert!(!failed);
}
