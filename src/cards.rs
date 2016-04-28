//
//! Cards are used whenever a player lands on a Chance or a
//! Community Chest space. They are meant to be randomly ordered and
//! drawn from a Vector in the Board struct.
//!
//! Cards may grant players a sum of money, require them to pay a fee,
//! advance them to a specified space, or give them a Get Out of Jail Free
//! Card (planned for a future release)
//!

/// Represents a Community Chest card
pub enum CommunityChest {
    AdvanceToGo,
    BankErrorInYourFavor,
    GoToJail,
    PaySchoolFees,
}

/// Represents a Chance card
pub enum Chance {
    AdvanceToGo,
    AdvanceToNearestUtility,
    AdvanceToNearestRailroad,
    GoBack3Spaces,
    AdvanceToBoardwalk,
}