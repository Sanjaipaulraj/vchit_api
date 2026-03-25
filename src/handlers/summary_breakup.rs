use axum::Json;

use crate::models::SummaryBreakupModel;

pub async fn summary_breakup() -> Json<SummaryBreakupModel> {
    println!("Summary Breakup called");
    let summary_breakup: SummaryBreakupModel = SummaryBreakupModel {
        total_amount: 45000.00,
        cash_amount: 1500.00,
        upi_amount: 2650.00,
        cheque_amount: 6987.56,
    };
    return Json(summary_breakup);
}
