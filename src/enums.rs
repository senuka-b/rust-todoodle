
pub enum HomeChoices {
    CreateTodo,
    DisplayTodo,
    EditTodo,
    DeleteTodo
}

impl HomeChoices {
    pub fn get_option(input: u32) -> Option<HomeChoices> {
        match input {
            1 => Some(HomeChoices::CreateTodo),
            2 => Some(HomeChoices::DisplayTodo),
            3 => Some(HomeChoices::EditTodo),
            4 => Some(HomeChoices::DeleteTodo),

            _ => None
        }

        
    }

}