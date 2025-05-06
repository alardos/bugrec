use std::{todo, collections::HashMap, ops::Div, vec, cmp::Reverse, panic, dbg, };

use sqlx::{Pool, Postgres};

use crate::repository::{partner_repo::{self, Partner}, manual_purchase_repo::{ManualPurchase, self}, purchase_repo::{Purchase, self}, common_repo::Location, transfer_repo::{Transfer, self}};

/**
 * Merge: 
 * - From phone:
 *      - location
 *      - time of decision
 *      - item
 * - From OTP:
 *      - amount spent
 *      - time of the payment
 *      - partner
 */

async fn gen_purchase(pool: &Pool<Postgres>) {
    let partners = partner_repo::get_all(pool).await;
    let mut transfers = transfer_repo::get_all(pool).await; // They are still unassigned
    let mut manual_purchases = manual_purchase_repo::get_all(pool).await; // They are still  
    
    let transfer_manual_purchase: HashMap<Transfer, ManualPurchase> = HashMap::new();

    let new_purchases = create_purchases(partners, transfers, manual_purchases);

    purchase_repo::save_all(new_purchases);
    manual_purchase_repo::clear();
}

pub fn create_purchases(
    partners: Vec<Partner>, 
    mut transfers: Vec<Transfer>, 
    mut manual_purchases: Vec<ManualPurchase>
) -> Vec<Purchase> {

    let mut purchases = vec![];

    // todo transactions that do not requre manual purchases
    transfers.sort_by_key(|t|t.time);
    
    for transfer in transfers {
        let taking = remove_related(&transfer, &mut manual_purchases);

        for mp in &taking {
            purchases.push(Purchase {
                id: None,
                item_id: mp.item_id,
                time: transfer.time,
                partner_id: transfer.partner_id,
                sum: mp.price,
                transfer_id: transfer.id,
            });
        }
    }

    assert!(&manual_purchases.len() == &0);

    purchases

}

const HOUR_IN_MILLIS: u64 = 3_600_000;
const METER_IN_GPS_COORD: f64 = 0.00001;

pub fn remove_related(transfer: &Transfer, manual_purchases: &mut Vec<ManualPurchase>) -> Vec<ManualPurchase> {

    // let mut prior = manual_purchases.iter().filter(|mp| mp.time < transfer.time).collect::<Vec<&ManualPurchase>>(); 
    let mut prior: Vec<ManualPurchase> = vec![];
    manual_purchases.sort_by_key(|mp| mp.time);
    if manual_purchases.len() > 0 {
        let mut x = 1;
        while x <= manual_purchases.len() {
            if manual_purchases.get(manual_purchases.len() - x).unwrap().time < transfer.time {
                let mp = manual_purchases.remove(manual_purchases.len() - x);
                prior.push(mp);
            } else {
                x = x+1;
            }
        }
    }
            


    let mut taking: Vec<ManualPurchase> = vec![];
        
    let mut sum: f64 = 0.0;
    while sum <= transfer.sum {
        if prior.len() == 0 { break; }
        let next = prior.pop().unwrap();
        assert!(next.partner_id == transfer.partner_id, "Manual purchase has different partner from directly following transfer"); 
        
        
        taking.push(next)
    }

    let mut binding = taking.iter().map(|mp|mp.price).reduce(|acc,n| acc+n);
    let found_sum = binding.get_or_insert(0.0);
    assert!(&transfer.sum == found_sum);


    manual_purchases.append(&mut prior);

    taking
}

pub fn connect_transfers_to_manual_purchases(transfers: Vec<Transfer>, manual_purchases: Vec<ManualPurchase>, partners: Vec<Partner>) -> HashMap<Transfer, ManualPurchase> {
//    let result_map: HashMap<Transfer, ManualPurchase> = HashMap::new();
//    let mut distinct_location_set: Vec<Vec<ManualPurchase>> = vec![];
//
//    for transfer in transfers {
//
//        let partner = partners.iter().find(|p| p.id == transfer.partner_id).unwrap();
//
//        if partner.special {
//            result_map.put(transfer, vec![]);
//            continue;
//        }
//
//        for manual_purchase in manual_purchases {
//            let nearest_set = nearest_set(&manual_purchase, &distinct_location_set);
//
//            let nearest_locations: Vec<(f64, f64)> = nearest_set.iter().map(|x|x.location).collect();
//        let acc_distance = metric_distance(
//            &Location::avg_loc(&acc_locations),
//            &origo.location
//            );
//
//        return if acc_distance > n_distance { 
//            acc 
//        } else { 
//            n 
//        }
//
//    };
//
//    distinct_location_set.iter().reduce(recuce).unwrap()
//    t
    todo!()
}

// fn is_near(set: Vec<Location>, other: Location) -> Option<bool> {
//     let avg_loc = &Location::avg_loc_vec(&set);
//     let distance = other.metric_distance(avg_loc)
// 
//     Some(distance < METER_IN_GPS_COORD * 500.0)
// }
// 
// 
// 
// 
// 
// impl ManualPurchase {
//     fn is_possibly_related(&self, t: &Transfer, p: &Partner) -> bool {
//         // by partner
//         // by sum
//         //
// 
//         // with manually set partner
// 
//         
//         self.time.abs_diff(t.time) < HOUR_IN_MILLIS // by itme
//         || self.partner_id == t.partner_id
//         || metric_distance(self.location, p.location) < METER_IN_GPS_COORD * 500.0
//     }
// 
// }
// 
// 
// fn metric_distance(a: &(f64,f64), b: &(f64, f64)) -> f64 {
// }
