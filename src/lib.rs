pub mod filters;
pub mod inputs;
pub mod outputs;
use filters::*;
use inputs::*;
use outputs::*;

use std::convert::TryFrom;

#[derive(Debug)]
pub struct Pipeline(pub InputBlock, pub FilterBlock, pub OutputBlock);

/// A `Pipeline` is a set of `Input`s, `Filter`s, and `Output`s.
impl Pipeline {
    /// Execute a pipeline.
    pub fn run(self) {
        let filter_receiver = self.0.run();
        let output_receiver = self.1.run(filter_receiver);
        self.2.run(output_receiver);
    }

    /// Construct a `Pipeline` by parsing a TOML configuration file.
    pub fn from_toml(toml: toml::Value) -> Result<Pipeline, toml::de::Error> {
        // blocks
        let mut inputs = InputBlock(vec![]);
        let mut filters = FilterBlock(vec![]);
        let mut outputs = OutputBlock(vec![]);

        if let Some(input_block) = toml.get("inputs") {
            if let Some(input_block) = input_block.as_array() {
                input_block.iter().for_each(|input| {
                    if let Some(input_block) = input.as_table() {
                        input_block.iter().for_each(|input| {
                            inputs.0.push(input.to_input());
                        })
                    }
                });
            }
        }

        if let Some(filter_block) = toml.get("filters") {
            if let Some(filter_block) = filter_block.as_array() {
                filter_block.iter().for_each(|filter| {
                    if let Some(filter_block) = filter.as_table() {
                        filter_block.iter().for_each(|filter| {
                            filters.0.push(filter.to_filter());
                        })
                    }
                });
            }
        }

        if let Some(output_block) = toml.get("outputs") {
            if let Some(output_block) = output_block.as_array() {
                output_block.iter().for_each(|output| {
                    if let Some(output_block) = output.as_table() {
                        output_block.iter().for_each(|output| {
                            outputs.0.push(output.to_output());
                        })
                    }
                });
            }
        }

        Ok(Pipeline(inputs, filters, outputs))
    }
}

trait InputConfig {
    fn to_input(&self) -> Input;
}

impl InputConfig for (&String, &toml::Value) {
    fn to_input(&self) -> Input {
        match self.0.as_str() {
            "generator" => {
                let plugin = Generator::try_from(self.1).unwrap();
                Input::Generator(plugin, None)
            }
            "http_poller" => {
                let plugin = HttpPoller::try_from(self.1).unwrap();
                Input::HttpPoller(Box::new(plugin), None)
            }
            _ => panic!("Bad configuration for {} input block.", self.0),
        }
    }
}

trait FilterConfig {
    fn to_filter(&self) -> Filter;
}

impl FilterConfig for (&String, &toml::Value) {
    fn to_filter(&self) -> Filter {
        match self.0.as_str() {
            "mutate" => {
                let plugin = Mutate::try_from(self.1).expect("Incorrect Mutate filter config.");
                Filter::Mutate(Box::new(plugin))
            }
            "json" => {
                let plugin = Json::try_from(self.1).expect("Incorrect Json filter config.");
                Filter::Json(plugin)
            }
            _ => panic!("Bad configuration for {} filter block.", self.0),
        }
    }
}

trait OutputConfig {
    fn to_output(&self) -> Output;
}

impl OutputConfig for (&String, &toml::Value) {
    fn to_output(&self) -> Output {
        match self.0.as_str() {
            "stdout" => {
                let plugin = Stdout::try_from(self.1).expect("Incorrect Stdout output config.");
                Output::Stdout(plugin)
            }
            _ => panic!("Bad configuration for {} output block.", self.0),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::*;
    use std::path::Path;

    #[test]
    fn parse_toml() {
        let pipeline = Pipeline::from_toml(Path::new("./example_configs/full.toml"));
        assert!(pipeline.is_ok());
    }
}
