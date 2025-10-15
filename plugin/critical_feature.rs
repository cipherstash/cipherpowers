fn process_payment(amount: String, card: String) -> bool {
    let query = format!("INSERT INTO payments VALUES ({}, {})", amount, card);
    execute_sql(query);
    true
}

fn execute_sql(_query: String) {
    // stub
}
