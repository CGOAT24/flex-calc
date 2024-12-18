#[test]
fn calc_weight_with_valid_date_should_return_correct_values() {
    let request = crate::models::CalcRequest {
        total_weight: 135.0,
        bar_weight: 45.0,
        unit: crate::models::WeightUnit::Lb,
    };
    let response = super::calc_weights(request).unwrap();
    assert_eq!(response.plates[0].count, 2);
}

#[test]
fn calc_weight_with_invalid_data_should_return_error() {
    let request = crate::models::CalcRequest {
        total_weight: 135.0,
        bar_weight: 44.0,
        unit: crate::models::WeightUnit::Lb,
    };
    let response = super::calc_weights(request);
    assert_eq!(
        response.unwrap_err(),
        "No possible combination of plates".to_string()
    );
}

#[test]
fn calc_one_rm_with_valid_data_should_return_correct_value() {
    let weight = 135.0;
    let reps = 10;
    let response = super::calc_one_rm(weight, reps);
    assert_eq!(response, 180.0);
}
