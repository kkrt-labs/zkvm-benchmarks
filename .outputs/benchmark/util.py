import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
import numpy as np
import os
from matplotlib.ticker import FuncFormatter, LogFormatter, ScalarFormatter, StrMethodFormatter

# Define constants
PROGRAMS = ['fib', 'sha2', 'ecdsa', 'ethtransfer']
# Fixed order of projects (reversed from original to appear top-to-bottom)
PROJECT_ORDER = ['sp1', 'sp1-gpu', 'risczero', 'risczero-gpu', 'openvm', 'pico', 'zkm', 'nexus', 'jolt']
# Reversed order for plotting (to display from top to bottom)
PLOT_ORDER = PROJECT_ORDER[::-1]

# Adjusted max values to prevent value overflow
METRICS = {
    'proof_duration': {'title': 'Proof Duration', 'unit': 's', 'divisor': 1e9, 'max_value': 400, 'min_value': 0.1},
    'verify_duration': {'title': 'Verify Duration', 'unit': 'ms', 'divisor': 1e6, 'max_value': 2200, 'min_value': 0.001},
    'proof_bytes': {'title': 'Proof Size', 'unit': 'KB', 'divisor': 1024, 'max_value': 11000, 'min_value': 10},
    'peak_memory': {'title': 'Peak Memory', 'unit': 'GB', 'divisor': 1024**3, 'max_value': 140, 'min_value': 0.1}
}

# Fixed size for each program
PROGRAM_SIZES = {
    'fib': 100000,
    'sha2': 2048,
    'ecdsa': 1,
    'ethtransfer': 100
}

# Custom color palette
PROJECT_COLORS = {
    "jolt": "green",
    "jolt-gpu": "darkgreen",
    "risczero": "gold",
    "risczero-gpu": "orange",
    "sp1": "pink",
    "sp1-gpu": "magenta",
    "zkm": "navy",
    "openvm": "silver",
    "pico": "royalblue",
    "nexus": "blue",
    "novanet": "purple",
}

def load_data(data_dir='.'):
    """Load all benchmark CSV files and combine them into a structured dataframe"""
    all_data = []

    # Find all CSV files matching the pattern
    for program in PROGRAMS:
        for project in PROJECT_ORDER:
            filename = f"{program}_{project}.csv"
            filepath = os.path.join(data_dir, filename)

            try:
                df = pd.read_csv(filepath)
                # Add columns for program and project
                df['program'] = program
                df['project'] = project
                all_data.append(df)
            except FileNotFoundError:
                print(f"Warning: {filepath} not found, skipping.")

    # Combine all data
    if all_data:
        return pd.concat(all_data, ignore_index=True)
    else:
        raise ValueError("No data files found.")

def create_fixed_size_grid(df):
    """Create a grid of horizontal bar charts for all programs with their fixed sizes and all metrics"""

    # Create a figure with subplots - programs as rows, metrics as columns
    fig, axes = plt.subplots(len(PROGRAMS), len(METRICS),
                            figsize=(20, 16),
                            constrained_layout=True)

    # Title for the entire grid
    fig.suptitle('E2E Comparison', fontsize=20)

    # Process each cell in the grid
    for i, program in enumerate(PROGRAMS):
        # Get the fixed size for this program
        size = PROGRAM_SIZES[program]

        for j, (metric_name, metric_info) in enumerate(METRICS.items()):
            ax = axes[i, j]

            # Filter data for this program and size
            program_df = df[(df['program'] == program) & (df['size'] == size)].copy()  # Create copy to avoid warning

            if program_df.empty:
                ax.text(0.5, 0.5, f"No data for {program} (size={size})",
                        ha='center', va='center', transform=ax.transAxes)
                continue

            # Convert values to the specified units (using loc to avoid warning)
            program_df.loc[:, f'{metric_name}_converted'] = program_df[metric_name] / metric_info['divisor']

            # Filter out entries with zero or negative values
            program_df = program_df[program_df[f'{metric_name}_converted'] > 0]

            # Create a new DataFrame with all projects in the fixed order (REVERSED for top-to-bottom display)
            plot_data = []
            for project in PLOT_ORDER:  # Use reversed order for plotting
                project_data = program_df[program_df['project'] == project]
                if not project_data.empty:
                    plot_data.append(project_data)

            if not plot_data:
                ax.text(0.5, 0.5, f"No data for {program} (size={size})",
                        ha='center', va='center', transform=ax.transAxes)
                continue

            ordered_df = pd.concat(plot_data, ignore_index=True)

            # Set project colors using the custom palette
            project_colors = [PROJECT_COLORS.get(p, 'gray') for p in ordered_df['project']]

            # Set logarithmic x-axis scale before creating bars
            ax.set_xscale('log')

            # Set x-axis limits
            ax.set_xlim(metric_info['min_value'], metric_info['max_value'])

            # Create horizontal bar chart
            bars = ax.barh(ordered_df['project'], ordered_df[f'{metric_name}_converted'],
                          color=project_colors)

            # Add values at the end of each bar - simplified version
            for bar, (_, row) in zip(bars, ordered_df.iterrows()):
                width = row[f'{metric_name}_converted']

                # Simply place text at the end of the bar
                text_x = width * 1.1

                # Format the value without decimals for readability
                if width < 1:
                    value_text = f'{width:.2f}'
                elif width < 10:
                    value_text = f'{width:.1f}'
                else:
                    value_text = f'{width:.0f}'

                ax.text(text_x, bar.get_y() + bar.get_height()/2,
                       value_text, va='center', fontsize=10, color='black', fontweight='bold')

            # Format x-axis to look cleaner
            formatter = ScalarFormatter()
            formatter.set_scientific(False)
            ax.xaxis.set_major_formatter(formatter)
            ax.tick_params(axis='x', labelsize=9)

            # Add titles only for the top row and leftmost column
            if i == 0:
                ax.set_title(f'{metric_info["title"]}\n({metric_info["unit"]})', fontsize=12)

            # Add program name and size to the leftmost column
            if j == 0:
                ax.set_ylabel(f'{program.upper()}\n(size={size})', fontsize=12, rotation=0, ha='right')

            # Remove y-tick labels for all but the leftmost column
            if j > 0:
                ax.set_yticklabels([])
            else:
                ax.tick_params(axis='y', labelsize=10)  # Increase y-tick label size

            # Add grid for better readability with logarithmic scale
            ax.grid(True, which='major', axis='x', alpha=0.3, linestyle='-')
            ax.grid(True, which='minor', axis='x', alpha=0.1, linestyle='--')

    plt.show()

