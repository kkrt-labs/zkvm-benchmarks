import matplotlib.pyplot as plt
import pandas as pd
import numpy as np
import glob
import os
from matplotlib.patches import Patch

# Group projects by architecture type
ARCHITECTURE_GROUPS = {
    "vRAM Style Architecture": {
        "projects": ["sp1turbo", "sp1turbo-gpu", "risczero", "risczero-gpu", "zkm", "novanet"],
        "color": "lightblue"  # Background color for the group
    },
    "Modular Style Architecture": {
        "projects": ["openvm", "jolt", "jolt-gpu", "nexus"],
        "color": "lightyellow"  # Background color for the group
    }
}

# Classification of proof systems
PROOF_SYSTEMS = {
    "sp1turbo": "FRI-STARK",
    "sp1turbo-gpu": "FRI-STARK",
    "risczero": "FRI-STARK",
    "risczero-gpu": "FRI-STARK",
    "zkm": "FRI-STARK",
    "novanet": "Nebula",
    "jolt": "Lasso Lookup",
    "jolt-gpu": "Lasso Lookup",
    "openvm": "FRI-STARK",
    "nexus": "Nova"
}

# Display order is organized according to architecture groups
FIXED_ORDER = []
for group_name, group_info in ARCHITECTURE_GROUPS.items():
    FIXED_ORDER.extend(group_info["projects"])

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
    "sp1turbo": "s",
    "sp1turbo-gpu": "s",
    "zkm": "s",
    "openvm": "o",
    "nexus": "o",
    "novanet": "s",
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
        "y_label": "Proof Size (MB)",
        "column_names": {
            'n': 'proof size (bytes)',
            'size': 'proof_bytes'
        },
        "conversion_factors": {
            'n': 1000 * 1000,  # from bytes to MB
            'size': 1000 * 1000  # from bytes to MB
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

    # Create a list of existing projects, ordered by FIXED_ORDER
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
    plt.figure(figsize=(12, 8))

    # Dictionary for creating legends by group
    group_handles = {}

    # Plot a line graph for each project, grouped by architecture type
    for arch_group, group_info in ARCHITECTURE_GROUPS.items():
        # Filter only the projects belonging to this group
        group_projects = [p for p in existing_projects if p in group_info["projects"]]

        # Save the legend handles for this group
        group_handles[arch_group] = []

        for proj in group_projects:
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
                line, = plt.plot(x_data, y_data, marker=marker, linestyle='-', linewidth=2,
                                  label=f"{proj} ({PROOF_SYSTEMS.get(proj, '')})", color=color, markersize=8)

                # Add to group legend
                group_handles[arch_group].append(line)

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

    # Create legends for groups
    legend_elements = []

    # Add group headers and each project's handle
    for group_name, handles in group_handles.items():
        if handles:  # Only if projects are plotted in the graph
            # Add the group header
            legend_elements.append(Patch(facecolor='white', edgecolor='white',
                                         label=f"\n{group_name}"))
            # Add projects within the group
            legend_elements.extend(handles)

    # Show legend with grouping
    plt.legend(handles=legend_elements, bbox_to_anchor=(1.05, 1), loc='upper left',
               borderaxespad=0., frameon=True, shadow=True)

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
    fig, ax = plt.subplots(figsize=(max(14, len(existing_projects) * 2), 10))

    # Set bar width and group spacing
    bar_width = 0.15
    group_spacing = 0.3
    architecture_spacing = 0.8  # Additional space between architecture groups

    # Group projects by architecture group
    grouped_projects = {}
    for group_name, group_info in ARCHITECTURE_GROUPS.items():
        grouped_projects[group_name] = [p for p in existing_projects if p in group_info["projects"]]

    # Calculate x-axis positions
    x_positions = {}
    current_x = 0

    for group_name, projects in grouped_projects.items():
        if not projects:  # Skip if no projects in this group
            continue

        for i, program in enumerate(programs):
            if group_name not in x_positions:
                x_positions[group_name] = {}
            x_positions[group_name][program] = []

            for j, proj in enumerate(projects):
                x_pos = current_x + j * (len(programs) * bar_width + group_spacing) + i * bar_width
                x_positions[group_name][program].append(x_pos)

        # Set start position for the next group
        if projects:
            current_x += len(projects) * (len(programs) * bar_width + group_spacing) + architecture_spacing

    # Plot bars
    bars = {}
    x_tick_positions = []
    x_tick_labels = []

    # Draw background for each architecture group
    for group_name, projects in grouped_projects.items():
        if not projects:  # Skip if no projects in this group
            continue

        # Get the first and last position for the group
        min_x = min([min(x_positions[group_name][program]) for program in programs])
        max_program = programs[0]  # Use the first program arbitrarily
        max_x = max([x_positions[group_name][max_program][-1] + bar_width for max_program in programs])

        # Draw the background for the group
        rect = plt.Rectangle(
            (min_x - 0.3, 0),
            max_x - min_x + 0.6,
            1,  # Set height to 1 (reference value for logarithmic scale)
            color=ARCHITECTURE_GROUPS[group_name]["color"],
            alpha=0.3,
            zorder=-1,
            transform=ax.get_xaxis_transform()
        )
        ax.add_patch(rect)

        # Add the group title
        ax.text(
            (min_x + max_x) / 2,
            1.05,  # Adjust the y-axis position
            group_name,
            ha='center',
            va='bottom',
            fontsize=12,
            fontweight='bold',
            transform=ax.get_xaxis_transform()
        )

    # Plot bar graphs for each program and project
    for group_name, projects in grouped_projects.items():
        if not projects:
            continue

        for program in programs:
            values = []
            project_positions = []
            colors = []

            for proj in projects:
                # Get data for the combination of project and program
                if proj in all_data[program] and DEFAULT_N_VALUES[program] in all_data[program][proj]:
                    values.append(all_data[program][proj][DEFAULT_N_VALUES[program]])
                else:
                    values.append(10000)  # Use 10000 if data is not available

                project_positions.append(x_positions[group_name][program][projects.index(proj)])
                colors.append(PROJECT_COLORS.get(proj, "gray"))

            if program not in bars:
                bars[program] = {}

            # Plot bars for this program in this architecture group
            bars[program][group_name] = ax.bar(
                project_positions,
                values,
                bar_width,
                color=colors,
                label=f"{program} (n={DEFAULT_N_VALUES[program]})" if group_name == list(grouped_projects.keys())[0] else ""
            )

            # Add x-axis label positions (only add labels on the last program)
            if program == programs[-1]:
                for j, proj in enumerate(projects):
                    pos = project_positions[j]
                    x_tick_positions.append(pos)
                    x_tick_labels.append(f"{proj}\n({PROOF_SYSTEMS.get(proj, '')})")

    # Set the position and labels for the x-axis ticks
    ax.set_xticks(x_tick_positions)
    ax.set_xticklabels(x_tick_labels, rotation=45, ha='right')

    # Title and axis labels for the graph
    ax.set_title(title, fontsize=14)
    ax.set_xlabel("Project (Proof System)", fontsize=12)
    ax.set_ylabel(y_label, fontsize=12)
    ax.set_yscale("log")
    ax.grid(True, linestyle='--', alpha=0.6, axis='y')

    # Display legend
    ax.legend()

    # Display values on each bar
    for group_name, projects in grouped_projects.items():
        if not projects:
            continue

        for program in programs:
            if group_name in bars[program]:
                for bar_idx, bar in enumerate(bars[program][group_name]):
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
    plt.subplots_adjust(top=0.9)  # Make room for group titles
    plt.show()
