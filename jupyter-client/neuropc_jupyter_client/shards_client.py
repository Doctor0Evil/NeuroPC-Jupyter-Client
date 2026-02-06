from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path

import json
import pandas as pd

from .neurofs import NeuroFSIndex


@dataclass
class ShardsClient:
    index: NeuroFSIndex

    @classmethod
    def from_index_path(cls, index_path: str | Path) -> "ShardsClient":
        return cls(index=NeuroFSIndex.load(index_path))

    def load_biosession_metrics(self) -> pd.DataFrame:
        biosession_path = self.index.shard("biosession")
        if not biosession_path.exists():
            return pd.DataFrame()
        rows = []
        for line in biosession_path.read_text().splitlines():
            line = line.strip()
            if not line:
                continue
            try:
                rows.append(json.loads(line))
            except json.JSONDecodeError:
                continue
        return pd.DataFrame(rows)

    def load_donutloop_events(self) -> list[dict]:
        donut_path = self.index.shard("donutloop_log")
        if not donut_path.exists():
            return []
        events = []
        for line in donut_path.read_text().splitlines():
            line = line.strip()
            if not line:
                continue
            try:
                events.append(json.loads(line))
            except json.JSONDecodeError:
                continue
        return events
