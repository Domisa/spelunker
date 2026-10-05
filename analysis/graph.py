# /// script
#
# requires-python = ">=3.12"
# dependencies = [
#    "pandas",
#    "matplotlib",
#    "seaborn",
# ]
# ///

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns


try:
    df = pd.read_json("output.json")
    print("Data loaded successfully.")


except FileNotFoundError:
    print("Error: 'output.json' file not found. Please ensure the file exists in the current directory.")
    exit(1)