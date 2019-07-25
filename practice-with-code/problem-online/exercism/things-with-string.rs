pub fn reply(message: &str) -> &str {
    let is_question = message.trim().ends_with('?');
    let is_silence = message.trim().is_empty();
    let has_letters = message.chars().any(|c| c.is_alphabetic());
    let all_upper = message.chars().all(|c| c.is_uppercase() || !c.is_alphabetic());
    let is_yell = all_upper && has_letters;
//    let is_yell = message.chars().all(|c| c.is_uppercase()) || message.trim().ends_with('!');

    if is_question && is_yell { "Calm down, I know what I'm doing!" }
    else if is_question { "Sure." }
    else if is_yell { "Whoa, chill out!" }
    else if is_silence { "Fine. Be that way!" }
    else { "Whatever." }
}
