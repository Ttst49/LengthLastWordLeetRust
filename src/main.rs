fn length_of_last_word(s: String) -> i32 {
    let mut len = 0i32;
    for &c in s.as_bytes().iter().rev() {
        if c != b' ' {
            len+=1
        }else if len !=0 {
            break
        }
        
    }
    len
}


fn main() {
    let sentence = String::from("Luffy is still JoyBoy");
    length_of_last_word(sentence);
}
