pub fn is_space(ch: char) -> bool {
    match ch {
        ' ' | '\t' | '\n' => true,
        _ => false,
    }
}

pub fn is_digital(ch: char) -> bool {
    match ch {
        '0'..='9' => true,
        _ => false,
    }
}

pub fn is_alpha(ch: char) -> bool {
    //判断是否为字母或者下划线
    match ch {
        'a'..='z' => true,
        'A'..='Z' => true,
        '_' => true,
        _ => false,
    }
}

pub fn is_alphadigital(ch: char) -> bool {
    is_alpha(ch) || is_digital(ch)
}
