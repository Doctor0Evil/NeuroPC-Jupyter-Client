from __future__ import annotations

import matplotlib.pyplot as plt
import pandas as pd


def plot_roh_trend(df: pd.DataFrame, time_col: str = "t", roh_col: str = "roh_score") -> None:
    if time_col not in df or roh_col not in df:
        return
    plt.figure(figsize=(8, 3))
    plt.plot(df[time_col], df[roh_col], marker="o")
    plt.xlabel("Time")
    plt.ylabel("RoH score")
    plt.title("RoH trend over biosessions")
    plt.grid(True)
    plt.tight_layout()
