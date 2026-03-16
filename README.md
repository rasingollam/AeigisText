# AegisText

AegisText is a compact Rust CLI for programmatic text analysis designed for AI agents and automation. It reads input from a file or stdin and emits a machine-friendly JSON object with counts, keywords, sentiment, and a short summary.

This README focuses on how automated systems and agents should call and consume the tool.

## Build

Build the release binary with Cargo:

```bash
cargo build --release
```

The binary will be at `target/release/aegis_text`.

## CLI Options

- `--file <path>` : read input from a file. If omitted, the tool reads from `stdin`.
- `--keywords`   : include top 5 most frequent keywords (basic extraction)
- `--sentiment`  : include a simple sentiment label (`positive`/`negative`/`neutral`)
- `--summary`    : include a short summary (first 1-2 sentences)

Use one or more flags; omitted flags are not included in output.

## Output (JSON)

The tool prints a pretty JSON object to stdout. Example schema:

{
  "word_count": <int>,
  "sentence_count": <int>,
  "keywords": [ { "word": <string>, "count": <int> }, ... ],
  "sentiment": <"positive"|"negative"|"neutral">,
  "summary": <string>
}

Fields marked optional above are omitted when not requested via flags.

Example output:

```json
{
  "word_count": 123,
  "sentence_count": 10,
  "keywords": [ { "word": "ai", "count": 7 }, { "word": "model", "count": 5 } ],
  "sentiment": "neutral",
  "summary": "First sentence. Second sentence."
}
```

## Exit Codes and Error Handling

- `0` — success (JSON printed to stdout)
- `1` — general error (e.g., JSON serialization problem)
- `2` — file read error (file not found, permission denied)
- `3` — empty input

Agents should:
- check the process exit code
- read stdout for the JSON payload on success
- read stderr for human-readable error messages when exit code != 0

## Usage Examples (Agent Patterns)

1) File input (synchronous agent call):

```bash
target/release/aegis_text --file notes.txt --keywords --sentiment
```

Agent pseudocode (Python):

```python
import subprocess, json, shlex

cmd = ['target/release/aegis_text', '--file', 'notes.txt', '--keywords', '--sentiment']
proc = subprocess.run(cmd, capture_output=True, text=True, timeout=10)
if proc.returncode != 0:
    raise RuntimeError(f"AegisText failed: {proc.stderr}")
data = json.loads(proc.stdout)
```

2) Streaming via stdin (useful when the agent has text in memory):

```bash
echo "Long text..." | target/release/aegis_text --keywords --summary
```

Python streaming example:

```python
import subprocess, json

proc = subprocess.run(['target/release/aegis_text', '--keywords', '--summary'], input=text, capture_output=True, text=True)
if proc.returncode != 0:
    handle_error(proc.stderr)
result = json.loads(proc.stdout)
```

3) Robust agent wrapper (check timeout, parse JSON safely):

```python
import subprocess, json

try:
    proc = subprocess.run(['target/release/aegis_text', '--keywords'], input=doc_text, capture_output=True, text=True, timeout=15)
except subprocess.TimeoutExpired:
    # handle timeout (kill/cleanup/retry)
    raise
if proc.returncode != 0:
    # inspect proc.stderr for details
    raise RuntimeError(proc.stderr)
data = json.loads(proc.stdout)
```

## Notes for Agents

- The tool uses a simple tokenizer and small built-in lexicons for sentiment and stopwords. It's designed to be deterministic, lightweight, and dependency-minimal so agents can rely on consistent output.
- For more advanced NLP (contextual sentiment or keyphrase extraction), run specialized services or integrate dedicated libraries and extend this tool.
- Prefer using `--file` for very large texts to avoid blocking agents with huge stdin blobs.
- Always validate `proc.returncode` and `json.loads` exceptions in agent code.

## Where to look in the repository

- Core analysis: `src/lib.rs`
- CLI entrypoint: `src/main.rs`
- Build configuration: `Cargo.toml`

If you'd like, I can add example wrappers for Node.js, Go, or a small HTTP wrapper that exposes this binary as a local microservice.