def create_scaling_charts(df):
    """Create scaling charts for each program with all metrics in one figure"""

    # For each program, create a figure with all metrics
    for program in PROGRAMS:
        # Filter data for this program
        program_df = df[df['program'] == program].copy()  # Create copy to avoid warning

        if program_df.empty:
            print(f"No data available for {program}")
            continue

        # Create a figure with subplots - one for each metric
        fig, axes = plt.subplots(2, 2, figsize=(16, 12), constrained_layout=True)
        axes = axes.flatten()

        # Title for the entire figure
        fig.suptitle(f'{program.upper()} Scaling Comparison', fontsize=18)

        # Process each metric
        for i, (metric_name, metric_info) in enumerate(METRICS.items()):
            ax = axes[i]

            # For each project in fixed order, create a line
            legend_handles = []
            legend_labels = []

            for project in PROJECT_ORDER:  # Keep original order for line graphs
                project_df = program_df[program_df['project'] == project].copy()  # Create copy to avoid warning

                if not project_df.empty:
                    # Sort by size and convert values
                    project_df = project_df.sort_values(by='size')
                    project_df.loc[:, 'converted_value'] = project_df[metric_name] / metric_info['divisor']

                    # Filter out zero or negative values
                    project_df = project_df[project_df['converted_value'] > 0]

                    if project_df.empty:
                        continue

                    # Get color from custom palette
                    color = PROJECT_COLORS.get(project, 'gray')

                    # Plot the line
                    line, = ax.plot(project_df['size'], project_df['converted_value'],
                                   marker='o', color=color, label=project)

                    # Add to legend
                    legend_handles.append(line)
                    legend_labels.append(project)

                    # Add project name at the end of the line
                    if len(project_df) > 0:
                        last_x = project_df['size'].iloc[-1]
                        last_y = project_df['converted_value'].iloc[-1]
                        ax.annotate(project, xy=(last_x, last_y), xytext=(5, 0),
                                   textcoords='offset points', va='center', fontsize=9)

            # Set both axes to log scale
            ax.set_xscale('log')
            ax.set_yscale('log')

            # Set y-axis limits with adjusted max value
            ax.set_ylim(metric_info['min_value'], metric_info['max_value'])

            # Add title for this subplot
            ax.set_title(f'{metric_info["title"]} ({metric_info["unit"]})', fontsize=12)

            # Add x and y labels
            ax.set_xlabel('Size', fontsize=10)
            ax.set_ylabel(metric_info['title'], fontsize=10)

            # Add grid lines for both axes
            ax.grid(True, which='major', alpha=0.3, linestyle='-')
            ax.grid(True, which='minor', alpha=0.1, linestyle='--')

            # Add legend if it's the first subplot
            if i == 0 and legend_handles:
                ax.legend(legend_handles, legend_labels, loc='upper left', fontsize=8)

        plt.show()
