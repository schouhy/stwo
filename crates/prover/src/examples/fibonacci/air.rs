
use super::component::{FibonacciComponent, FibonacciInput, FibonacciTraceGenerator};
use crate::core::air::{Air, AirProver, Component, ComponentProver};
use crate::core::backend::simd::SimdBackend;
use crate::core::channel::Blake2sChannel;
use crate::core::fields::m31::BaseField;
use crate::core::poly::circle::CircleEvaluation;
use crate::core::poly::BitReversedOrder;
use crate::core::prover::VerificationError;
use crate::core::{ColumnVec, InteractionElements, LookupValues};
use crate::trace_generation::registry::ComponentGenerationRegistry;
use crate::trace_generation::{AirTraceGenerator, AirTraceVerifier, ComponentTraceGenerator};

pub struct FibonacciAirGenerator {
    pub registry: ComponentGenerationRegistry,
}

impl FibonacciAirGenerator {
    pub fn new(inputs: &FibonacciInput) -> Self {
        let mut component_generator = FibonacciTraceGenerator::new();
        component_generator.add_inputs(inputs);
        let mut registry = ComponentGenerationRegistry::default();
        registry.register("fibonacci", component_generator);
        Self { registry }
    }
}

impl AirTraceVerifier for FibonacciAirGenerator {
    fn interaction_elements(&self, _channel: &mut Blake2sChannel) -> InteractionElements {
        InteractionElements::default()
    }

    fn verify_lookups(&self, _lookup_values: &LookupValues) -> Result<(), VerificationError> {
        Ok(())
    }
}

impl AirTraceGenerator<SimdBackend> for FibonacciAirGenerator {
    fn write_trace(&mut self) -> Vec<CircleEvaluation<SimdBackend, BaseField, BitReversedOrder>> {
        FibonacciTraceGenerator::write_trace("fibonacci", &mut self.registry)
    }

    fn interact(
        &self,
        _trace: &ColumnVec<CircleEvaluation<SimdBackend, BaseField, BitReversedOrder>>,
        _elements: &InteractionElements,
    ) -> Vec<CircleEvaluation<SimdBackend, BaseField, BitReversedOrder>> {
        vec![]
    }

    fn to_air_prover(&self) -> impl AirProver<SimdBackend> {
        let component_generator = self
            .registry
            .get_generator::<FibonacciTraceGenerator>("fibonacci");
        FibonacciAir {
            component: component_generator.component(),
        }
    }

    fn composition_log_degree_bound(&self) -> u32 {
        let component_generator = self
            .registry
            .get_generator::<FibonacciTraceGenerator>("fibonacci");
        assert!(component_generator.inputs_set(), "Fibonacci input not set.");
        component_generator
            .component()
            .max_constraint_log_degree_bound()
    }
}

#[derive(Clone)]
pub struct FibonacciAir {
    pub component: FibonacciComponent,
}

impl FibonacciAir {
    pub fn new(component: FibonacciComponent) -> Self {
        Self { component }
    }
}

impl Air for FibonacciAir {
    fn components(&self) -> Vec<&dyn Component> {
        vec![&self.component]
    }
}

impl AirTraceVerifier for FibonacciAir {
    fn interaction_elements(&self, _channel: &mut Blake2sChannel) -> InteractionElements {
        InteractionElements::default()
    }

    fn verify_lookups(&self, _lookup_values: &LookupValues) -> Result<(), VerificationError> {
        Ok(())
    }
}

impl AirTraceGenerator<SimdBackend> for FibonacciAir {
    fn interact(
        &self,
        _trace: &ColumnVec<CircleEvaluation<SimdBackend, BaseField, BitReversedOrder>>,
        _elements: &InteractionElements,
    ) -> Vec<CircleEvaluation<SimdBackend, BaseField, BitReversedOrder>> {
        vec![]
    }

    fn to_air_prover(&self) -> impl AirProver<SimdBackend> {
        self.clone()
    }

    fn composition_log_degree_bound(&self) -> u32 {
        self.component.max_constraint_log_degree_bound()
    }
}

impl AirProver<SimdBackend> for FibonacciAir {
    fn prover_components(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        vec![&self.component]
    }
}
