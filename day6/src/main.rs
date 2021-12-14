
#[derive(Debug, Copy, Clone)]
struct FishCycleGroup {
    cycle: i32,
    count: i64,
}

fn main() {
    // fish: 3,4,3,1,2
    // decrement timer to 0
    // at 0 the timer is reset to 6 and a new fish is created
    // new fish timers start at 8

    /*

    Intutive solution is deeply inefficient.
    We can make it better
    One option is to create a bucket for each of the eight days
    We count how many fish there are at each day
    Then in each cycle 0..=255 we adjust the accounts for that day
    At the end we math it up and we won't have had to do nearly as much work

    */

    // let mut fish_iv = vec![3,4,3,1,2];
    let mut fish_iv = vec![1,2,4,5,5,5,2,1,3,1,4,3,2,1,5,5,1,2,3,4,4,1,2,3,2,1,4,4,1,5,5,1,3,4,4,4,1,2,2,5,1,5,5,3,2,3,1,1,3,5,1,1,2,4,2,3,1,1,2,1,3,1,2,1,1,2,1,2,2,1,1,1,1,5,4,5,2,1,3,2,4,1,1,3,4,1,4,1,5,1,4,1,5,3,2,3,2,2,4,4,3,3,4,3,4,4,3,4,5,1,2,5,2,1,5,5,1,3,4,2,2,4,2,2,1,3,2,5,5,1,3,3,4,3,5,3,5,5,4,5,1,1,4,1,4,5,1,1,1,4,1,1,4,2,1,4,1,3,4,4,3,1,2,2,4,3,3,2,2,2,3,5,5,2,3,1,5,1,1,1,1,3,1,4,1,4,1,2,5,3,2,4,4,1,3,1,1,1,3,4,4,1,1,2,1,4,3,4,2,2,3,2,4,3,1,5,1,3,1,4,5,5,3,5,1,3,5,5,4,2,3,2,4,1,3,2,2,2,1,3,4,2,5,2,5,3,5,5,1,1,1,2,2,3,1,4,4,4,5,4,5,5,1,4,5,5,4,1,1,5,3,3,1,4,1,3,1,1,4,1,5,2,3,2,3,1,2,2,2,1,1,5,1,4,5,2,4,2,2,3];

    let mut fish: Vec<FishCycleGroup> = vec![
        FishCycleGroup{cycle: 0, count: 0},
        FishCycleGroup{cycle: 1, count: 0},
        FishCycleGroup{cycle: 2, count: 0},
        FishCycleGroup{cycle: 3, count: 0},
        FishCycleGroup{cycle: 4, count: 0},
        FishCycleGroup{cycle: 5, count: 0},
        FishCycleGroup{cycle: 6, count: 0},
        FishCycleGroup{cycle: 7, count: 0},
        FishCycleGroup{cycle: 8, count: 0},
    ];

    for (idx, age) in fish_iv.iter().enumerate() {
        fish[*age as usize].count += 1;
    }

    println!("{:?}", fish);

    for day in 1..=256 {
        let mut carry = fish[0];
        fish[0] = fish[1];
        fish[1] = fish[2];
        fish[2] = fish[3];
        fish[3] = fish[4];
        fish[4] = fish[5];
        fish[5] = fish[6];
        fish[6] = fish[7];
        fish[7] = fish[8];

        fish[6].count += carry.count;
        fish[8] = FishCycleGroup{cycle: 8, count: carry.count};

        let count = fish.iter().fold(0, |acc, x| acc + x.count);
        println!("Day {}, Fish {}", day, count);
    }



}
