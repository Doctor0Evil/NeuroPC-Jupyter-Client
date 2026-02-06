from __future__ import annotations

from dataclasses import dataclass, asdict
from typing import Dict
import uuid


@dataclass
class EvolutionProposalRecord:
    proposal_id: str
    author_module: str
    description: str
    suggested_roh_delta: float

    @classmethod
    def draft(
        cls,
        author_module: str,
        description: str,
        suggested_roh_delta: float,
    ) -> "EvolutionProposalRecord":
        return cls(
            proposal_id=str(uuid.uuid4()),
            author_module=author_module,
            description=description,
            suggested_roh_delta=suggested_roh_delta,
        )

    def to_json_dict(self) -> Dict:
        return asdict(self)
