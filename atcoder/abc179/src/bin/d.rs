use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! { n: usize, k: usize }
    let mut lr = Vec::with_capacity(k);
    for _ in 0..k {
        input! { l: usize, r: usize }
        lr.push((l, r));
    }
    let mut dp = vec![0usize; n];
    let mut sdp = vec![0usize; n];
    dp[0] = 1;
    sdp[0] = 1;
    for i in 1..n {
        for (l, r) in lr.iter() {
            if i >= *l {
                dp[i] += if i < *r + 1 {
                    sdp[i-*l] - 0
                } else {
                    if sdp[i-*l] < sdp[i-*r-1] {
                        MOD + sdp[i-*l] - sdp[i-*r-1]
                    } else {
                        sdp[i-*l] - sdp[i-*r-1]
                    }
                };
                dp[i] %= MOD;
            }
        }
        sdp[i] = (sdp[i-1] + dp[i]) % MOD;
    }
    println!("{}", dp[n-1]);
}
