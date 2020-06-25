use crate::word::ForthWord;
#[derive(Debug)]
pub struct Dictionary<'a> {
    words: Vec<ForthWord<'a>>,
}

impl<'a> Dictionary<'a> {
    pub fn new(dict: Vec<ForthWord<'a>>) -> Self {
        Dictionary {
            words: dict,
        }
    }
    pub fn get_by_pos(&self, pos: usize) -> &ForthWord<'a> {
        &self.words[pos]
    }

    pub fn add(&mut self, w:ForthWord<'a>) {
        self.words.push(w);
    }

    pub fn find(&self, name: &str) -> Option<usize> {
        let word = self
            .words
            .iter()
            .rev()
            .position(|x| x.name.eq_ignore_ascii_case(name));

        match word {
            Some(w) => Some(self.words.len() - w - 1),
            None => None,
        }
    }

    pub fn last_word(&mut self) -> &mut ForthWord<'a> {
        let len = self.words.len();
        &mut self.words[len - 1]
    }

    pub fn list_words(& self) {
        for (i, w) in self.words.iter().enumerate() {
            println!("{:>4}: {:?}", i, w);
        }
    }
}