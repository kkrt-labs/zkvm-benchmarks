import matplotlib.pyplot as plt
import pandas as pd
import numpy as np
import glob
import os

# Common constants and settings
FIXED_ORDER = ["jolt", "jolt-gpu", "risczero", "risczero-gpu", "sp1turbo", "sp1turbo-gpu",
               "zkm", "openvm", "nexus", "novanet"]

PROJECT_COLORS = {
    "jolt": "skyblue",
    "jolt-gpu": "blue",
    "risczero": "lightgreen",
    "risczero-gpu": "green",
    "sp1turbo": "plum",
    "sp1turbo-gpu": "purple",
    "zkm": "gold",
    "openvm": "silver",
    "nexus": "orange",
    "novanet": "pink",
}

# Marker style for each project
PROJECT_MARKERS = {
    "jolt": "o",
    "jolt-gpu": "o",
    "risczero": "s",
    "risczero-gpu": "s",
    "sp1turbo": "^",
    "sp1turbo-gpu": "^",
    "zkm": "d",
    "openvm": "p",
    "nexus": "*",
    "novanet": "X",
}

# Settings for each metric
METRICS_CONFIG = {
    "prover_time": {
        "title": "Prover Time",
        "y_label": "Prover Time (s)",
        "column_names": {
            'n': 'prover time (ms)',
            'size': 'proof_duration'
        },
        "conversion_factors": {
            'n': 1000,  # from ms to s
            'size': 1000 * 1000 * 1000  # from ns to s
        }
    },
    "proof_size": {
        "title": "Proof Size",
        "y_label": "Proof Size (kb)",
        "column_names": {
            'n': 'proof size (bytes)',
            'size': 'proof_bytes'
        },
        "conversion_factors": {
            'n': 1000,  # from bytes to kb
            'size': 1000  # from bytes to kb
        }
    },
    "peak_memory": {
        "title": "Peak Memory Usage",
        "y_label": "Memory (GB)",
        "column_names": {
            'n': 'peak memory (MB)',
            'size': 'peak_memory_mb'
        },
        "conversion_factors": {
            'n': 1024,  # from MB to GB
            'size': 1024  # from MB to GB
        }
    }
}

# Default n values for each program
DEFAULT_N_VALUES = {
    "fib": 100000,
    "sha2": 2048,
    "ecdsa": 1,
    "ethtransfer": 100
}

def collect_data(program, n_values, metrics):
    """
    Collect data for the specified program and n values.

    Parameters:
        program (str): Program name or list of programs.
        n_values (dict or list): n values for each program, or list of n values for a single program.
        metrics (str): Metric name.

    Returns:
        dict: Data for each project and program.
        list: List of existing projects.
    """
    metrics_config = METRICS_CONFIG[metrics]
    column_names = metrics_config["column_names"]
    conversion_factors = metrics_config["conversion_factors"]

    # Determine whether program is a single string or a list
    programs = [program] if isinstance(program, str) else program

    # Determine if n_values is a list (for a single program) or a dict (for multiple programs)
    if isinstance(n_values, list):
        # For a single program, use the list for n values
        program_n_values = {prog: n_values for prog in programs}
    else:
        # For multiple programs, use the dictionary to get n values
        program_n_values = n_values

    # Data collection
    all_data = {}
    for prog in programs:
        all_data[prog] = {}
        file_list = glob.glob(f"20250312/{prog}_*.csv")

        for file in file_list:
            basename = os.path.splitext(os.path.basename(file))[0]
            if basename.startswith(prog + '_'):
                project = basename[len(prog)+1:]
            else:
                project = basename.split("_")[1]

            try:
                df = pd.read_csv(file)

                # Get n values (handle single value or multiple values)
                n_vals = program_n_values.get(prog, [])
                if not isinstance(n_vals, list):
                    n_vals = [n_vals]  # convert to list if single value

                if project not in all_data[prog]:
                    all_data[prog][project] = {}

                # Get data for each n value
                for n in n_vals:
                    if 'n' in df.columns:
                        row = df[df['n'] == n]
                        if not row.empty and column_names['n'] in row.columns:
                            all_data[prog][project][n] = row.iloc[0][column_names['n']] / conversion_factors['n']
                    elif 'size' in df.columns:
                        row = df[df['size'] == n]
                        if not row.empty and column_names['size'] in row.columns:
                            all_data[prog][project][n] = row.iloc[0][column_names['size']] / conversion_factors['size']
            except Exception as e:
                print(f"Error processing file {file}: {e}")
                continue

    # Create a list of existing projects
    existing_projects = []
    for proj in FIXED_ORDER:
        if any(proj in all_data[prog] for prog in programs):
            existing_projects.append(proj)

    # Add any remaining unclassified projects
    remaining_projects = set()
    for prog in programs:
        for proj in all_data[prog]:
            if proj not in FIXED_ORDER:
                remaining_projects.add(proj)

    existing_projects.extend(sorted(remaining_projects))

    return all_data, existing_projects

