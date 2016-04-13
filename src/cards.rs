pub enum CommunityChest {
    AdvanceToGo,
    BankErrorInYourFavor(i32),
    GoToJail,
    PaySchoolFees(i32),
    GetOutOfJailFree,
}

pub enum Chance {
    AdvanceToGo,
    AdvanceToNearestUtility,
    AdvanceToNearestRailroad,
    GoBack3Spaces,
    AdvanceToBoardwalk,
}
