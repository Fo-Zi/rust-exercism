pub enum MessageType{
    Sentence,
    SentenceAllCaps,
    Question,
    QuestionAllCaps,
    Silence
}

impl MessageType {
    fn as_str(&self) -> &'static str {
        match self {
            MessageType::Sentence           => "Whatever.",
            MessageType::SentenceAllCaps    => "Whoa, chill out!",
            MessageType::Question           => "Sure.",
            MessageType::QuestionAllCaps    => "Calm down, I know what I'm doing!",
            MessageType::Silence            => "Fine. Be that way!",
        }
    }
}

pub fn get_msg_type(message: &str) -> MessageType{
    let trimmed_msg = message.chars().filter(|char| char.is_alphanumeric() || *char == '?' || *char == '!' ).collect::<String>();
    match trimmed_msg.chars().last(){
        None => MessageType::Silence,
        Some(c) => match c {
            '?' => if is_all_caps(&trimmed_msg) { MessageType::QuestionAllCaps } else { MessageType::Question },
            _   => if is_all_caps(&trimmed_msg) { MessageType::SentenceAllCaps } else { MessageType::Sentence } 
        },
    }
}

fn is_all_caps(input: &str) -> bool {
    let alphabetic_chars: Vec<_> = input.chars().filter(|c| c.is_alphabetic()).collect();
    !alphabetic_chars.is_empty() && alphabetic_chars.iter().all(|c| c.is_uppercase())
}

pub fn reply(message: &str) -> &str {
    let msg_type = get_msg_type(message);
    &msg_type.as_str()
}
