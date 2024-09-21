**devai** - **Command Agent runner to accelerate production coding. File based, fully customizable, NOT for building snake games.**

STILL IN HEAVY DEVELOPMENT

Cherry-pick the code and make it work for yourself.

_P.S. If possible, try to refrain from publishing `devai-custom` type crates, as this might be more confusing than helpful. However, any other name is great._

Usage: `devai run _command_agent_name_ -f "**/mod.rs"`

It's very rudimentary at this stage and will change significantly between `0.0.z` releases.

**IMPORTANT** Make sure to use it on a fully committed repository so that if changes break things, you can quickly revert.

## Concept

The concept of this command line, is to have a completely configurable "Command Agent" in markdown format

This is very early experimentation, but the goal is to continue maturing the concept.

The idea is to have more "Command Agents" described as markdown configuration files that can be run with the devai command (CLI).

Right now, we only have `proof-comments.md`, and the command line logic is primitive and hardcoded. It runs for all files or specified files and saves the AI Rust result back to the file.

## Future Plan

- Support the `# Items` section with `yaml` or `Rhai`
- Support for `config.toml` in the `.devai/` with the genai table with the model
- Support for `# Config` to override some `config.toml` properties (e.g., model)
- More `Rhai` modules/functions
- Support for `# Before All`  `# Before` and `# After` and `# After All` (all `Rhai`)
- `--dry-req` will do a dry run of the request by just saving the content of the request in a file.
- `--dry-res` will do a real AI request but just capture the AI response in a file (request will be captured as well).
- `--capture` will do the normal run but will capture the req and response in the req/response file.

## Future Command Agent File

### Simplest form, just an instruction.

Right now, this will run it for the source file targeted, and whatever the AI returns will get saved back to this file. (very primitive, and just a proof of concept for now)

```md
... some instruction ...
```

