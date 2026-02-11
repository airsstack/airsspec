//! [`ValidatablePlan`] trait implementation for [`Plan`].
//!
//! This bridges the domain type (`Plan`) with the validation framework's
//! abstraction (`ValidatablePlan`), following the Dependency Inversion
//! Principle: the domain "detail" depends on the framework "abstraction".

use crate::validation::ValidatablePlan;

use super::types::Plan;

impl ValidatablePlan for Plan {
    fn spec_id_str(&self) -> &str {
        self.spec_id().as_str()
    }

    fn step_count(&self) -> usize {
        self.steps().len()
    }

    fn is_completed(&self) -> bool {
        self.is_completed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::PlanStep;
    use crate::spec::SpecId;

    #[test]
    fn test_plan_spec_id_str() {
        let plan = Plan::new(
            SpecId::new(1_737_734_400, "test-spec"),
            "Approach",
            vec![PlanStep::new(0, "Step 1", "desc")],
        );
        assert_eq!(plan.spec_id_str(), "1737734400-test-spec");
    }

    #[test]
    fn test_plan_step_count() {
        let plan = Plan::new(
            SpecId::new(1_737_734_400, "test"),
            "Approach",
            vec![
                PlanStep::new(0, "Step 1", "desc"),
                PlanStep::new(1, "Step 2", "desc"),
            ],
        );
        assert_eq!(plan.step_count(), 2);
    }

    #[test]
    fn test_plan_step_count_empty() {
        let plan = Plan::new(SpecId::new(1_737_734_400, "test"), "Approach", vec![]);
        assert_eq!(plan.step_count(), 0);
    }

    #[test]
    fn test_plan_not_completed() {
        let plan = Plan::new(
            SpecId::new(1_737_734_400, "test"),
            "Approach",
            vec![PlanStep::new(0, "Step 1", "desc")],
        );
        assert!(!plan.is_completed());
    }

    #[test]
    fn test_plan_completed() {
        let mut plan = Plan::new(
            SpecId::new(1_737_734_400, "test"),
            "Approach",
            vec![PlanStep::new(0, "Step 1", "desc")],
        );
        plan.complete_step(0, None).unwrap();
        assert!(plan.is_completed());
    }
}
