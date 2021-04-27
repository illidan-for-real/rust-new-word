use rand::{self, prelude::SliceRandom, thread_rng};

fn main() {

    // choose random word
    // cut at last possible vowel
    // choose second random word
    // cut and append word at first or second vowel

    fn genwords() -> Vec<String> {

        let test_str:Vec<&str> = vec!["amazing","hilarious","snowball","articulate","hunter","paper","designated","follow","readable","remembered","volcanic","annoyance"];
        let mut test_wordlist = Vec::new();

        let mut rng = thread_rng();

        let findword = test_str.choose(&mut rng).unwrap();
        let mut findword2 = test_str.choose(&mut rng).unwrap();

        while findword == findword2 {
            println!("words are the same");
            findword2 = test_str.choose(&mut rng).unwrap();
        }

        test_wordlist.push(findword.to_string());
        test_wordlist.push(findword2.to_string());
        test_wordlist
    }

    fn build() {

        let mut new_word = String::new();

        let vowels = vec!['a','e','i','o','u'];

        let word1 = genwords().first().unwrap().clone();
        let word2 = genwords().last().unwrap().clone();

        let mut a = Vec::new();
        for (i,c) in word1.chars().enumerate() {
            if vowels.contains(&c) {
                a.push(i)
            }
        }

        let mut b = Vec::new();
        for (i,c) in word2.chars().enumerate() {
            if vowels.contains(&c) {
                b.push(i)
            }
        }

/*         println!("{:?}",a); */

        let (start,_end) = word1.split_at(a.last().unwrap().clone());

        let mut nthnum =  0;

        if vowels.contains(&word2.chars().nth(0).unwrap()) {
            nthnum = 1;
        } else {
            nthnum = 0;
        }

        let (start2,end2) = word2.split_at(b.iter().nth(nthnum).unwrap().clone());

/*         println!("{:?}",start);
        println!("{:?}",end2); */

        println!("sample word 1:{:?}",word1);
        println!("sample word 2:{:?}",word2);

        new_word.push_str(start);
        new_word.push_str(end2);

        println!("");
        println!("{:?}",new_word);
        println!("");

    }

    build();
    build();
    build();
    build();


}
