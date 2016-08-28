use std::cmp::max;
use std::iter::repeat;

/// This struct is used to store our items that we want in our knap-sack.
///
#[derive(Copy, Clone)]
struct Want<'a> {
    name: &'a str,
    weight: usize,
    value: usize,
}

/// Global, immutable allocation of our items. This is so we can reference
/// this in multiple functions.
const ITEMS: &'static [Want<'static>] = &[Want {
                                              name: "map",
                                              weight: 9,
                                              value: 150,
                                          },
                                          Want {
                                              name: "compass",
                                              weight: 13,
                                              value: 35,
                                          },
                                          Want {
                                              name: "water",
                                              weight: 153,
                                              value: 200,
                                          },
                                          Want {
                                              name: "sandwich",
                                              weight: 50,
                                              value: 160,
                                          },
                                          Want {
                                              name: "glucose",
                                              weight: 15,
                                              value: 60,
                                          },
                                          Want {
                                              name: "tin",
                                              weight: 68,
                                              value: 45,
                                          },
                                          Want {
                                              name: "banana",
                                              weight: 27,
                                              value: 60,
                                          },
                                          Want {
                                              name: "apple",
                                              weight: 39,
                                              value: 40,
                                          },
                                          Want {
                                              name: "cheese",
                                              weight: 23,
                                              value: 30,
                                          },
                                          Want {
                                              name: "beer",
                                              weight: 52,
                                              value: 10,
                                          },
                                          Want {
                                              name: "suntancream",
                                              weight: 11,
                                              value: 70,
                                          },
                                          Want {
                                              name: "camera",
                                              weight: 32,
                                              value: 30,
                                          },
                                          Want {
                                              name: "T-shirt",
                                              weight: 24,
                                              value: 15,
                                          },
                                          Want {
                                              name: "trousers",
                                              weight: 48,
                                              value: 10,
                                          },
                                          Want {
                                              name: "umbrella",
                                              weight: 73,
                                              value: 40,
                                          },
                                          Want {
                                              name: "waterproof trousers",
                                              weight: 42,
                                              value: 70,
                                          },
                                          Want {
                                              name: "waterproof overclothes",
                                              weight: 43,
                                              value: 75,
                                          },
                                          Want {
                                              name: "note-case",
                                              weight: 22,
                                              value: 80,
                                          },
                                          Want {
                                              name: "sunglasses",
                                              weight: 7,
                                              value: 20,
                                          },
                                          Want {
                                              name: "towel",
                                              weight: 18,
                                              value: 12,
                                          },
                                          Want {
                                              name: "socks",
                                              weight: 4,
                                              value: 50,
                                          },
                                          Want {
                                              name: "book",
                                              weight: 30,
                                              value: 10,
                                          }];

/// This is a bottom-up dynamic programming solution to the 0-1 knap-sack problem.
///
/// ```
/// maximize value
/// subject to weights <= max_weight
/// ```
fn knap_01_dp<'a>(xs: &[Want<'a>], max_weight: usize) -> Vec<Want<'a>> {

    // Save this value, so we don't have to make repeated calls.
    let xs_len = xs.len();

    // Imagine we wrote a recursive function(item, max_weight) that returns a
    // usize corresponding to the maximum cumulative value by considering a
    // subset of items such that the combined weight <= max_weight.
    //
    // fn best_value(item: usize, max_weight: usize) -> usize{
    //     if item == 0 {
    //         return 0;
    //     }
    //     if xs[item - 1].weight > max_weight {
    //         return best_value(item - 1, max_weight, xs);
    //     }
    //     return max(best_value(item - 1, max_weight, xs),
    //                best_value(item - 1, max_weight - xs[item - 1].weight, xs)
    //                + xs[item - 1].value);
    //     }
    //
    // best_value(xs_len, max_weight) is equal to the maximum value that we
    // can add to the bag.
    //
    // The problem with using this function is that it performs redudant
    // calculations.
    //
    // The dynamic programming solution is to precompute all of the values we
    // need and put them into a 2D array.
    //
    // In a similar vein, the top-down solution would be to memoize the
    // function then compute the results on demand.

    let zero_vec: Vec<usize> = repeat(0).take(max_weight + 1).collect();
    let mut best_value: Vec<Vec<usize>> = repeat(zero_vec)
        .take(xs_len + 1)
        .collect();

    // loop over the items
    for i in 0..xs_len {
        // loop over the weights
        for w in 1..(max_weight + 1) {
            // do we have room in our knapsack?
            if xs[i].weight > w {
                // if we don't, then we'll say that the value doesn't change
                // when considering this item
                best_value[i + 1][w] = best_value[i][w];
            } else {
                // if we do, then we have to see if the value we gain by adding
                // the item, given the weight, is better than not adding the item
                best_value[i + 1][w] = max(best_value[i][w],
                                           best_value[i][w - xs[i].weight] + xs[i].value);
            }
        }
    }

    // a variable representing the weight left in the bag
    let mut left_weight = max_weight;

    // a possibly over-allocated dynamically sized vector to push results to
    let mut result = Vec::with_capacity(xs_len);

    // we built up the solution space through a forward pass over the data,
    // now we have to traverse backwards to get the solution
    for i in (1..xs_len + 1).rev() {
        // We can check if an item should be added to the knap-sack by comparing
        // best_value with and without this item. If best_value added this
        // item then so should we.
        if best_value[i][left_weight] != best_value[i - 1][left_weight] {
            result.push(xs[i - 1]);
            // we remove the weight of the object from the remaining weight
            // we can add to the bag
            left_weight -= xs[i - 1].weight;
        }
    }

    result
}

fn main() {
    let xs = knap_01_dp(ITEMS, 400);

    // Print the items. We have to reverse the order because we solved the
    // problem backward.
    for i in xs.iter().rev() {
        println!("Item: {}, Weight: {}, Value: {}", i.name, i.weight, i.value);
    }

    // Print the sum of weights.
    let weights = xs.iter().fold(0, |a, &b| a + b.weight);
    println!("Total Weight: {}", weights);

    // Print the sum of the values.
    let values = xs.iter().fold(0, |a, &b| a + b.value);
    println!("Total Value: {}", values);

}

#[cfg(test)]
mod tests {
    use super::{ITEMS, knap_01_dp};

    #[test]
    fn test_dp_results() {
        let dp_results = knap_01_dp(ITEMS, 400);
        let dp_weights = dp_results.iter().fold(0, |a, &b| a + b.weight);
        let dp_values = dp_results.iter().fold(0, |a, &b| a + b.value);
        assert_eq!(dp_weights, 396);
        assert_eq!(dp_values, 1030);
    }
}
