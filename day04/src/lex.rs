use paste::paste;

macro_rules! gen_emojis {
    ($prefix:ident, $($name:ident, $value:expr),+) => {
        $(
            paste! {
                const [<$prefix $name>]: char = $value;
            }
        )+
    };
}

gen_emojis!(VAR, _1, '\u{2764}', _2, '🧡', _3, '💛', _4, '💚', _5, '💙', _6, '💜');

gen_emojis!(
    NUM, _0, '0', _1, '1', _2, '2', _3, '3', _4, '4', _5, '5', _6, '6', _7, '7', _8, '8', _9, '9'
);

gen_emojis!(OP, _ADD, '➕', _SUB, '➖', _MUL, '\u{2716}', _DIV, '➗');

const SET: char = '👈';
const PRINT: char = '🎙';
const SPEAK: char = '📢';
const LOOP: char = '🗿';
const STOP: char = '🛑';
const NEWLINE: char = '\n';

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum Token {
    Variable(u8),
    Number(u8),
    Op(Operator),
    Set,
    Print,
    Speak,
    Loop,
    Stop,
    Newline,
    Unknown(char),
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            // Variables
            VAR_1 => Self::Variable(1),
            VAR_2 => Self::Variable(2),
            VAR_3 => Self::Variable(3),
            VAR_4 => Self::Variable(4),
            VAR_5 => Self::Variable(5),
            VAR_6 => Self::Variable(6),

            // Numbers
            NUM_0 => Self::Number(0),
            NUM_1 => Self::Number(1),
            NUM_2 => Self::Number(2),
            NUM_3 => Self::Number(3),
            NUM_4 => Self::Number(4),
            NUM_5 => Self::Number(5),
            NUM_6 => Self::Number(6),
            NUM_7 => Self::Number(7),
            NUM_8 => Self::Number(8),
            NUM_9 => Self::Number(9),

            // Operators
            OP_ADD => Self::Op(Operator::Add),
            OP_SUB => Self::Op(Operator::Sub),
            OP_MUL => Self::Op(Operator::Mul),
            OP_DIV => Self::Op(Operator::Div),

            // Variable
            SET => Self::Set,
            PRINT => Self::Print,
            SPEAK => Self::Speak,

            // Other
            LOOP => Self::Loop,
            STOP => Self::Stop,
            NEWLINE => Self::Newline,
            _ => Self::Unknown(c),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    pub fn operate(&self, v1: u8, v2: u8) -> u8 {
        match self {
            Self::Add => v1.wrapping_add(v2),
            Self::Sub => v1.wrapping_sub(v2),
            Self::Mul => v1.wrapping_mul(v2),
            Self::Div => v1.wrapping_div(v2),
        }
    }
}

pub fn tokenize(script: String) -> Vec<Token> {
    let mut res = Vec::new();

    for line in script.lines() {
        // Remove whitespace
        let line = line.trim().replace(" ", "").replace("\u{fe0f}", "");

        // Remove blank lines and comments
        if line.is_empty() || line.starts_with("#") {
            continue;
        }

        for c in line.chars() {
            res.push(Token::from(c));
        }

        res.push(Token::Newline);
    }

    res.push(Token::Stop);
    res.push(Token::Newline);

    res
}
