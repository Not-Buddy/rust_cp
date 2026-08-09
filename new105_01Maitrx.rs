use std::cmp::min;

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n : usize = mat.len();
        let m : usize = mat[0].len();
        
        const INF : i32 = 1e9 as i32;
        let mut dist : Vec<Vec<i32>> = vec![vec![INF;m];n];
        
        for i in 0..n{
            for j in 0..m{
                if mat[i][j] == 0{
                    dist[i][j] = 0;
                }
                else{
                    if i>0{
                        dist[i][j] = min(dist[i][j],dist[i-1][j]+1);
                    }
                    if j>0{
                        dist[i][j] = min(dist[i][j],dist[i][j-1]+1);
                    }
                }
            }
        }


        for i in (0..n).rev(){
            for j in (0..m).rev(){
                if i<n-1{
                    dist[i][j] = min(dist[i][j],dist[i+1][j]+1);
                }
                if j<m-1{
                    dist[i][j] = min(dist[i][j], dist[i][j+1]+1);
                }
            }
        }
        
        dist

    }
}
