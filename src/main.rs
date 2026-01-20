use anyhow::Result;
use serde::Deserialize;
use std::env;
use swarms_rs::llm::provider::openai::OpenAI;
use swarms_rs::structs::sequential_workflow::SequentialWorkflow;
use swarms_rs::structs::tool::{Tool, ToolError};
use tracing_subscriber::prelude::*;

const MAX_TOKENS: u64 = 4096;

struct CalculateTool;
#[derive(Deserialize)]
struct CalculateArgs {
    expression: String,
}

impl Tool for CalculateTool {
    type Error = ToolError;

    type Args = CalculateArgs;

    type Output = String;

    const NAME: &'static str = "calculate_tool";

    fn definition(&self) -> swarms_rs::llm::request::ToolDefinition {
        swarms_rs::llm::request::ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Evaluates a mathematical expression and returns the result".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "expression": {
                        "type": "string",
                        "description": "A mathematical expression to evaluate (e.g., '2 + 2', '3.14 * 5')"
                    }
                },
                "required": ["expression"]
            }),
        }
    }

    fn call(
        &self,
        args: Self::Args,
    ) -> impl Future<Output = std::result::Result<Self::Output, Self::Error>> + Send + Sync {
        async move {
            return match meval::eval_str(args.expression) {
                Ok(v) => Ok(v.to_string()),
                Err(e) => Err(ToolError::ToolCallError(Box::new(e))),
            };
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info,swarms_rs=debug"))
        .with(
            tracing_subscriber::fmt::layer()
                .with_line_number(true)
                .with_file(true)
                .with_ansi(true),
        )
        .init();

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let client = OpenAI::new(api_key).set_model("gpt-4-turbo");

    let worker_agent = client
        .agent_builder()
        .agent_name("The Working Agent")
        .system_prompt(
            "You solve math problems. \
            Show your reasoning and use the Calculate Tool <DONE>.",
        )
        .user_name("Worker")
        .temperature(0.1)
        .max_tokens(MAX_TOKENS)
        .add_stop_word("<DONE>")
        .add_tool(CalculateTool {})
        .build();

    let verifier_agent = client
        .agent_builder()
        .agent_name("The Verifier Agent")
        .system_prompt(
            "You solve math problems. \
            Solve independently, compare to provided answer. \
            Report AGREE or DISAGREE with explanation <DONE>.",
        )
        .user_name("Verifier")
        .temperature(0.1)
        .max_tokens(MAX_TOKENS)
        .add_stop_word("<DONE>")
        .build();

    let workflow = SequentialWorkflow::builder()
        .name("Calculation Workflow")
        .add_agent(Box::new(worker_agent))
        .add_agent(Box::new(verifier_agent))
        .build();

    let task = "A stock price increases by 40% on Monday, then decreases by 40% on Tuesday. If it started at $100, what is the final price?";
    let result = workflow.run(task).await?;
    println!("{}", result);
    Ok(())
}
