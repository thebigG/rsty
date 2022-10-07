fn main() {
    let mut count: i32 = 0;
    'counting_up: loop{
        println!("count = {count}");
        let mut remaining = 10;
        loop{
            println!("remaining = {remaining}");
            if remaining ==  9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    show_ids();
}

fn show_ids()
{
    let ids:[i32; 5] = [1,2,3,4,5];
    for id in ids
    {
        println!("id:{id}");
    }
}
