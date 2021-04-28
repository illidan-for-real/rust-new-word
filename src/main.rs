use rand::{self, Rng, prelude::SliceRandom, thread_rng};

fn main() {

    // choose random word
    // cut at last possible vowel
    // choose second random word
    // cut and append word at first or second vowel

    fn genwords() -> Vec<String> {

        let test_str:Vec<&str> = vec!["sauce","amazing","hilarious","snowball","articulate","hunter","paper","designated","follow","readable","remembered","volcanic","annoyance","illidan","june","money","celebrate","arbitrary","brazilian","hospitality","gravity","resteraunt","cafeteria","piggy","agility","smoke","orange","blue","brilliant","painting","grandfather"];
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
/*         println!("{:?}",&a);
 */
        let mut addamt = 0;

        if a.len() == 1 {
            addamt = 1;
        } else {
            addamt = 0;
        }

        let mut rng = thread_rng().gen_range(1..a.len()+addamt);

        let (start,_end) = word1.split_at(a.get(rng -1).unwrap().clone());

        let mut nthnum =  0;

        if vowels.contains(&word2.chars().nth(0).unwrap()) {
            nthnum = 1;
        } else {
            nthnum = 0;
        }

        let (start2,end2) = word2.split_at(b.iter().nth(nthnum).unwrap().clone());

/*         println!("{:?}",start);
        println!("{:?}",end2); */

        let mut rands = thread_rng();

        let adjectives = ["happy","annoyed","quick","scarce","sad","strong","slow","boring","dull","jealous","obnoxious", "droll", "ridiculous", "silly", "whimsical", "adorable", "wicked", "grumpy", "spectacular", "old-fashioned", "nutty", "expensive", "clever", "adventurous", "evil", "bloody", "bad", "naughty", "thieving" ];
        let verbs = ["hit","run","cheer","laugh","cry","explode"];
        let desc = ["conveying", "insulting", "shouting","alerting"];
        let animals = ["bats","cats","rats","koalas","apes","elephants","dolphins","beetles","whales","wolves","toads","birds","ants","squid","starfish","jellyfish","panthers","stingray","spiders","rabbits","pigs","cows","sheep","donkeys", "zombies", "unicorns", "vampires", "starfish", "kittens", "cockroaches", "cockapoo", "hamsters", "insects", "huskydoodles", "ostriches" ];
        let upping = ["enlighten","grow","break","recover","catch","eat","manipulate","wave","destroy", "smell", "understand", "enhance", "disguise", "identify", "mock", "clone", "control"];
        let bodypart = ["legs","heads","foot","legs","toes","tails","faces","eyes","hands", "minds", "farts", "feathers", "eyeballs", "pie holes", "smelly socks", "dark secrets", "winning personalities", "desire for fame and fortune"];
        let color = ["red","blue","green","orange","light blue","dark green","pink","light purple","teal","brown", "luminous", "shimmering", "white as snow", "invisible", "rainbow hued", "glowing", "crimson", "golden", "albino"];
        let group = ["group","colony","species","pack","swarm","gang", "army", "militia", "bunch", "pile", "team", "crowd", "jury", "corporation", "class"];
        let place = ["norway","england","africa","peru","india","canada","australia","outer space","thailand","poland","germany", "the shadow of the moon", "mars", "the mountains of uranus", "the desert", "suburban back yards", "disnyeland", "the white house", "fluffy clouds", "all over the place", "places of worship", "the bottom of your cup", "impossible places"  ];

        let findadj = adjectives.choose(&mut rands).unwrap();
        let findverb = verbs.choose(&mut rands).unwrap();
        let finddesc = desc.choose(&mut rands).unwrap();
        let findanimal = animals.choose(&mut rands).unwrap();
        let findup = upping.choose(&mut rands).unwrap();
        let findbpart = bodypart.choose(&mut rands).unwrap();
        let findcolor = color.choose(&mut rands).unwrap();
        let findgroup = group.choose(&mut rands).unwrap();
        let findplace = place.choose(&mut rands).unwrap();



        let mut new_sentence = String::new();

        let mut sentence_template = String::new();

        let mut randef = thread_rng().gen_range(0..2);

        match randef {
            0 => {
                let s = format!("A {} technique used by {} to {} their {}", findadj, findanimal, findup, findbpart);
                sentence_template.push_str(&s);
            }
            1 => {
                let s = format!("A {} of {} {} usually found in {}", findgroup, findcolor, findanimal, findplace);
                sentence_template.push_str(&s);
            }
            2 => {
                let s = format!("A {} technique used by {} to {} their {}", findadj, findanimal, findup, findbpart);
                sentence_template.push_str(&s);
            }

            _ => {
                let s = format!("A {} technique used by {} to {} their {}", findadj, findanimal, findup, findbpart);
                sentence_template.push_str(&s);
            }
        }
        



        // print sample words
        println!("sample word 1:{:?}",word1);
        println!("sample word 2:{:?}",word2);

        // push start and end parts of words
        new_word.push_str(start);
        new_word.push_str(end2);

        // print new word
        println!("");
        println!("{:?}",new_word);

        // print definition
        println!("{:?}",sentence_template);
        println!("");


    }

    build();
    build();
    build();
    build();


}
