import matplotlib.pyplot as plt
import pandas as pd
import numpy as np
import glob
import os
from matplotlib.patches import Patch
import matplotlib.colors as mcolors

# Group projects by architecture type
ARCHITECTURE_GROUPS = {
    "vRAM Style Architecture": {
        "projects": ["sp1turbo", "sp1turbo-gpu", "risczero", "risczero-gpu", "openvm", "zkm", "jolt", "jolt-gpu", "nexus", "novanet"],
        "color": "lightblue"  # Background color for the group
    },
    # "Modular Style Architecture": {
    #     "projects": list(reversed(["openvm", "jolt", "jolt-gpu", "nexus"])),
    #     "color": "lightyellow"  # Background color for the group
    # }
}

# Classification of proof systems
PROOF_SYSTEMS = {
    "sp1turbo": "FRI-STARK",
    "sp1turbo-gpu": "FRI-STARK",
    "risczero": "FRI-STARK",
    "risczero-gpu": "FRI-STARK",
    "zkm": "FRI-STARK",
    "novanet": "Nebula-Nova",
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
    "jolt": "green",
    "jolt-gpu": "darkgreen",
    "risczero": "gold",
    "risczero-gpu": "orange",
    "sp1turbo": "pink",
    "sp1turbo-gpu": "magenta",
    "zkm": "navy",
    "openvm": "silver",
    "nexus": "blue",
    "novanet": "purple",
}

