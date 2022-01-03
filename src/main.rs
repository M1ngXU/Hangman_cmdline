use std::collections::HashSet;
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};
use colored::*;

#[test]
fn test_process_guess() {
    let mut h: HashSet<char> = HashSet::new();
    let word = String::from("abc");
    assert_eq!(process_guess(&"a".to_string(), &mut h, &word), GuessResult::CorrectGuess('a'));
    assert!(h.contains(&'a'));
    assert_eq!(process_guess(&"a".to_string(), &mut h, &word), GuessResult::AlreadyGuessed('a'));
    assert_eq!(process_guess(&"d".to_string(), &mut HashSet::new(), &word), GuessResult::WrongGuess('d'));
    assert_eq!(process_guess(&"bb".to_string(), &mut HashSet::new(), &word), GuessResult::InvalidGuess);
}

#[test]
fn test_get_output_by_word() {
    let mut h: HashSet<char> = HashSet::new();
    let word = String::from("abc");
    assert_eq!(get_output_by_word(&word, &h), "_ _ _".to_string());
    process_guess(&"a".to_string(), &mut h, &word);
    assert_eq!(get_output_by_word(&word, &h), "a _ _".to_string());
    process_guess(&"c".to_string(), &mut h, &word);
    assert_eq!(get_output_by_word(&word, &h), "a _ c".to_string());
}

#[derive(Debug, PartialEq)]
enum GuessResult {
    AlreadyGuessed(char),
    CorrectGuess(char),
    WrongGuess(char),
    InvalidGuess
}

fn process_guess(guess: &String, guessed: &mut HashSet<char>, word: &String) -> GuessResult {
    if guess.len() != 1 {
        GuessResult::InvalidGuess
    } else {
        let mut c = guess.chars().next().unwrap();
        if !c.is_alphabetic() {
            GuessResult::InvalidGuess
        } else {
            c.make_ascii_lowercase();
            // insert returns false if value was already in the hashset
            if !guessed.insert(c) {
                GuessResult::AlreadyGuessed(c)
            } else if word.to_lowercase().contains(c) {
                GuessResult::CorrectGuess(c)
            } else {
                GuessResult::WrongGuess(c)
            }
        }
    }
}

