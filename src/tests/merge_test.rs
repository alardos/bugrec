use std::vec;

use crate::
use crate::transfer::Transfer;

fn default_transfer() -> Transfer {
    Transfer { 
        id: 1, 
        auto_category: String::from("category"), 
        currency: String::from("HUF"), 
        description: String::from("desc"),
        sum: 1000.0, 
        original_balance: 5000.0,
        time: 10101010101010,
        partner_id: 1, 
        way_of_payment_id: 1,
        assigned: false,
        note: "sample note".to_string(),
    }
}

fn default_mp() -> ManualPurchase {
    ManualPurchase {
        id: 1,
        time: 103210123210,
        price: 200.0,
        longitude: 0.0,
        latitude: 0.0,
        partner_id: 1,
        item_id: 1,
    }
}



#[test]
fn manual_purchases_before_the_trasfer_are_removed_from_other_manual_purchases() {

    let transfer = Transfer {time: 10, sum: 20.0, ..default_transfer()};
    let manual_purchase = ManualPurchase {time: 1, price: 10.0, ..default_mp()};

    let mut manual_purchases = vec![manual_purchase.clone(), manual_purchase.clone()];
    
    let related = remove_related(&transfer, &mut manual_purchases);

    assert!(manual_purchases.len() == 0);
    assert!(related.len() == 2);
}


