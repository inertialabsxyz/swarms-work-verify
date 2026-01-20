# Swarms Work Verify

A Worker -> Verifier multi-agent workflow using the swarms-rs library.

## Features

- Sequential workflow with worker and verifier agents
- Mathematical calculation tool using `meval`
- OpenAI

## Setup

1. Install Rust (if not already installed)
2. Copy `.env.example` to `.env` and add your OpenAI API key:
   ```
   OPENAI_API_KEY=your_key_here
   ```

## Usage

```bash
cargo run
```

## How it Works

The application creates a sequential workflow with two agents:

1. **Worker Agent**: Solves math problems using a calculation tool
2. **Verifier Agent**: Independently verifies the solution and reports agreement/disagreement

Each task flows through both agents sequentially, providing a verification layer for accuracy.

## Dependencies

- `swarms-rs` - Multi-agent framework
- `tokio` - Async runtime
- `meval` - Mathematical expression evaluator
- `anyhow` - Error handling
- `dotenv` - Environment variable management
- `tracing-subscriber` - Logging
