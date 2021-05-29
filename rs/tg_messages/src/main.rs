use std::collections::HashMap;
use std::option::Option::Some;

use serde_json::{Result, Value};

fn is_emoji(c: char) -> bool {
    let n = c as i32;
    (n >= 0x2030) && (n < 0x1FAE0)
}

fn count_emojis(text: &str) -> i32 {
    text.chars().fold(0, |count, c| {
        if is_emoji(c) {
            // println!("{}", c);
            count + 1
        } else {
            count
        }
    })
}

fn main() {
    println!("Starting processing.");

    let text = std::fs::read_to_string("result.json").unwrap();
    let val: Result<Value> = serde_json::from_str(&text);
    let val = val.unwrap();

    let messages = val.as_object().unwrap()["messages"].as_array().unwrap();

    #[derive(Debug)]
    struct Count<'str> {
        all_emojis_count: i32,
        emojis_in_str: i32,
        the_heaviest_emoji_string: &'str str,
    }

    let mut map = HashMap::<&str, Count>::new();

    messages.iter().for_each(|msg| {
        let msg = msg.as_object().unwrap();

        if let Some(text) = msg["text"].as_str() {
            let count = count_emojis(text);

            if count > 0 {
                if let Some(name) = msg["from"].as_str() {
                    if let Some(Count {
                                    all_emojis_count,
                                    emojis_in_str,
                                    the_heaviest_emoji_string,
                                }) = map.get_mut(name) {
                        *all_emojis_count += count;

                        if &count >= emojis_in_str {
                            *emojis_in_str = count;
                            *the_heaviest_emoji_string = text;
                        }
                    } else {
                        map.insert(name, Count {
                            all_emojis_count: count,
                            emojis_in_str: count,
                            the_heaviest_emoji_string: text,
                        });
                    }

                    // println!("{} >> {}", name, text);
                }
            }
        }
    });

    println!("Message count: {}\n", messages.len());

    println!("Max: {:?}", map.iter().fold(map.iter().next().unwrap(), |(max_name, res), (name, i)| {
        if i.emojis_in_str >= res.emojis_in_str {
            (name, i)
        } else {
            (max_name, res)
        }
    }));

    // println!("{}", count_emojis( "When the imposter is SUS! 🤣🤣🤣🤣🤣🤣😂😂🤣🤣🤣🤣🤣🤣🤣🤣🤣👌👌🤣🤣🤣🤣🤣🤣🤣🤣🤣🤣LIKE AMONG US THE FUNNY GAME👌😂😂😂😂🤣🤣🤣🤣🤣🤣🤣🤣🤣🤣🤣😂"));

    // println!("{:x}", '🤣' as i32);

    println!("Hello, world!");
}
