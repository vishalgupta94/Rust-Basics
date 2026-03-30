// #[derive(Debug)]
// struct Deck{
//     cards: Vec<String>
// }

// impl Deck{
//     fn new() -> Self {
//     let cards = ["spades", "diamonds", "hearts","clubs"];
//     let ranks = ["one", "two", "king", "queen"];

//     let mut deck = vec![];


//     for rank in ranks{
//         for card in cards {
//             deck.push(format!("{} of {}",rank, card));
//         }
//     }

//     Deck {
//         cards: deck
//     }
//     }
// }

// fn main(){
//     let deck = Deck::new();
//     println!("Hello world {:#?} ", deck);
// }


impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        
        let mut  lower = 0;
        let mut higher = nums.len() - 1;

        while lower <= higher {
            let mut mid = (lower + higher) / 2;

            if nums[mid] == target{
                return mid as i32;
            }else if nums[mid] > target{
                higher = mid - 1;
            }else{
                lower = mid + 1;
            }

        }

        return -1;
    }
}