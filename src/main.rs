mod operations;

fn main() { 
    let mut first_num: u64 = 0;
    let mut second_num: u64 = 1;
    let mut third_num: u64;
    let mut values_array: Vec<u64> = vec![0, 1];
    let mut count: usize = 0;  
    let desired_value: usize = 32;  
    loop {
        third_num = operations::sum(first_num, second_num); 
        values_array.push(third_num);
        first_num = second_num;
        second_num = third_num;
        count += 1;
        if count == desired_value{
            break;
        }
      } 
    println!("{:?}", values_array);  
}
