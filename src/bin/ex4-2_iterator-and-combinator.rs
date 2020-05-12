/*
 2. 다음 문제를 반복자와 컴비네이터를 사용하여 작성하세요.
 let data = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
 data의 i 뻔째 값과 i + 1 번째 값을 곱한 값이
 만약 3의 배수이거나 5의 배수이면 제외한 벡터를 만들어 출력하세요.
 또 만들어진 벡터에서 2의 배수의 갯수를 세어서 출력하세요.
 */

fn main() {
    let data = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
    let itor_1 = data.iter().zip(&data[1..])
        .map(|(a, b)| a * b)
        .filter(|v| v % 3 != 0 && v % 5 != 0);

    let itor_2 = itor_1.clone();

    println!("{:?}", itor_1.collect::<Vec<i32>>());
    println!("{:?}", itor_2.filter(|v| v % 2 == 0).count());
}