def plot_n_line_graph(program="ethtransfer", n_values=[1, 10, 100], metrics="prover_time"):
    """
    Plot a line graph showing changes in the selected metric for different n values for the specified program (each project is distinguished by a legend).

    Parameters:
        program (str): Program name to display (e.g., "ethtransfer", "fib", "sha2", "ecdsa").
        n_values (list): List of n values to compare (e.g., [1, 10, 100]).
        metrics (str): One of "prover_time", "proof_size", or "peak_memory".
    """
    if metrics not in METRICS_CONFIG:
        raise ValueError(f"metrics must be one of: {', '.join(METRICS_CONFIG.keys())}")

    metrics_config = METRICS_CONFIG[metrics]
    metric_title = metrics_config["title"]
    y_label = metrics_config["y_label"]

    # Collect data
    all_data, existing_projects = collect_data(program, n_values, metrics)

    if not existing_projects:
        print(f"No data found for {program} program with {metrics} metrics.")
        return

    # Graph settings
    plt.figure(figsize=(10, 6))

    # Plot a line graph for each project
    for proj in existing_projects:
        if proj not in all_data[program]:
            continue

        # Get data values for each n value
        x_data = []
        y_data = []

        for n in n_values:
            if n in all_data[program][proj]:
                x_data.append(n)
                y_data.append(all_data[program][proj][n])

        if len(x_data) > 0:  # Only plot if data is available
            color = PROJECT_COLORS.get(proj, "gray")
            marker = PROJECT_MARKERS.get(proj, "o")
            plt.plot(x_data, y_data, marker=marker, linestyle='-', linewidth=2,
                    label=proj, color=color, markersize=8)

    # Set the x-axis to logarithmic scale
    plt.xscale('log')
    plt.xticks(n_values, [str(n) for n in n_values])  # Display actual values

    # Set the y-axis to logarithmic scale
    plt.yscale('log')

    # Grid lines
    plt.grid(True, linestyle='--', alpha=0.6)

    # Title and axis labels
    plt.title(f"{program.capitalize()} {metric_title} vs Input Size (n)")
    plt.xlabel("Input Size (n)")
    plt.ylabel(y_label)

    # Show legend (with border, placed at upper right)
    plt.legend(bbox_to_anchor=(1.05, 1), loc='upper left', borderaxespad=0.,
              frameon=True, shadow=True)

    plt.tight_layout()
    plt.show()

def plot_programs_by_project(metrics="prover_time"):
    """
    Display data for each program (fib, sha2, ecdsa, ethtransfer) by project using a bar graph.

    Parameters:
        metrics (str): One of "prover_time", "proof_size", or "peak_memory".
    """
    if metrics not in METRICS_CONFIG:
        raise ValueError(f"metrics must be one of: {', '.join(METRICS_CONFIG.keys())}")

    metrics_config = METRICS_CONFIG[metrics]
    title = metrics_config["title"]
    y_label = metrics_config["y_label"]

    # Adjust the program list based on the metric
    if metrics == "peak_memory":
        programs = ["fib"]
    else:
        programs = ["fib", "sha2", "ecdsa", "ethtransfer"]

    # Collect data
    all_data, existing_projects = collect_data(programs, DEFAULT_N_VALUES, metrics)

    if not existing_projects:
        print(f"No data found for {metrics}.")
        return

    # Graph settings
    fig, ax = plt.subplots(figsize=(max(12, len(existing_projects) * 2), 8))

    # Set bar width and group spacing
    bar_width = 0.15
    group_spacing = 0.3

    # Calculate x-axis positions
    x_positions = {}
    for i, program in enumerate(programs):
        x_positions[program] = [j * (len(programs) * bar_width + group_spacing) + i * bar_width for j in range(len(existing_projects))]

    # Plot bars
    bars = {}
    for program in programs:
        values = []
        for proj in existing_projects:
            # If a combination of program and proj exists, get the data for DEFAULT_N_VALUES[program]
            if proj in all_data[program] and DEFAULT_N_VALUES[program] in all_data[program][proj]:
                values.append(all_data[program][proj][DEFAULT_N_VALUES[program]])
            else:
                values.append(10000)  # Use 10000 if data is not available

        # Plot all bars (using 10000 for missing values)
        bars[program] = ax.bar(
            x_positions[program],
            values,
            bar_width,
            color=[PROJECT_COLORS.get(proj, "gray") for proj in existing_projects],
            label=f"{program} (n={DEFAULT_N_VALUES[program]})"
        )

    # Set the position and labels for the x-axis ticks
    x_tick_positions = [j * (len(programs) * bar_width + group_spacing) + (len(programs) - 1) * bar_width / 2 for j in range(len(existing_projects))]
    ax.set_xticks(x_tick_positions)
    ax.set_xticklabels(existing_projects, rotation=45, ha='right')

    # Title and axis labels for the graph
    ax.set_title(title)
    ax.set_xlabel("Project")
    ax.set_ylabel(y_label)
    ax.set_yscale("log")
    ax.grid(True, linestyle='--', alpha=0.6, axis='y')

    # Display legend
    ax.legend()

    # Display values on each bar
    for program in programs:
        for bar_idx, bar in enumerate(bars[program]):
            height = bar.get_height()

            # Special display for missing data (10000)
            if height == 10000:
                label_text = "N/A"
                label_color = "red"
            else:
                label_text = f"{height:.2f}"
                label_color = "black"

            ax.text(
                bar.get_x() + bar.get_width() / 2,
                height,
                label_text,
                ha='center',
                va='bottom',
                fontsize=8,
                rotation=90,
                color=label_color
            )

    plt.tight_layout()
    plt.show()
