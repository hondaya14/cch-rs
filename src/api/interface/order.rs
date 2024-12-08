use serde::Deserialize;
use toml::Value;

pub fn parse_orders(value: Option<&Value>) -> Vec<Order> {
    let mut orders = vec![];
    if let Some(value) = value {
        if let Some(orders_value) = value.as_array() {
            for order_value in orders_value {
                if let Some(order) = Order::from(order_value) {
                    orders.push(order);
                }
            }
        }
    }
    orders
}

#[derive(Deserialize, Debug)]
pub struct Order {
    pub item: String,
    pub quantity: u32,
}
impl Order {
    pub fn from(value: &Value) -> Option<Self> {
        // `item` の取得と変換
        let item = value.get("item")?.as_str()?.to_string();

        // `quantity` の取得と変換
        let quantity = value.get("quantity")?.as_integer()?;
        let quantity = u32::try_from(quantity).ok()?;
        Some(Order { item, quantity })
    }

    pub fn to_string(&self) -> String {
        format!("{}: {}", self.item, self.quantity)
    }
}
