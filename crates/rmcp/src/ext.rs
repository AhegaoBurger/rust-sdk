use rmcp_types::model::Tool;
use schemars::JsonSchema;
use std::sync::Arc;

pub trait ToolExt {
    fn with_output_schema<T: JsonSchema + 'static>(self) -> Self;
    fn with_input_schema<T: JsonSchema + 'static>(self) -> Self;
}

impl ToolExt for Tool {
    fn with_output_schema<T: JsonSchema + 'static>(mut self) -> Self {
        self.output_schema = Some(crate::handler::server::tool::cached_schema_for_type::<T>());
        self
    }

    fn with_input_schema<T: JsonSchema + 'static>(mut self) -> Self {
        self.input_schema = Arc::new(crate::handler::server::tool::cached_schema_for_type::<T>().as_object().unwrap().clone());
        self
    }
}