# Color intensity by program - define base colors and their intensity variations
PROGRAM_COLOR_INTENSITY = {
    "fib": 1.0,       # Full intensity
    "sha2": 0.8,      # 80% intensity
    "ecdsa": 0.6,     # 60% intensity
    "ethtransfer": 0.4  # 40% intensity
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
    Plot a line graph showing changes in the selected metric for different n values for the specified program.
    Each project is labeled directly next to its corresponding line.
    Values >= 10000 are excluded from plotting as they represent missing or invalid data.

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
    all_data, all_existing_projects = collect_data(program, n_values, metrics)

    if not all_existing_projects:
        print(f"No data found for {program} program with {metrics} metrics.")
        return

    # Sort projects according to FIXED_ORDER if available, otherwise alphabetically
    existing_projects = []
    for proj in FIXED_ORDER:
        if proj in all_existing_projects:
            existing_projects.append(proj)

    # Add any remaining projects that aren't in FIXED_ORDER
    for proj in sorted(all_existing_projects):
        if proj not in existing_projects:
            existing_projects.append(proj)

    if not existing_projects:
        print(f"No valid projects found for {program} program with {metrics} metrics.")
        return

    # Graph settings
    plt.figure(figsize=(12, 8))

    # フォントサイズを20に統一
    plt.rcParams.update({'font.size': 20})

    # Plot a line graph for each project
    for proj in existing_projects:
        if proj not in all_data[program]:
            continue

        # Get data values for each n value, excluding values >= 10000
        x_data = []
        y_data = []

        for n in n_values:
            if n in all_data[program][proj]:
                value = all_data[program][proj][n]
                # Only include values less than 10000 (exclude missing/invalid data)
                if value < 10000:
                    x_data.append(n)
                    y_data.append(value)

        if len(x_data) > 0:  # Only plot if valid data is available
            color = PROJECT_COLORS.get(proj, "gray")
            marker = PROJECT_MARKERS.get(proj, "o")
            line, = plt.plot(x_data, y_data, marker=marker, linestyle='-', linewidth=2,
                             color=color, markersize=8)

            # Add label directly at the end of the line
            # Use the last point for label position
            if len(x_data) > 0:
                # Label at the last point of the line
                label_x = x_data[-1]
                label_y = y_data[-1]

                # Add the project name and proof system as a text label
                plt.annotate(
                    f"{proj} ({PROOF_SYSTEMS.get(proj, '')})",
                    xy=(label_x, label_y),
                    xytext=(10, 0),  # Slight offset to the right
                    textcoords='offset points',
                    fontsize=16,
                    color=color,
                    fontweight='bold',
                    verticalalignment='center',
                    bbox=dict(boxstyle="round,pad=0.3", fc="white", ec=color, alpha=0.8)
                )

    # Set the x-axis to logarithmic scale
    plt.xscale('log')
    plt.xticks(n_values, [str(n) for n in n_values], fontsize=20)  # Display actual values

    # Set the y-axis to logarithmic scale
    plt.yscale('log')
    plt.tick_params(axis='y', labelsize=20)

    # Grid lines
    plt.grid(True, linestyle='--', alpha=0.6)

    # Title and axis labels
    plt.title(f"{program.capitalize()} {metric_title} vs Input Size (n)", fontsize=20)
    plt.xlabel("Input Size (n)", fontsize=20)
    plt.ylabel(y_label, fontsize=20)

    # Adjust margins to make space for text labels
    plt.tight_layout()
    plt.subplots_adjust(right=0.85)  # Add some space on the right for labels

    plt.show()

def adjust_color_intensity(base_color, intensity):
    """
    Adjust the intensity of a color.

    Parameters:
        base_color (str): The base color (like 'skyblue' or '#RRGGBB')
        intensity (float): Intensity factor between 0 and 1

    Returns:
        str: Adjusted color in hex format
    """
    # Convert named color to RGB
    rgb = mcolors.to_rgb(base_color)

    # Darken the color by adjusting towards black
    # We preserve the hue but reduce brightness by mixing with black
    darker_rgb = tuple(c * intensity for c in rgb)

    # Convert back to hex
    return mcolors.to_hex(darker_rgb)

def plot_programs_by_project_simple(metrics="prover_time", selected_projects=None, selected_programs=None, hidden_projects=None):
    """
    Display data for each program by project using a horizontal bar graph
    without architecture grouping.

    Parameters:
        metrics (str): One of "prover_time", "proof_size", or "peak_memory".
        selected_projects (list): List of projects to display. If None, all projects are displayed.
        selected_programs (list): List of programs to display. If None, default programs are used.
        hidden_projects (list): List of projects to display in light gray. These projects will still be shown,
                               but with reduced visual prominence.
    """
    if metrics not in METRICS_CONFIG:
        raise ValueError(f"metrics must be one of: {', '.join(METRICS_CONFIG.keys())}")

    metrics_config = METRICS_CONFIG[metrics]
    title = metrics_config["title"]
    y_label = metrics_config["y_label"]

    # Initialize hidden_projects if it's None
    if hidden_projects is None:
        hidden_projects = []

    # Adjust the program list based on the metric and selected_programs
    if selected_programs is not None:
        # Filter selected programs that are valid
        valid_programs = ["fib", "sha2", "ecdsa", "ethtransfer"]
        programs = [prog for prog in selected_programs if prog in valid_programs]
    else:
        # Use default programs based on metrics
        if metrics == "peak_memory":
            programs = ["fib"]
        else:
            programs = ["fib", "sha2", "ecdsa", "ethtransfer"]

    if not programs:
        print("No valid programs selected.")
        return

    title = title + " (" + ", ".join(programs) + ")"

    # Collect data
    all_data, all_existing_projects = collect_data(programs, DEFAULT_N_VALUES, metrics)

    # Filter existing projects based on selected_projects if provided
    if selected_projects is not None:
        existing_projects = [p for p in all_existing_projects if p in selected_projects]
    else:
        existing_projects = all_existing_projects

    if not existing_projects:
        print(f"No data found for {metrics} with the selected projects and programs.")
        return

    # Sort projects according to FIXED_ORDER if available, otherwise alphabetically
    sorted_projects = []
    for proj in FIXED_ORDER:
        if proj in existing_projects:
            sorted_projects.append(proj)

    # Add any remaining projects that aren't in FIXED_ORDER
    for proj in sorted(existing_projects):
        if proj not in sorted_projects:
            sorted_projects.append(proj)

    existing_projects = sorted_projects

    # 重要な変更: プロジェクトの順序を逆転させ、上から下への表示にする
    existing_projects = existing_projects[::-1]

    # Graph settings - increase figure size for better readability
    fig, ax = plt.subplots(figsize=(16, max(10, len(existing_projects) * 0.8)))

    # フォントサイズを20に統一
    plt.rcParams.update({'font.size': 20})

    # Set bar height and spacing
    bar_height = 0.15
    group_spacing = 0.2  # 狭めたスペース

    # Calculate y-axis positions
    y_positions = {}

    # 重要な変更: プログラムの順序を逆転させ、上から下へ表示するように変更
    reversed_program_idx = {prog: len(programs) - 1 - i for i, prog in enumerate(programs)}

    for program in programs:
        y_positions[program] = []
        prog_idx = reversed_program_idx[program]  # 逆順のインデックスを使用

        for j, proj in enumerate(existing_projects):
            y_pos = j * (len(programs) * bar_height + group_spacing) + prog_idx * bar_height
            y_positions[program].append(y_pos)

    # Plot bars
    bars = {}
    y_tick_positions = []
    y_tick_labels = []
    y_tick_colors = []  # Store colors for each tick label

    # Plot horizontal bar graphs for each program and project
    # プログラムの順序はそのまま保持（凡例のために元の順序を維持）
    for program in programs:
        values = []
        project_positions = []
        colors = []
        is_hidden = []  # Flag to track if the project is in hidden_projects

        for proj in existing_projects:
            # Get data for the combination of project and program
            if proj in all_data[program] and DEFAULT_N_VALUES[program] in all_data[program][proj]:
                values.append(all_data[program][proj][DEFAULT_N_VALUES[program]])
            else:
                values.append(10000)  # Use 10000 if data is not available

            project_positions.append(y_positions[program][existing_projects.index(proj)])

            # Check if this project is in hidden_projects
            is_hidden_project = proj in hidden_projects
            is_hidden.append(is_hidden_project)

            # Apply color based on whether the project is hidden or not
            if is_hidden_project:
                # Use light gray for hidden projects
                colors.append("lightgray")
            else:
                # Apply intensity to base color according to program for normal projects
                base_color = PROJECT_COLORS.get(proj, "gray")
                intensity = PROGRAM_COLOR_INTENSITY.get(program, 1.0)
                adjusted_color = adjust_color_intensity(base_color, intensity)
                colors.append(adjusted_color)

        if program not in bars:
            bars[program] = {}

        # Plot horizontal bars for this program
        bars[program] = ax.barh(
            project_positions,
            values,
            bar_height,
            color=colors,
            label=f"{program} (n={DEFAULT_N_VALUES[program]})"
        )

        # Add y-axis label positions (only add labels for the first program now since we're showing from top to bottom)
        # プログラムのインデックスが最大（上部に表示）の場合にラベルを追加
        if reversed_program_idx[program] == len(programs) - 1:
            for j, proj in enumerate(existing_projects):
                pos = project_positions[j]
                y_tick_positions.append(pos)
                y_tick_labels.append(f"{proj}\n({PROOF_SYSTEMS.get(proj, '')})")

                # Store the color for this project's label (gray if hidden)
                if proj in hidden_projects:
                    y_tick_colors.append("gray")  # Use gray for hidden projects
                else:
                    y_tick_colors.append(PROJECT_COLORS.get(proj, "black"))

    # Set the position and labels for the y-axis ticks with font size 20
    ax.set_yticks(y_tick_positions)
    y_labels = ax.set_yticklabels(y_tick_labels, fontsize=20)  # フォントサイズ20に統一

    # Set the color of each y-axis label to match its corresponding project color
    for i, label in enumerate(y_labels):
        label.set_color(y_tick_colors[i])
        # Make the text bold for better visibility unless it's a hidden project
        if y_tick_colors[i] != "gray":
            label.set_fontweight('bold')

    # Title and axis labels for the graph with font size 20
    ax.set_title(title, fontsize=20)
    ax.set_ylabel("Project", fontsize=20)
    ax.set_xlabel(y_label, fontsize=20)
    ax.set_xscale("log")

    # Make grid lines more prominent
    ax.grid(True, linestyle='--', alpha=0.7, axis='x', linewidth=1.2)

    # フォントサイズ20に統一
    ax.tick_params(axis='x', labelsize=18)

    # Create a custom legend for programs with different intensities
    program_legend_elements = []
    for program in programs:
        intensity = PROGRAM_COLOR_INTENSITY.get(program, 1.0)
        # Use a common color to show the intensity variations
        legend_color = adjust_color_intensity("#3366CC", intensity)
        program_legend_elements.append(
            Patch(facecolor=legend_color,
                  label=f"{program} (n={DEFAULT_N_VALUES[program]})")
        )

    # Add the main legend for programs with font size 20
    legend = ax.legend(handles=program_legend_elements, loc='lower right', fontsize=20)
    legend.get_frame().set_linewidth(1.5)  # Make legend border more visible

    # Display values on each bar - make them more visible
    for program in programs:
        for bar_idx, bar in enumerate(bars[program]):
            width = bar.get_width()
            project = existing_projects[bar_idx]
            is_hidden_project = project in hidden_projects

            # Special display for missing data (10000)
            if width == 10000:
                label_text = "N/A"
                label_color = "red" if not is_hidden_project else "darkgray"
            else:
                label_text = f"{width:.2f}"
                label_color = "black" if not is_hidden_project else "darkgray"

            # Add background to text for better readability
            ax.text(
                width,
                bar.get_y() + bar.get_height() / 2,
                label_text,
                ha='left',
                va='center',
                fontsize=20,  # フォントサイズ20に統一
                fontweight='bold' if not is_hidden_project else 'normal',  # Make text bold unless hidden
                color=label_color,
                bbox=dict(facecolor='white', alpha=0.7, pad=1, edgecolor='none')  # Add background to text
            )

    # Add more padding
    plt.tight_layout()
    plt.subplots_adjust(left=0.20, right=0.95)  # More space for y-axis labels

    # Add a border around the plot
    for spine in ax.spines.values():
        spine.set_linewidth(1.5)

    plt.show()

def plot_programs_by_project(metrics="prover_time", selected_projects=None, selected_programs=None):
    """
    Display data for each program by project using a horizontal bar graph.
    Each program's bars have different color intensity.

    Parameters:
        metrics (str): One of "prover_time", "proof_size", or "peak_memory".
        selected_projects (list): List of projects to display. If None, all projects are displayed.
        selected_programs (list): List of programs to display. If None, default programs are used.
    """
    if metrics not in METRICS_CONFIG:
        raise ValueError(f"metrics must be one of: {', '.join(METRICS_CONFIG.keys())}")

    # Import path effects explicitly
    import matplotlib.patheffects as path_effects

    metrics_config = METRICS_CONFIG[metrics]
    title = metrics_config["title"]
    y_label = metrics_config["y_label"]

    # Adjust the program list based on the metric and selected_programs
    if selected_programs is not None:
        # Filter selected programs that are valid
        valid_programs = ["fib", "sha2", "ecdsa", "ethtransfer"]
        programs = [prog for prog in selected_programs if prog in valid_programs]
    else:
        # Use default programs based on metrics
        if metrics == "peak_memory":
            programs = ["fib"]
        else:
            programs = ["fib", "sha2", "ecdsa", "ethtransfer"]

    if not programs:
        print("No valid programs selected.")
        return

    # Collect data
    all_data, existing_projects = collect_data(programs, DEFAULT_N_VALUES, metrics)

    # Filter existing projects based on selected_projects if provided
    if selected_projects is not None:
        existing_projects = [p for p in existing_projects if p in selected_projects]

    if not existing_projects:
        print(f"No data found for {metrics} with the selected projects and programs.")
        return

    # Graph settings - increase figure size for better readability
    fig, ax = plt.subplots(figsize=(16, max(12, len(existing_projects) * 1.5)))

    # Increase font sizes globally
    plt.rcParams.update({'font.size': 14})

    # Set bar height and group spacing
    bar_height = 0.15
    group_spacing = 0.3
    architecture_spacing = 0.8  # Additional space between architecture groups

    # Group projects by architecture group
    grouped_projects = {}
    for group_name, group_info in ARCHITECTURE_GROUPS.items():
        group_projs = [p for p in existing_projects if p in group_info["projects"]]
        if group_projs:  # Only include groups with projects to display
            grouped_projects[group_name] = group_projs

    # Calculate y-axis positions (reversed from original x-axis positions for horizontal bars)
    y_positions = {}
    current_y = 0

    for group_name, projects in grouped_projects.items():
        if not projects:  # Skip if no projects in this group
            continue

        for i, program in enumerate(programs):
            if group_name not in y_positions:
                y_positions[group_name] = {}
            y_positions[group_name][program] = []

            for j, proj in enumerate(projects):
                y_pos = current_y + j * (len(programs) * bar_height + group_spacing) + i * bar_height
                y_positions[group_name][program].append(y_pos)

        # Set start position for the next group
        if projects:
            current_y += len(projects) * (len(programs) * bar_height + group_spacing) + architecture_spacing

    # Plot bars
    bars = {}
    y_tick_positions = []
    y_tick_labels = []
    y_tick_colors = []  # Store colors for each tick label

    # Draw background for each architecture group
    for group_name, projects in grouped_projects.items():
        if not projects:  # Skip if no projects in this group
            continue

        # Get the first and last position for the group
        min_y = min([min(y_positions[group_name][program]) for program in programs])
        max_program = programs[0]  # Use the first program arbitrarily
        max_y = max([y_positions[group_name][max_program][-1] + bar_height for max_program in programs])

        # Draw the background for the group
        rect = plt.Rectangle(
            (0, min_y - 0.3),
            1,  # Set width to 1 (reference value for logarithmic scale)
            max_y - min_y + 0.6,
            color=ARCHITECTURE_GROUPS[group_name]["color"],
            alpha=0.3,
            zorder=-1,
            transform=ax.get_yaxis_transform()
        )
        ax.add_patch(rect)

        # Add the group title with larger text
        ax.text(
            0,  # X-axis position
            (min_y + max_y) / 2,  # Y-axis position
            group_name,
            ha='right',
            va='center',
            fontsize=20,  # Increased font size for architecture group
            fontweight='bold',
            transform=ax.get_yaxis_transform(),
            rotation=90
        )

    # Plot horizontal bar graphs for each program and project
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

                project_positions.append(y_positions[group_name][program][projects.index(proj)])

                # Apply intensity to base color according to program
                base_color = PROJECT_COLORS.get(proj, "gray")
                intensity = PROGRAM_COLOR_INTENSITY.get(program, 1.0)
                adjusted_color = adjust_color_intensity(base_color, intensity)
                colors.append(adjusted_color)

            if program not in bars:
                bars[program] = {}

            # Plot horizontal bars for this program in this architecture group
            bars[program][group_name] = ax.barh(
                project_positions,
                values,
                bar_height,
                color=colors,
                label=f"{program} (n={DEFAULT_N_VALUES[program]})" if group_name == list(grouped_projects.keys())[0] else ""
            )

            # Add y-axis label positions (only add labels on the last program)
            if program == programs[-1]:
                for j, proj in enumerate(projects):
                    pos = project_positions[j]
                    y_tick_positions.append(pos)
                    y_tick_labels.append(f"{proj}\n({PROOF_SYSTEMS.get(proj, '')})")
                    # Store the base color for this project's label
                    y_tick_colors.append(PROJECT_COLORS.get(proj, "black"))

    # Set the position and labels for the y-axis ticks with increased font size
    ax.set_yticks(y_tick_positions)
    y_labels = ax.set_yticklabels(y_tick_labels, fontsize=20)  # Increased font size for y-axis labels

    # Set the color of each y-axis label to match its corresponding project color
    for i, label in enumerate(y_labels):
        label.set_color(y_tick_colors[i])
        # Add a subtle black outline instead of using path_effects
        label.set_fontweight('bold')  # Make the text bold for better visibility

    # Title and axis labels for the graph with increased font sizes
    ax.set_title(title, fontsize=20)  # Increased font size for title
    ax.set_ylabel("Project", fontsize=20)  # Increased font size for y-axis label
    ax.set_xlabel(y_label, fontsize=20)  # Increased font size for x-axis label
    ax.set_xscale("log")

    # Make grid lines more prominent
    ax.grid(True, linestyle='--', alpha=0.7, axis='x', linewidth=1.2)

    # Increase x-axis tick font size
    ax.tick_params(axis='x', labelsize=12)

    # Create a custom legend for programs with different intensities
    program_legend_elements = []
    for program in programs:
        intensity = PROGRAM_COLOR_INTENSITY.get(program, 1.0)
        # Use a common color to show the intensity variations
        legend_color = adjust_color_intensity("#3366CC", intensity)
        program_legend_elements.append(
            Patch(facecolor=legend_color,
                  label=f"{program} (n={DEFAULT_N_VALUES[program]})")
        )

    # Add the main legend for programs with increased font size
    legend = ax.legend(handles=program_legend_elements, loc='lower right', fontsize=20)  # Increased font size
    legend.get_frame().set_linewidth(1.5)  # Make legend border more visible

    # Display values on each bar - make them more visible
    for group_name, projects in grouped_projects.items():
        if not projects:
            continue

        for program in programs:
            if group_name in bars[program]:
                for bar_idx, bar in enumerate(bars[program][group_name]):
                    width = bar.get_width()

                    # Special display for missing data (10000)
                    if width == 10000:
                        label_text = "N/A"
                        label_color = "red"
                    else:
                        label_text = f"{width:.2f}"
                        label_color = "black"

                    # Add background to text for better readability
                    ax.text(
                        width,
                        bar.get_y() + bar.get_height() / 2,
                        label_text,
                        ha='left',
                        va='center',
                        fontsize=20,  # Increased font size for bar values
                        fontweight='bold',  # Make text bold
                        color=label_color,
                        bbox=dict(facecolor='white', alpha=0.7, pad=1, edgecolor='none')  # Add background to text
                    )

    # Add more padding for group titles
    plt.tight_layout()
    plt.subplots_adjust(left=0.15, right=0.9)  # More space for titles and y-axis labels

    # Add a border around the plot
    for spine in ax.spines.values():
        spine.set_linewidth(1.5)

    plt.show()

def plot_fib_memory_time_scatter():
    """
    Create a scatter plot with peak memory (GB) on x-axis and prover time (s) on y-axis
    for the fib program at n=100000, using the collect_data function.
    """
    target_n = DEFAULT_N_VALUES["fib"]  # n=100000

    # Collect data for both metrics
    memory_data, memory_projects = collect_data("fib", [target_n], "peak_memory")
    time_data, time_projects = collect_data("fib", [target_n], "prover_time")

    # Combine data into a single structure for scatter plot
    combined_data = {}
    projects = set(memory_projects + time_projects)

    for project in projects:
        if (project in memory_data.get("fib", {}) and
            project in time_data.get("fib", {}) and
            target_n in memory_data["fib"].get(project, {}) and
            target_n in time_data["fib"].get(project, {})):

            combined_data[project] = {
                'memory': memory_data["fib"][project][target_n],
                'time': time_data["fib"][project][target_n]
            }

    if not combined_data:
        print(f"No complete data found for fib program with n={target_n}.")
        return

    # Graph settings
    fig, ax = plt.subplots(figsize=(12, 8))

    # Store handles for the legend
    handles_by_group = {}

    # Group projects by architecture type for different markers/colors
    for arch_group, group_info in ARCHITECTURE_GROUPS.items():
        # Filter projects in this group that have data
        group_projects = [p for p in group_info["projects"] if p in combined_data]

        # Initialize handles for this group
        handles_by_group[arch_group] = []

        for project in group_projects:
            # Get the color and marker for this project
            color = PROJECT_COLORS.get(project, "gray")
            marker = PROJECT_MARKERS.get(project, "o")

            # Add data point
            scatter = ax.scatter(
                combined_data[project]['memory'],
                combined_data[project]['time'],
                color=color,
                marker=marker,
                s=150,  # Size of marker - larger for better visibility
                label=f"{project} ({PROOF_SYSTEMS.get(project, '')})",
                edgecolors='black',
                linewidths=0.5,
                alpha=0.8,
                zorder=3  # Ensure points are on top of grid lines
            )

            # Add project name as annotation
            ax.annotate(
                project,
                (combined_data[project]['memory'], combined_data[project]['time']),
                xytext=(7, 7),
                textcoords='offset points',
                fontsize=9,
                fontweight='bold',
                bbox=dict(boxstyle="round,pad=0.3", fc="white", alpha=0.7)
            )

            # Store for legend
            handles_by_group[arch_group].append(scatter)

    # Visualize architecture group regions
    for arch_group, group_info in ARCHITECTURE_GROUPS.items():
        group_projects = [p for p in group_info["projects"] if p in combined_data]

        if not group_projects:
            continue

        # Get the memory and time values for this group
        memory_values = [combined_data[p]['memory'] for p in group_projects]
        time_values = [combined_data[p]['time'] for p in group_projects]

        # # Only add group label if there are enough projects in this group
        # if len(group_projects) > 0:
        #     # Calculate geometric mean center for the label
        #     geo_mean_memory = np.exp(np.mean(np.log(memory_values)))
        #     geo_mean_time = np.exp(np.mean(np.log(time_values)))

        #     # Add group label
        #     ax.text(
        #         geo_mean_memory,
        #         geo_mean_time * 1.5,  # Shift up to avoid overlap
        #         arch_group,
        #         ha='center',
        #         va='center',
        #         fontsize=11,
        #         fontweight='bold',
        #         bbox=dict(
        #             boxstyle="round,pad=0.5",
        #             fc=group_info["color"],
        #             ec="gray",
        #             alpha=0.3
        #         ),
        #         zorder=2
        #     )

    # Grid and labels
    ax.grid(True, linestyle='--', alpha=0.6, zorder=1)
    ax.set_title(f"Fibonacci (n={target_n}) Peak Memory vs Prover Time", fontsize=14)
    ax.set_xlabel("Peak Memory (GB)", fontsize=12)
    ax.set_ylabel("Prover Time (s)", fontsize=12)

    # Set logarithmic scales for better visualization
    ax.set_xscale('log')
    ax.set_yscale('log')

    # Add legends grouped by architecture type
    legend_elements = []

    # Add group headers and project entries
    for group_name, handles in handles_by_group.items():
        if handles:  # Only add groups that have data
            # Add the group header
            legend_elements.append(Patch(facecolor='white', edgecolor='white', label=f"\n{group_name}"))
            # Add all projects within the group
            legend_elements.extend(handles)

    # Create the legend
    ax.legend(
        handles=legend_elements,
        loc='upper left',
        bbox_to_anchor=(1.05, 1),
        borderaxespad=0.,
        frameon=True,
        fontsize=10
    )

    plt.tight_layout()
    plt.show()