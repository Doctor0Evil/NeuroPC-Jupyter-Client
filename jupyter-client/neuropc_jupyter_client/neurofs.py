from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Dict


@dataclass
class NeuroFSIndex:
    paths: Dict[str, Path]

    @classmethod
    def load(cls, index_path: str | Path) -> "NeuroFSIndex":
        index_path = Path(index_path)
        paths: Dict[str, Path] = {}
        for line in index_path.read_text().splitlines():
            line = line.strip()
            if not line or line.startswith("#") or "=" not in line:
                continue
            if line.startswith("["):
                continue
            key, val = [part.strip() for part in line.split("=", 1)]
            val = val.strip('"')
            paths[key] = index_path.parent / val
        return cls(paths=paths)

    def shard(self, name: str) -> Path:
        return self.paths[name]
