# NeuroPC Jupyter Client (0xNP0C)

NeuroPC-Jupyter-Client makes Jupyter “just another client” of the sovereign data/guardrail plane for NeuroPC. All real state and policies live in ALN/JSON/NDJSON shards, and notebooks only read approved metrics and emit proposal records; they never mutate kernels or policies directly.[web:0]

Core ideas:

- Single data/guardrail plane backed by shards like `.rohmodel.aln`, `.stake.aln`, `.neurorights.json`, `.evolve.jsonl`, `.donutloop.aln`, `.ocpuenv`, `.vkernel.aln`, `.biosession.aln`.[web:0]
- Anchor Rust crates: `organiccpucore` (BioState, EcoMetrics, OrganicCpuPolicy), `organiccpualn` (ALN/JSON bindings), and `sovereigntycore` (RoH + neurorights + stake + token guards).[web:0]
- Sovereign kernel manifest (`bostrom-sovereign-kernel-v2.ndjson`) tells the client which shards and guard pipeline to use (RoH guard → neurorights guard → stake/token guard → donutloop logging).[web:0]
- Jupyter interacts only through the `neuro-assistant-api` JSON RPC process, with safe operations: `load_shards`, `simulate_change`, `submit_proposal`.[web:0]

## Layout

- `shards/` – host-bound ALN/JSON state shards.
- `rust-core/` – Rust crates for Organic CPU core, ALN bindings, and sovereignty core.
- `neuro-assistant-api/` – Rust binary exposing a JSON-over-stdin/stdout API for Jupyter.
- `jupyter-client/` – Python package and example notebooks.

## Quick start

1. Build Rust core and neuro-assistant API:

   ```bash
   cd rust-core
   cargo build --release
   cd ../neuro-assistant-api
   cargo build --release

Install the Python client:

bash
cd jupyter-client
pip install -e .
Run the neuro-assistant daemon:

bash
./target/release/neuro-assistant-api --manifest ../bostrom-sovereign-kernel-v2.ndjson
Launch Jupyter in jupyter-client/ and open notebooks/00_bootstrap_neurofs.ipynb.

NeuroPC tag: 0xNP0C.


***

### neuropc_tag.txt

```text
neuropc-tag 0xNP0C
