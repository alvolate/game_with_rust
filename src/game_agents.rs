pub mod environment{
    use crate::game_agents::agent::Snake;
    pub struct Environment{
        pub width: u32,
        pub snake: Snake,
    }
    impl Environment{
        pub fn new(width: u32) -> Self{
            Self{
                width:width,
                snake: Snake::new(14),
            }
        }
        pub fn get_snakehead(&self) -> u32{
            self.snake.body[0]
        }
}}

pub mod agent{
    pub struct Snake{
        pub body: Vec<u32>,
    }
    impl Snake{
        pub fn new(index:u32) -> Self{
            Self{
                body: vec![index],
            }
        }
    }

}