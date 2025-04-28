use rand::Rng;

pub struct Game {
    pub secret: u32,
    pub lower_bound: u32,
    pub higher_bound: u32,
    pub history: Vec<(u32, String)>, 
    pub hint_step: u32, 
}

impl Default for Game {
    fn default() -> Self {
        let secret = rand::rng().random_range(1..=100); 
        Self {
            secret,
            lower_bound: 1,
            higher_bound: 100,
            history: Vec::new(), 
            hint_step: 30, 
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Game::default()
    }

    pub fn check_guess(&mut self, guess: u32) -> String {
       
        let result = if guess == self.secret {
            "Ты угадал! умничка<3".to_string()
        } else if guess < self.secret {
            "Не. Нужно побольше((".to_string()
        } else {
            "Перебор чёт.".to_string()
        };

        self.history.push((guess, result.clone())); 

        result
    }

    pub fn give_hint(&mut self) {
        let step = self.hint_step;

     
        self.hint_step = match self.hint_step {
            30 => 20,
            20 => 10,
            10 => 5,
            _ => 5, 
        };

      
        self.lower_bound = self.secret.saturating_sub(step).max(1);
        self.higher_bound = (self.secret + step).min(100);
    }
}

