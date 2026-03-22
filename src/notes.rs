// Dynamic programming : 
pub fn fibonacci(n: usize) -> u64 {
    if n <= 1 {
        return n as u64;
    }

    let mut dp = vec![0u64; n + 1];
    dp[1] = 1;

    for i in 2..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    dp[n]
}
// Apply Dynamic programming to the Knapsack problem : 
/*
- We have n items 
- each item has : 
    -> weight w[i]
    -> value v[i]
- a knapsack with capacity W 
Ex : 
Items : 
    1 ----> W:2 ,V:3 
    2 ----> W:3 ,V:4 
    3 ----> W:4 ,V:5
    4 ----> W:5 ,V:8

Knapsack capacity : 5   

dp[2][3] =
max(
    dp[1][5],
    value[2] + dp[1][5-3]
)

0  0  1  2  3  4  5
0  0  0  0  0  0  0
1  0  0  3  3  3  3
2  0  0  3  4  4  7 
3  0  0 
4  0  0
*/
// dp[1][2] = max(dp[0][2] , 3 + 0 )  = 3 
// dp[1][3] = max(dp[0][3] , 3 + 0 ) = 3 
// dp[2][3] = max(dp[1][3] , 4 + dp[1][0] ) = 4 
pub fn knapsack_daynamic_programming(){
    let mut items : Vec<(usize,usize)> = vec![];
    items.push((2,3)); 
    items.push((3,4)); 
    items.push((4,5)); 
    items.push((5,8)); 
    let nb_items = items.len();
    let capacity : usize = 9;
    let mut dp = vec![vec![0; capacity + 1]; nb_items + 1];
    for i in 1..nb_items+1 {
        for w in 0..capacity+1{
            if items[i-1].0 > w{
                dp[i][w] = dp[i-1][w];
            }else{
                dp[i][w] =
                    usize::max(
                        dp[i-1][w],
                        items[i-1].1 + dp[i-1][w-items[i-1].0]
                    )
            }
        }
    }
    println!("Maximum possible value is : {}",dp[nb_items][capacity]);
}
////////////////////////////////////////////////////////////////////////

