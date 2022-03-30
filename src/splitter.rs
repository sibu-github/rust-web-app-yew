const MAIN_CHARS_LOWER:u16 = 2437;
const MAIN_CHARS_UPPER:u16 = 2489;
const CHAR_HASANTA:u16 = 2509;
const CHAR_RHA:u16 = 2525;
const CHAR_RRA:u16 = 2524;
const CHAR_ANUSVARA:u16 = 2434;
const CHAR_KHANDATA:u16 = 2510;
const CHAR_YYA:u16 = 2527;
const CHAR_RRI:u16 = 2528;

fn is_main_chars(ch: u16) -> bool {
    ch == CHAR_ANUSVARA ||
    ch == CHAR_RHA ||
    ch == CHAR_RRA ||
    ch == CHAR_KHANDATA ||
    ch == CHAR_YYA ||
    ch == CHAR_RRI ||
    ch >= MAIN_CHARS_LOWER && ch <= MAIN_CHARS_UPPER 
}

fn is_hasanta(ch: u16) -> bool {
    ch == CHAR_HASANTA
}

fn is_breakpoint(prev: u16, next: u16) -> bool {
    is_main_chars(next) && !is_hasanta(prev)
}

pub fn get_splitted(input: &str) -> Vec<String> {
    let all_chars = input.trim().chars().map(|ch| ch as u16).collect::<Vec<_>>();
    let mut collect = vec![];
    let mut start = 0;
    loop {
        if start >= all_chars.len() {
            break;
        }
        let mut end = start + 1;
        loop {
            if end >= all_chars.len() {
                break;
            }
            let next = all_chars[end];
            let prev = all_chars[end - 1];
            if is_breakpoint(prev, next) {
                break;
            }
            end += 1;
        }
        let s = String::from_utf16_lossy(&all_chars[start..end]);
        collect.push(s);
        start = end;
    }

    collect
}

#[derive(Debug, PartialEq, Eq)]
pub enum MatchType {
    FullCorrectPos,
    FullIncorrectPos,
    PartialCorrectPos,
    PartialIncorrectPos,
    Wrong
}



pub fn get_splitted_with_matching(input: &str, match_str: &str) -> Vec<(String, MatchType)> {
    let vec1 = get_splitted(input);
    let vec2 = get_splitted(match_str);

    let mut v = vec![];
    for i in 0..vec1.len() {
        let e1 = &vec1[i];
        if let Some(e2) = vec2.get(i) {
            // Full match with correct pos
            if e1 == e2 {
                v.push((e1.to_owned(), MatchType::FullCorrectPos));
                continue;
            }
            // Full match with incorrect pos 
            if vec2.contains(e1) {
                v.push((e1.to_owned(), MatchType::FullIncorrectPos));
                continue;
            }
            // Partial match with correct pos
            if is_partial_match(e1, e2) {
                v.push((e1.to_owned(), MatchType::PartialCorrectPos));
                continue;
            }
            // partial match with incorrect pos 
            if vec2.iter().any(|e|  is_partial_match(e1, e)) {
                v.push((e1.to_owned(), MatchType::PartialIncorrectPos));
                continue;
            }
        }

        // otherwise it is wrong match
        v.push((e1.to_owned(), MatchType::Wrong));
    }


    v
}

