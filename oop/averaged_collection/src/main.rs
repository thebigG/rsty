use averaged_collection::AveragedCollection;

fn main() {
    let mut avg = AveragedCollection::new();

    avg.add(12);
    avg.add(20);
    avg.add(22);
    avg.add(55);
    avg.add(30);

    println!("Average:{}", avg.average());
}
