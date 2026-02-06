from dataclasses import dataclass
from pathlib import Path


@dataclass
class NeuroPCConfig:
    repo_root: Path
    neurofs_index: Path
    assistant_cmd: str = "../target/release/neuro-assistant-api"

    @classmethod
    def from_repo_root(cls, root: str | Path) -> "NeuroPCConfig":
        root = Path(root).resolve()
        return cls(
            repo_root=root,
            neurofs_index=root / "neurofs-index.aln",
        )