fn is_partial_match(s1: &str, s2: &str) -> bool {
    let v1 = s1.chars().map(|ch| ch as u16).collect::<Vec<_>>();
    let v2 = s2.chars().map(|ch| ch as u16).collect::<Vec<_>>();
    v1.iter().any(|&e| {
        is_main_chars(e) && v2.contains(&e)
    })
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_splitted_with_match_1(){
        let input = "";
        let match_str = "";
        let expected = Vec::<(String, MatchType)>::new();
        assert_eq!(get_splitted_with_matching(input, match_str), expected);
    }

    #[test]
    fn test_splitted_with_match_2(){
        let input = "অবিবাহিত";
        let match_str = "অবিবাহিত";
        let expected = vec![
            ("অ".to_string(), MatchType::FullCorrectPos),
            ("বি".to_string(), MatchType::FullCorrectPos),
            ("বা".to_string(), MatchType::FullCorrectPos),
            ("হি".to_string(), MatchType::FullCorrectPos),
            ("ত".to_string(), MatchType::FullCorrectPos),
        ];
        assert_eq!(get_splitted_with_matching(input, match_str), expected);
    }

    #[test]
    fn test_splitted_with_match_3(){
        let input = "অভিযান";
        let match_str = "অভিশাপ";
        let expected = vec![
            ("অ".to_string(), MatchType::FullCorrectPos),
            ("ভি".to_string(), MatchType::FullCorrectPos),
            ("যা".to_string(), MatchType::Wrong),
            ("ন".to_string(), MatchType::Wrong),
        ];
        assert_eq!(get_splitted_with_matching(input, match_str), expected);
    }

    #[test]
    fn test_splitted_with_match_4(){
        let input = "যানজট";
        let match_str = "জলযান";
        let expected = vec![
            ("যা".to_string(), MatchType::FullIncorrectPos),
            ("ন".to_string(), MatchType::FullIncorrectPos),
            ("জ".to_string(), MatchType::FullIncorrectPos),
            ("ট".to_string(), MatchType::Wrong),
        ];
        assert_eq!(get_splitted_with_matching(input, match_str), expected);
    }

    #[test]
    fn test_splitted_with_match_5(){
        let input = "দুর্ঘটনা";
        let match_str = "দুর্বিপাক";
        let expected = vec![
            ("দু".to_string(), MatchType::FullCorrectPos),
            ("র্ঘ".to_string(), MatchType::PartialCorrectPos),
            ("ট".to_string(), MatchType::Wrong),
            ("না".to_string(), MatchType::Wrong),
        ];
        assert_eq!(get_splitted_with_matching(input, match_str), expected);
    }

    #[test]
    fn test_splitted_with_match_6(){
        let input = "কল্পতরু";
        let match_str = "কল্পকাব্য";
        let expected = vec![
            ("ক".to_string(), MatchType::FullCorrectPos),
            ("ল্প".to_string(), MatchType::FullCorrectPos),
            ("ত".to_string(), MatchType::Wrong),
            ("রু".to_string(), MatchType::Wrong),
        ];
        assert_eq!(get_splitted_with_matching(input, match_str), expected);
    }

    #[test]
    fn test_splitted_with_match_7(){
        let input = "মনোমোহিনীর";
        let match_str = "মোহিনীমোহন";
        let expected = vec![
            ("ম".to_string(), MatchType::PartialCorrectPos),
            ("নো".to_string(), MatchType::PartialIncorrectPos),
            ("মো".to_string(), MatchType::FullIncorrectPos),
            ("হি".to_string(), MatchType::FullIncorrectPos),
            ("নী".to_string(), MatchType::FullIncorrectPos),
            ("র".to_string(), MatchType::Wrong),
        ];
        assert_eq!(get_splitted_with_matching(input, match_str), expected);
    }





    // #[test]
    // fn test_blank(){
    //     let input = "";
    //     let expected = Vec::<String>::new();
    //     assert_eq!(get_splitted(input), expected);
    // }

    // #[test]
    // fn test_one_letter_a(){
    //     let input = "অ";
    //     let expected = vec!["অ".to_string()];
    //     assert_eq!(get_splitted(input), expected);
    // }
    // #[test]
    // fn test_one_letter_aa(){
    //     let input = "আ";
    //     let expected = vec!["আ".to_string()];
    //     assert_eq!(get_splitted(input), expected);
    // }
    // #[test]
    // fn test_one_letter_ksh(){
    //     let input = "ক্ষ";
    //     let expected = vec!["ক্ষ".to_string()];
    //     assert_eq!(get_splitted(input), expected);
    // }
    // #[test]
    // fn test_one_letter_ttva(){
    //     let input = "ত্ত্ব";
    //     let expected = vec!["ত্ত্ব".to_string()];
    //     assert_eq!(get_splitted(input), expected);
    // }
    // #[test]
    // fn test_one_letter_hathat(){
    //     let input = "হঠাৎ";
    //     let expected = vec!["হ".to_string(), "ঠা".to_string(), "ৎ".to_string()];
    //     assert_eq!(get_splitted(input), expected);
    // }
    // #[test]
    // fn test_one_letter_sporshokator(){
    //     let input = "স্পর্শকাতর";
    //     let expected = vec![
    //         "স্প".to_string(), 
    //         "র্শ".to_string(), 
    //         "কা".to_string(),
    //         "ত".to_string(),
    //         "র".to_string(),
    //     ];
    //     assert_eq!(get_splitted(input), expected);
    // }
    // #[test]
    // fn test_one_letter_guriguri(){
    //     let input = "গুড়িগুড়ি";
    //     let expected = vec![
    //         "গু".to_string(), 
    //         "ড়ি".to_string(), 
    //         "গু".to_string(),
    //         "ড়ি".to_string(),
    //     ];
    //     assert_eq!(get_splitted(input), expected);
    // }
    // #[test]
    // fn test_one_letter_anura(){
    //     let input = "অনূঢ়া";
    //     let expected = vec![
    //         "অ".to_string(), 
    //         "নূ".to_string(), 
    //         "ঢ়া".to_string(),
    //     ];
    //     assert_eq!(get_splitted(input), expected);
    // }
    // #[test]
    // fn test_one_letter_chandbadanidhwani(){
    //     let input = "চাঁদবদনীধ্বনি";
    //     let expected = vec![
    //         "চাঁ".to_string(), 
    //         "দ".to_string(), 
    //         "ব".to_string(),
    //         "দ".to_string(),
    //         "নী".to_string(),
    //         "ধ্ব".to_string(),
    //         "নি".to_string(),
    //     ];
    //     assert_eq!(get_splitted(input), expected);
    // }
    // #[test]
    // fn test_one_letter_dusahasik(){
    //     let input = "দুঃসাহসিক";
    //     let expected = vec![
    //         "দুঃ".to_string(), 
    //         "সা".to_string(), 
    //         "হ".to_string(),
    //         "সি".to_string(),
    //         "ক".to_string(),
    //     ];
    //     assert_eq!(get_splitted(input), expected);
    // }
    // #[test]
    // fn test_one_letter_latikalabangalata(){
    //     let input = "লতিকালবঙ্গলতা";
    //     let expected = vec![
    //         "ল".to_string(), 
    //         "তি".to_string(), 
    //         "কা".to_string(),
    //         "ল".to_string(),
    //         "ব".to_string(),
    //         "ঙ্গ".to_string(),
    //         "ল".to_string(),
    //         "তা".to_string(),
    //     ];
    //     assert_eq!(get_splitted(input), expected);
    // }
    // #[test]
    // fn test_one_letter_long_word_1(){
    //     let input = "অনন্তসর্পশ্রেনিপরিবেস্থিত​";
    //     let expected = vec![
    //         "অ".to_string(),
    //         "ন".to_string(),
    //         "ন্ত".to_string(),
    //         "স".to_string(),
    //         "র্প".to_string(),
    //         "শ্রে".to_string(),
    //         "নি".to_string(),
    //         "প".to_string(),
    //         "রি".to_string(),
    //         "বে".to_string(),
    //         "স্থি".to_string(),
    //         "ত​".to_string(),
    //     ];
    //     assert_eq!(get_splitted(input), expected);
    // }
    // #[test]
    // fn test_one_letter_long_word_2(){
    //     let input = "অঘটনঘটনপটিয়সী";
    //     let expected = vec![
    //         "অ".to_string(),
    //         "ঘ".to_string(),
    //         "ট".to_string(),
    //         "ন".to_string(),
    //         "ঘ".to_string(),
    //         "ট".to_string(),
    //         "ন".to_string(),
    //         "প".to_string(),
    //         "টি".to_string(),
    //         "য়".to_string(),
    //         "সী".to_string(),
    //     ];
    //     assert_eq!(get_splitted(input), expected);
    // }
    // #[test]
    // fn test_one_letter_long_word_3(){
    //     let input = "মূর্তমহেশ্বরমুজ্জ্বলভাস্করমিষ্টমমরনরবন্দ্যম";
    //     let expected = vec![
    //         "মূ".to_string(),
    //         "র্ত".to_string(),
    //         "ম".to_string(),
    //         "হে".to_string(),
    //         "শ্ব".to_string(),
    //         "র".to_string(),
    //         "মু".to_string(),
    //         "জ্জ্ব".to_string(),
    //         "ল".to_string(),
    //         "ভা".to_string(),
    //         "স্ক".to_string(),
    //         "র".to_string(),
    //         "মি".to_string(),
    //         "ষ্ট".to_string(),
    //         "ম".to_string(),
    //         "ম".to_string(),
    //         "র".to_string(),
    //         "ন".to_string(),
    //         "র".to_string(),
    //         "ব".to_string(),
    //         "ন্দ্য".to_string(),
    //         "ম".to_string(),
    //     ];
    //     assert_eq!(get_splitted(input), expected);
    // }
    // #[test]
    // fn test_one_letter_long_word_4(){
    //     let input = "লশুনপলাণ্ডগুঞ্জনকুম্ভীশ্রাপথন্নসুতকান্নাভোজ্যান্যমধুমাংসমূত্ররেতোহমেধ্যাভক্ষভক্ষণেগায়ত্র‍্যাষ্টসহ";
    //     let expected = vec![
    //         "ল".to_string(),
    //         "শু".to_string(),
    //         "ন".to_string(),
    //         "প".to_string(),
    //         "লা".to_string(),
    //         "ণ্ড".to_string(),
    //         "গু".to_string(),
    //         "ঞ্জ".to_string(),
    //         "ন".to_string(),
    //         "কু".to_string(),
    //         "ম্ভী".to_string(),
    //         "শ্রা".to_string(),
    //         "প".to_string(),
    //         "থ".to_string(),
    //         "ন্ন".to_string(),
    //         "সু".to_string(),
    //         "ত".to_string(),
    //         "কা".to_string(),
    //         "ন্না".to_string(),
    //         "ভো".to_string(),
    //         "জ্যা".to_string(),
    //         "ন্য".to_string(),
    //         "ম".to_string(),
    //         "ধু".to_string(),
    //         "মা".to_string(),
    //         "ং".to_string(),
    //         "স".to_string(),
    //         "মূ".to_string(),
    //         "ত্র".to_string(),
    //         "রে".to_string(),
    //         "তো".to_string(),
    //         "হ".to_string(),
    //         "মে".to_string(),
    //         "ধ্যা".to_string(),
    //         "ভ".to_string(),
    //         "ক্ষ".to_string(),
    //         "ভ".to_string(),
    //         "ক্ষ".to_string(),
    //         "ণে".to_string(),
    //         "গা".to_string(),
    //         "য়".to_string(),
    //         "ত্র‍্যা".to_string(),
    //         "ষ্ট".to_string(),
    //         "স".to_string(),
    //         "হ".to_string(),
    //     ];
    //     assert_eq!(get_splitted(input), expected);
    // }





}

