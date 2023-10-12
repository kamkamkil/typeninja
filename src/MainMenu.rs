use crate::style::TextStyle;

 
 enum MenuState{
    StartTest,
    Results,
}

pub(crate) struct MainMenu{
    state: MenuState,
    testStyle: TextStyle
}

impl MainMenu{
    fn new(style: TextStyle) -> MainMenu{
        MainMenu { state: MenuState::StartTest, testStyle: style }
    }


}
