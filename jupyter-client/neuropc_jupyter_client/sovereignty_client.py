from __future__ import annotations

import json
import subprocess
from dataclasses import dataclass
from pathlib import Path
from typing import Literal, TypedDict

from .evolution_proposals import EvolutionProposalRecord


class ProposalDecision(TypedDict):
    type: Literal["allowed", "rejected", "deferred"]
    reason: str | None


@dataclass
class SovereigntyClient:
    cmd: str
    manifest: Path

    def _run_request(self, req: dict) -> dict:
        proc = subprocess.Popen(
            [self.cmd, "--manifest", str(self.manifest)],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )
        assert proc.stdin and proc.stdout
        proc.stdin.write(json.dumps(req) + "\n")
        proc.stdin.flush()
        proc.stdin.close()
        line = proc.stdout.readline()
        proc.stdout.close()
        proc.wait()
        return json.loads(line)

    def load_shards(self, index_path: Path) -> None:
        req = {"type": "load_shards", "index_path": str(index_path)}
        self._run_request(req)

    def simulate_change(self, proposal: EvolutionProposalRecord) -> dict:
        req = {"type": "simulate_change", "proposal": proposal.to_json_dict()}
        return self._run_request(req)

    def submit_proposal(self, proposal: EvolutionProposalRecord) -> dict:
        req = {"type": "submit_proposal", "proposal": proposal.to_json_dict()}
        return self._run_request(req)
