use rand::{self, prelude::SliceRandom, thread_rng};

fn main() {

    //let vowels:Vec<&str> = vec!["a","e","i","o","u"];

    fn genword() -> String {
        let test_str:Vec<&str> = vec!["amazing","hilarious","snowball","articulate","hunter"];
        let mut rng = thread_rng();
        let findword = test_str.choose(&mut rng).unwrap();
        findword.to_string()
    }
    
    buildword();

    fn buildword() {
        // define test vector
        // create empty word & rng
        let mut word= String::from("");
        // start gen word
        let testword = genword();
 
        match testword.chars().nth(0) {
            Some('a') => {}
            Some('e') => {}
            Some('i') => {}
            Some('o') => {}
            Some('u') => {}
            _ => {word.push_str(&testword);}

        }

        println!("{:?}",word);
    }

}
