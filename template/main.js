async function getPartners() {
    return (await fetch("http://localhost:8000/api/partners")).json();
}

async function getRecords() {
    return (await fetch("http://localhost:8000/api/records")).json();
}
async function getItems() {
    return (await fetch("http://localhost:8000/api/item/get-all")).json();
}

async function getRecordsByPartner(id) {
    return (await fetch("http://localhost:8000/api/records/by_partner?id="+id)).json();
}

async function getBalance() {
    return (await fetch("http://localhost:8000/api/balance")).json();
}

async function getRecord(id) {
    return (await fetch("http://localhost:8000/api/records/get?id="+id)).json();
}

async function getPartner(id) {
    return (await fetch("http://localhost:8000/api/partners/get?id="+id)).json();
}

async function getItem(id) {
    return (await fetch("http://localhost:8000/api/item/get?id="+id)).json();
}

async function postItem(item) {
    console.log("postItem",item);
    return (await fetch("http://localhost:8000/api/item/create",{method: "POST", body: JSON.stringify(item)}))
}
async function postPurchase(purchase) {
    return (await fetch("http://localhost:8000/api/purchase/create",{method: "POST", body: JSON.stringify(purchase)}))
}

async function postTag(tag) {
    return (await fetch("http://localhost:8000/api/tag/create",{method: "POST", body: JSON.stringify(tag)}))
}

async function getPurchases() {
    return (await fetch("http://localhost:8000/api/purchase/get-all")).json()
}

async function getTags() {
    return (await fetch("http://localhost:8000/api/tag/prepared-list")).json()
}

async function postAssignTagToPartner(tagId, partnerId) {
    return (await fetch("http://localhost:8000/api/partners/assign-tag?tag-id="+tagId+"&partner-id="+partnerId, {method:"POST"}))
}

async function postAssignTagToItem(tagId, itemId) {
    return (await fetch("http://localhost:8000/api/item/assign-tag?tag-id="+tagId+"&item-id="+itemId, {method:"POST"}))
}

async function getNavHtml() {
    return ((await fetch("http://localhost:8000/nav")).text())
}

async function mergePartners(body) {
    return (await fetch("http://localhost:8000/api/partners/merge",{method: "POST", body: JSON.stringify(body)}))
}

async function getBudgetRecords() {
    return (await fetch("/api/budget-record/get-all",)).json();
}

async function postBudgetRecord(budgetRecord) {
    console.log(JSON.stringify(budgetRecord));
    return (await fetch("http://localhost:8000/api/budget-record/add",{method: "POST", body: JSON.stringify(budgetRecord)}))
}
async function updatePlan(plan) {
    return (await fetch(`http://localhost:8000/api/planning/update?category_id=${plan.category_id}&year=${plan.year}&month_id=${plan.month_id}&amount=${plan.amount}`,{method: "POST", body: JSON.stringify(plan)}))
}
