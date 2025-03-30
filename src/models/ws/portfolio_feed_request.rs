use {
    serde::{Deserialize, Serialize},
    std::fmt,
};

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum PortfolioUpdateType {
    Order,
    GTTOrder,
    Position,
    Holding,
}

impl fmt::Display for PortfolioUpdateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PortfolioUpdateType::GTTOrder => "gtt_order",
                PortfolioUpdateType::Holding => "holding",
                PortfolioUpdateType::Order => "order",
                PortfolioUpdateType::Position => "position",
            }
        )
    }
}
