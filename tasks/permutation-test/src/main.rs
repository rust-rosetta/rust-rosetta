fn main()
{
    let treatment = vec![85, 88, 75, 66, 25, 29, 83, 39, 97];
    let control = vec![68, 41, 10, 49, 16, 65, 32, 92, 28, 98];

    let mut data_set = control.clone();
    data_set.extend_from_slice(&treatment);

    let greater = combinations(treatment.iter().sum(), treatment.len() as i64, &data_set) as f64;
    let lesser = combinations(control.iter().sum(), control.len() as i64, &data_set) as f64;
    let total = binomial(data_set.len() as i64, treatment.len() as i64) as f64;

    println!("<= : {}%", (lesser / total * 100.0));
    println!(" > : {}%", (greater / total * 100.0));
}

fn factorial(x: i64) -> i64
{
    let mut product = 1;
    for a in 1..(x + 1)
    {
        product = product * a;
    }
    product
}

fn binomial(n: i64, k: i64) -> i64
{
    let numerator = factorial(n);
    let denominator = factorial(k) * factorial(n - k);
    numerator / denominator
}

fn combinations(total: i64, number: i64, data: &[i64]) -> i64
{
    if total < 0
    {
        return binomial(data.len() as i64, number);
    }

    if number == 0
    {
        return 0;
    }

    if number > data.len() as i64
    {
        return 0;
    }

    if number == data.len() as i64
    {
        if total < data.iter().sum()
        {
            return 1;
        }
        else
        {
            return 0;
        }
    }

    let tail = &data[1..];
    return combinations(total - data[0], number - 1, &tail) + combinations(total, number, &tail);
}
