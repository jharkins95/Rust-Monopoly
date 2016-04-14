/// Represents a Community Chest card
pub enum CommunityChest {
    AdvanceToGo,
    BankErrorInYourFavor(i32),
    GoToJail,
    PaySchoolFees(i32),
    GetOutOfJailFree,
}

/// Represents a Chance card
pub enum Chance {
    AdvanceToGo,
    AdvanceToNearestUtility,
    AdvanceToNearestRailroad,
    GoBack3Spaces,
    AdvanceToBoardwalk,
}