static UNKNOWN_CHAR: char = '?';
fn get_output_by_word(word: &String, guessed: &HashSet<char>) -> String {
    word.chars().map(| c | if !c.is_alphabetic() || guessed.contains(&c.to_ascii_lowercase()) { c.to_string() } else { UNKNOWN_CHAR.to_string() }).collect::<Vec<String>>().join("")
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

static HANGMANS: [ &str; 12 ] = [
	"          \n          \n          \n           \n           \n           \n  ",
	"          \n          \n          \n           \n           \n           \n/ \\",
	"          \n |        \n |        \n |         \n |         \n |         \n/ \\",
	"  ________\n |        \n |        \n |         \n |         \n |         \n/ \\",
	"  ________\n |/       \n |        \n |         \n |         \n |         \n/ \\",
	"  ________\n |/      |\n |        \n |         \n |         \n |         \n/ \\",
	"  ________\n |/      |\n |       𝙾\n |         \n |         \n |         \n/ \\",
	"  ________\n |/      |\n |       𝙾\n |       | \n |       | \n |         \n/ \\",
	"  ________\n |/      |\n |       𝙾\n |       | \n |       | \n |      /  \n/ \\",
	"  ________\n |/      |\n |       𝙾\n |       | \n |       | \n |      / \\\n/ \\",
	"  ________\n |/      |\n |       𝙾\n |      /| \n |       | \n |      / \\\n/ \\",
	"  ________\n |/      |\n |       𝙾\n |      /|\\\n |       | \n |      / \\\n/ \\",
];

// from https://www.hangmanwords.com/words
static WORDS: [ &str; 213 ] = [ "abruptly", "absurd", "abyss", "affix", "askew", "avenue", "awkward", "axiom", "azure", "bagpipes", "bandwagon", "banjo", "bayou", "beekeeper", "bikini", "blitz", "blizzard", "boggle", "bookworm", "boxcar", "boxful", "buckaroo", "buffalo", "buffoon", "buxom", "buzzard", "buzzing", "buzzwords", "caliph", "cobweb", "cockiness", "croquet", "crypt", "curacao", "cycle", "daiquiri", "dirndl", "disavow", "dizzying", "duplex", "dwarves", "embezzle", "equip", "espionage", "euouae", "exodus", "faking", "fishhook", "fixable", "fjord", "flapjack", "flopping", "fluffiness", "flyby", "foxglove", "frazzled", "frizzled", "fuchsia", "funny", "gabby", "galaxy", "galvanize", "gazebo", "giaour", "gizmo", "glowworm", "glyph", "gnarly", "gnostic", "gossip", "grogginess", "haiku", "haphazard", "hyphen", "iatrogenic", "icebox", "injury", "ivory", "ivy", "jackpot", "jaundice", "jawbreaker", "jaywalk", "jazziest", "jazzy", "jelly", "jigsaw", "jinx", "jiujitsu", "jockey", "jogging", "joking", "jovial", "joyful", "juicy", "jukebox", "jumbo", "kayak", "kazoo", "keyhole", "khaki", "kilobyte", "kiosk", "kitsch", "kiwifruit", "klutz", "knapsack", "larynx", "lengths", "lucky", "luxury", "lymph", "marquis", "matrix", "megahertz", "microwave", "mnemonic", "mystify", "naphtha", "nightclub", "nowadays", "numbskull", "nymph", "onyx", "ovary", "oxidize", "oxygen", "pajama", "peekaboo", "phlegm", "pixel", "pizazz", "pneumonia", "polka", "pshaw", "psyche", "puppy", "puzzling", "quartz", "queue", "quips", "quixotic", "quiz", "quizzes", "quorum", "razzmatazz", "rhubarb", "rhythm", "rickshaw", "schnapps", "scratch", "shiv", "snazzy", "sphinx", "spritz", "squawk", "staff", "strength", "strengths", "stretch", "stronghold", "stymied", "subway", "swivel", "syndrome", "thriftless", "thumbscrew", "topaz", "transcript", "transgress", "transplant", "triphthong", "twelfth", "twelfths", "unknown", "unworthy", "unzip", "uptown", "vaporize", "vixen", "vodka", "voodoo", "vortex", "voyeurism", "walkway", "waltz", "wave", "wavy", "waxy", "wellspring", "wheezy", "whiskey", "whizzing", "whomever", "wimpy", "witchcraft", "wizard", "woozy", "wristwatch", "wyvern", "xylophone", "yachtsman", "yippee", "yoked", "youthful", "yummy", "zephyr", "zigzag", "zigzagging", "zilch", "zipper", "zodiac", "zombie" ];
fn main() {
    let word = WORDS[(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() % WORDS.len() as u128) as usize].to_string();
    let different_characters = HashSet::<char>::from_iter(word.chars().filter(| c | c.is_alphabetic())).len();
    let mut guessed = HashSet::new();
    let mut guesses = 0;
    let mut mistakes = 0;

    clear_screen();
		println!("Welcome to Hangman!\n");
    while mistakes < 11 && different_characters > guesses - mistakes {
        println!(
            "Amount of wrong/total guesses: {}/{}\n{}\nGuessed: {}\nWord: {}",
            mistakes,
            guesses,
						HANGMANS[mistakes],
            guessed.iter().map(| c: &char | c.to_string()).collect::<Vec<String>>().join(" "),
            get_output_by_word(&word, &guessed)
        );

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line!");

        clear_screen();

        guess = guess.trim().to_string();

        println!("{}\n", match process_guess(&guess, &mut guessed, &word) {
            GuessResult::WrongGuess(c) => {
                guesses += 1;
                mistakes += 1;
                format!("{}! The word doesn't include `{}`.", "WRONG".red().bold(), (&*c.to_string()).red().bold())
            },
            GuessResult::CorrectGuess(c) => {
                guesses += 1;
                format!("{}! The word does include `{}`.", "CORRECT".green().bold(), (&*c.to_string()).green().bold())
            },
            GuessResult::InvalidGuess => format!("{}. Only enter ONE ALPHABETIC CHARACTER. (Entered: `{}`)", "Invalid guess".red().bold(), (&*guess).red().bold()),
            GuessResult::AlreadyGuessed(c) => format!("You {} guessed {}.", "already".yellow().bold(), (&*c.to_string()).yellow().bold())
        });
    }
		println!("{}", HANGMANS[mistakes]);
		if different_characters == guesses - mistakes {
    	println!("{}!!! The word was `{}`! Your guesses were {} times wrong.", "CORRRRRRRECT".green().bold(), word.trim().green().bold(), mistakes.to_string().trim().red().bold());
		} else {
			println!("{}!!! The word was `{}`! Your guesses were {} times wrong.", "WROOOOOOOONG".red().bold(), word.trim().green().bold(), mistakes.to_string().trim().red().bold());
		}
}
