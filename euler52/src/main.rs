fn main() {
    for x in 100000..=999999 {
        if is_permuted(x) {
            println!("{}", x);
            break;
        }
    }
}

fn is_permuted(x: u32) -> bool {
    // 拆成6个数字
    let vx: Vec<u32> = x
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    for i in 2..=6 {
        let m = i * x;
        let vm: Vec<u32> = m
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        for e in vm {
            if !vx.contains(&e) {
                return false;
            }
        }
    }
    return true;
}

/*
Another curious number is 434782608695652173913. If x=434782608695652173913 then 3x, 4x,...22x contain the same digits:

Table[434782608695652173913 i, {i, 3, 22}]

{1304347826086956521739, 1739130434782608695652,
2173913043478260869565, 2608695652173913043478,
3043478260869565217391, 3478260869565217391304,
3913043478260869565217, 4347826086956521739130,
4782608695652173913043, 5217391304347826086956,
5652173913043478260869, 6086956521739130434782,
6521739130434782608695, 6956521739130434782608,
7391304347826086956521, 7826086956521739130434,
8260869565217391304347, 8695652173913043478260,
9130434782608695652173, 9565217391304347826086}

About the theory see Rademacher Elementary number theory.
*/
