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
        "projects": ["sp1turbo", "sp1turbo-gpu", "risczero", "risczero-gpu", "openvm", "pico", "zkm", "jolt", "jolt-gpu", "nexus", "novanet"],
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
    "pico": "FRI-STARK",
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
    "pico": "royalblue",
    "nexus": "blue",
    "novanet": "purple",
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
    "pico": "o",
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
            'size': 'peak_memory'
        },
        "conversion_factors": {
            'n': 1024,  # from MB to GB
            'size': 1024 * 1024 * 1024  # from B to GB
        }
    },
    "cycles": {
        "title": "Cycle Counts",
        "y_label": "Cycles",
        "column_names": {
            'n': 'cycles',
            'size': 'user_cycles'
        },
        "conversion_factors": {
            'n': 1,  # No conversion needed
            'size': 1  # No conversion needed
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
        file_list = glob.glob(f"20250414/{prog}_*.csv")

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

def plot_programs_by_project_simple(
    metrics="prover_time",
    selected_projects=None,
    selected_programs=None,
    hidden_projects=None,
):
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
        programs = ["fib", "sha2", "ecdsa", "ethtransfer"]

    if not programs:
        print("No valid programs selected.")
        return

    if len(programs) == 1:
        PROGRAM_COLOR_INTENSITY = {
            selected_programs[0]: 1.0
        }
    else:
        PROGRAM_COLOR_INTENSITY = {
            "fib": 1.0,
            "sha2": 0.8,
            "ecdsa": 0.6,
            "ethtransfer": 0.4
        }

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

def get_cycle_counts(selected_projects=None, selected_programs=None):
    """
    Display cycle counts for each project and program in a table format.
    For risczero and cycles projects, it looks for 'user_cycles' column instead of 'cycles'.

    Parameters:
        selected_projects (list): List of projects to display. If None, all projects are displayed.
        selected_programs (list): List of programs to display. If None, default programs are used.
    """
    # Adjust the program list based on selected_programs
    if selected_programs is not None:
        # Filter selected programs that are valid
        valid_programs = ["fib", "sha2", "ecdsa", "ethtransfer"]
        programs = [prog for prog in selected_programs if prog in valid_programs]
    else:
        programs = ["fib", "sha2", "ecdsa", "ethtransfer"]

    if not programs:
        print("No valid programs selected.")
        return

    # Collect data
    all_data, all_existing_projects = collect_data(programs, DEFAULT_N_VALUES, "cycles")

    # Filter projects if selected_projects is provided
    if selected_projects is not None:
        projects_to_display = [p for p in all_existing_projects if p in selected_projects]
    else:
        projects_to_display = all_existing_projects

    if not projects_to_display:
        print(f"No data found for cycles with the selected projects and programs.")
        return

    # Create DataFrame for displaying the table
    table_data = []
    for project in projects_to_display:
        row_data = {'Project': project, 'Proof System': PROOF_SYSTEMS.get(project, '')}

        for program in programs:
            n_value = DEFAULT_N_VALUES[program]
            program_col = f"{program} (n={n_value})"

            # Get cycle count for this project and program
            if program in all_data and project in all_data[program] and n_value in all_data[program][project]:
                cycle_count = all_data[program][project][n_value]
                if cycle_count == 0 or cycle_count is None:
                    row_data[program_col] = "N/A"
                else:
                    # Format as integer with commas for better readability
                    row_data[program_col] = f"{int(cycle_count):,}"
            else:
                row_data[program_col] = "N/A"

        table_data.append(row_data)

    # Create DataFrame and display
    df_table = pd.DataFrame(table_data)

    # Set column order
    columns = ['Project', 'Proof System']
    for program in programs:
        columns.append(f"{program} (n={DEFAULT_N_VALUES[program]})")

    # Reorder columns and display
    df_table = df_table[columns]

    # Set display options for better formatting
    pd.set_option('display.max_columns', None)
    pd.set_option('display.width', None)
    pd.set_option('display.max_colwidth', None)

    return df_table