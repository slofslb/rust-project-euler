mod poker;

fn main() {
    let data = std::fs::read_to_string("poker.txt").expect("打开文件出错");
    let data2 = data.replace("\r\n", "\n");
    let lines = data2.trim().split('\n');
    let mut palyer1_wins = 0;
    for line in lines {
        let hand1 = poker::Hand::new(&line[..14]);
        let hand2 = poker::Hand::new(&line[15..]);
        if hand1 > hand2 {
            //println!("{}", hand1.hand_type().ranking2());
            palyer1_wins += 1;
            println!("{:3}    {}  >  {}", palyer1_wins, hand1, hand2);
        }
    }
    println!("player1 wins: {}", palyer1_wins);
}
