use super::validate_working_days_per_week_schema::*;

pub fn perform(
    count: UnvalidatedWorkingDaysPerWeek,
) -> Result<ValidateWorkingDaysPerWeekEvent, ValidateWorkingDaysPerWeekError> {
    ValidatedWorkingDaysPerWeek::new(count)
}

// PRIVATE

impl ValidatedWorkingDaysPerWeek {
    pub fn new(count: String) -> Result<Self, ValidateWorkingDaysPerWeekError> {
        let count = count.parse::<f32>()?;
        if count > 0.0 && count < 7.0 {
            Ok(ValidatedWorkingDaysPerWeek(count))
        } else {
            Err(ValidateWorkingDaysPerWeekError::Invalid(
                "Specify between 0.0 and 7.0".to_string(),
            ))
        }
    }

    pub fn to_f32(&self) -> f32 {
        self.0
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::project_creating::validate_working_days_per_week::schema::ValidateWorkingDaysPerWeek;

//     #[test]
//     fn verify_perform_type() {
//         // 型チェックのために代入する
//         let _type_check: ValidateWorkingDaysPerWeek = super::perform;
//     }
// }