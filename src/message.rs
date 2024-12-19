use crate::common::View;

#[derive(PartialEq)]
pub(crate) enum Message {
    MoveToIndex(usize),
    GoToNextItem,
    GoToPreviousPreview,
    GoToFirstItem,
    GoToLastItem,
    SwitchWithNextItem,
    SwitchWithPreviousItem,
    SwitchWithFirstItem,
    ToggleSelection,
    SaveSelection,
    ShowView(View),
    Quit,
}
