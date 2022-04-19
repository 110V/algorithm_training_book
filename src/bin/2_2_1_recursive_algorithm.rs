

fn main() {
    let mut subset:Vec<i32> = vec![];
    search(3,1, &mut subset);   
}

fn search(n:i32,k: i32,subset:&mut Vec<i32>) {
    if k == n+1 {
        //부분집합을 처리한다.
        println!("{:?}",subset);
    }
    else { 
        // k를 부분집합에 포함시킨다.
        subset.push(k);
        search(n,k+1,subset);
        subset.pop();
        search(n,k+1, subset);
    }
}


