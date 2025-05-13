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
# Thread Counts
THREAD_COUNT = [1, 2, 4, 8, 16]

# Adjusted max values to prevent value overflow
METRICS = {
    'proof_duration': {'title': 'Prover Time', 'unit': 's', 'divisor': 1e9, 'max_value': 2000, 'min_value': 0.1},
    'verify_duration': {'title': 'Verifier Time', 'unit': 'ms', 'divisor': 1e6, 'max_value': 3200, 'min_value': 1},  # 最小値を1msに変更
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

def load_data(data_dir='.', is_thread=False):
    """
    Load benchmark data for parallel scaling
    """
    all_data = []

    for program in PROGRAMS:
        for project in PROJECT_ORDER:
            if is_thread and (project.endswith('-gpu') != True):
                for thread_count in THREAD_COUNT:
                    filename = f"{program}_{project}-cpu{thread_count}.csv"
                    filepath = os.path.join(data_dir, filename)

                    try:
                        df = pd.read_csv(filepath)
                        df['program'] = program
                        df['project'] = project
                        df['thread_count'] = thread_count
                        all_data.append(df)


                    except FileNotFoundError:
                        print(f"Warning: {filepath} not found, skipping.")

            else:
                filename = f"{program}_{project}.csv"
                filepath = os.path.join(data_dir, filename)

                try:
                    df = pd.read_csv(filepath)
                    # Add columns for program and project
                    df['program'] = program
                    df['project'] = project
                    df['thread_count'] = THREAD_COUNT[-1]  # Default to the last thread count (16)
                    all_data.append(df)
                except FileNotFoundError:
                    print(f"Warning: {filepath} not found, skipping.")

    # すべてのデータを結合
    if all_data:
        return pd.concat(all_data, ignore_index=True)
    else:
        raise ValueError("No data files found.")

def create_fixed_size_grid(df, thread_count=None):
    """
    Create a grid of horizontal bar charts for all programs with their fixed sizes and all metrics

    Parameters:
    df (DataFrame): The dataframe containing benchmark data
    thread_count (int, optional): The thread count to filter by. If None, no filtering is applied.
    """
    # Create a figure with subplots - programs as rows, metrics as columns
    thread_count = 16 if thread_count is None else thread_count
    fig, axes = plt.subplots(len(PROGRAMS), len(METRICS),
                            figsize=(20, 16),
                            constrained_layout=True)

    # Title for the entire grid
    title = 'E2E Performance Comparison'
    if thread_count == 16:
        title += f' (Multi Threads)'
    elif thread_count == 1:
        title += f' (Single Thread)'
    fig.suptitle(title, fontsize=20)

    # Process each cell in the grid
    for i, program in enumerate(PROGRAMS):
        # Get the fixed size for this program
        size = PROGRAM_SIZES[program]

        for j, (metric_name, metric_info) in enumerate(METRICS.items()):
            ax = axes[i, j]

            # Filter data for this program and size
            df = df[df['thread_count'] == thread_count] if thread_count else df
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

def create_scaling_grid(df, thread_count=None):
    """Create a grid of scaling line charts for all programs and metrics

    Parameters:
    df (DataFrame): The dataframe containing benchmark data
    thread_count (int, optional): スレッド数でフィルタリングする場合に指定。Noneの場合はフィルタリングしない
    """
    PROGRAMS_NEW = ['fib', 'sha2', 'ethtransfer']

    # Create a figure with subplots - programs as rows, metrics as columns
    thread_count = 16 if thread_count is None else thread_count
    fig, axes = plt.subplots(len(PROGRAMS_NEW), len(METRICS),
                            figsize=(20, 16),
                            constrained_layout=True)

    # Title for the entire grid
    title = 'Scalability Curve Comparison'
    if thread_count == 16:
        title += f' (Multi Threads)'
    elif thread_count == 1:
        title += f' (Single Thread)'
    fig.suptitle(title, fontsize=20)

    # Process each cell in the grid
    for i, program in enumerate(PROGRAMS_NEW):
        for j, (metric_name, metric_info) in enumerate(METRICS.items()):
            ax = axes[i, j]

            # Filter data for this program
            df = df[df['thread_count'] == thread_count] if thread_count else df
            program_df = df[df['program'] == program].copy()  # Create copy to avoid warning

            if program_df.empty:
                ax.text(0.5, 0.5, f"No data for {program}",
                        ha='center', va='center', transform=ax.transAxes)
                continue

            # For each project, create a line
            legend_handles = []
            legend_labels = []

            for project in PROJECT_ORDER:
                project_df = program_df[program_df['project'] == project].copy()

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

                    # Add to legend only for the first row and column
                    if i == 0 and j == 0:
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

            # Set y-axis limits
            ax.set_ylim(metric_info['min_value'], metric_info['max_value'])

            # Add titles only for the top row
            if i == 0:
                ax.set_title(f'{metric_info["title"]}\n({metric_info["unit"]})', fontsize=12)

            # Add program name to the leftmost column
            if j == 0:
                ax.set_ylabel(f'{program.upper()}', fontsize=12, rotation=0, ha='right')

            # Add x-label only for the bottom row
            if i == len(PROGRAMS_NEW) - 1:
                ax.set_xlabel('Size', fontsize=10)

            # Add grid lines for both axes
            ax.grid(True, which='major', alpha=0.3, linestyle='-')
            ax.grid(True, which='minor', axis='x', alpha=0.1, linestyle='--')

    # Add a common legend at the top of the figure
    if 'legend_handles' in locals() and legend_handles:
        fig.legend(legend_handles, legend_labels,
                  loc='upper center', ncol=len(legend_handles),
                  bbox_to_anchor=(0.5, 0.98), fontsize=10)

        # Adjust the subplot positions to make room for the legend
        plt.subplots_adjust(top=0.9)

    plt.show()

def create_parallel_scaling_grid(df):
    """
    Create a grid of line charts for all programs and metrics with parallel scaling

    Parameters:
    - df (DataFrame): The dataframe containing benchmark data
    """
    fig, axes = plt.subplots(len(PROGRAMS), len(METRICS),
                           figsize=(22, 18))

    fig.suptitle('Parallel Performance', fontsize=20, y=0.98)

    available_projects = df['project'].unique()

    legend_handles = []
    legend_labels = []

    for i, program in enumerate(PROGRAMS):
        size = PROGRAM_SIZES[program]

        for j, (metric_name, metric_info) in enumerate(METRICS.items()):
            ax = axes[i, j]

            program_df = df[(df['program'] == program) & (df['size'] == size)].copy()

            if program_df.empty:
                ax.text(0.5, 0.5, f"No data for {program} (size={size})",
                        ha='center', va='center', transform=ax.transAxes)
                continue

            program_df.loc[:, 'converted_value'] = program_df[metric_name] / metric_info['divisor']

            min_values = []
            max_values = []

            for project in available_projects:
                project_df = program_df[program_df['project'] == project].copy()

                if project_df.empty:
                    continue

                project_df = project_df.sort_values(by='thread_count')

                project_df = project_df[project_df['converted_value'] > 0]

                if project_df.empty:
                    continue

                color = PROJECT_COLORS.get(project, 'gray')

                line, = ax.plot(project_df['thread_count'], project_df['converted_value'],
                               marker='o', color=color, linewidth=2, markersize=8,
                               label=project)

                if i == 0 and j == 0:
                    legend_handles.append(line)
                    legend_labels.append(project)

                for _, row in project_df.iterrows():
                    thread_count = row['thread_count']
                    value = row['converted_value']

                    if value < 1:
                        value_text = f'{value:.2f}'
                    elif value < 10:
                        value_text = f'{value:.1f}'
                    else:
                        value_text = f'{int(value)}'

                    ax.text(thread_count, value*1.05, value_text,
                           ha='center', va='bottom', fontsize=8)

                last_x = project_df['thread_count'].iloc[-1]
                last_y = project_df['converted_value'].iloc[-1]
                ax.annotate(project, xy=(last_x, last_y), xytext=(5, 0),
                          textcoords='offset points', va='center', fontsize=8)

                min_values.append(project_df['converted_value'].min())
                max_values.append(project_df['converted_value'].max())

            ax.set_xticks([1, 2, 4, 8, 16])
            ax.set_xticklabels([1, 2, 4, 8, 16])

            if metric_name in ['proof_duration', 'verify_duration']:
                ax.set_yscale('log')
                if min_values and max_values:
                    min_val = max(min(min_values) * 0.8, 1e-10)
                    max_val = max(max_values) * 1.2
                    ax.set_ylim(min_val, max_val)
            else:
                if min_values and max_values:
                    min_val = max(0, min(min_values) * 0.9)
                    max_val = max(max_values) * 1.1
                    ax.set_ylim(min_val, max_val)

            if i == 0:
                ax.set_title(f'{metric_info["title"]}\n({metric_info["unit"]})', fontsize=12)

            if j == 0:
                ax.set_ylabel(f'{program.upper()}\n(size={size})', fontsize=12, rotation=0, ha='right')

            if i == len(PROGRAMS) - 1:
                ax.set_xlabel('Thread Count', fontsize=10)

            ax.grid(True, which='major', alpha=0.3, linestyle='-')

            if metric_name in ['proof_duration', 'verify_duration']:
                ax.grid(True, which='minor', alpha=0.1, linestyle='--')

    plt.tight_layout(rect=[0, 0, 1, 0.95])

    plt.show()

def create_e2e_performance_table(df, thread_count=1):
    """
    Create a table for E2E performance metrics
    This function filters the data based on the specified thread count and formats the metrics for display.
    The table is structured with projects as rows and metrics as columns, with each program's data displayed.

    Parameters:
    - df (DataFrame): The dataframe containing benchmark data
    - thread_count (int): The thread count to filter by. Default is 1.

    Returns:
    - result_df (DataFrame): A DataFrame containing the formatted metrics for each project and program.
    """
    if thread_count not in THREAD_COUNT:
        print(f"Warning: Thread count {thread_count} is not in {THREAD_COUNT}. Using default value 1.")
        thread_count = 1

    # Filter the dataframe based on the thread count
    if 'thread_count' in df.columns:
        df = df[df['thread_count'] == thread_count].copy()

    metrics_list = [
        (name, info['title'], info['unit'], info['divisor'])
        for name, info in METRICS.items()
    ]

    result_df = pd.DataFrame()

    for project in PROJECT_ORDER:
        project_data = {}

        for metric_name, metric_title, metric_unit, divisor in metrics_list:
            metric_display = f"{metric_title} ({metric_unit})"

            # このメトリクスの各プログラムの値
            metric_values = {}

            # 各プログラムについて処理
            for program in PROGRAMS:
                # サイズを取得
                size = PROGRAM_SIZES[program]

                # このプロジェクト、プログラムの組み合わせでデータをフィルタリング
                filtered_df = df[(df['project'] == project) & (df['program'] == program) & (df['size'] == size)].copy()

                if filtered_df.empty:
                    # データがない場合はNaNを設定
                    metric_values[program] = np.nan
                else:
                    # サイズが最も近い行を検索
                    filtered_df = filtered_df.copy()
                    if 'size' in filtered_df.columns:
                        filtered_df['size_diff'] = abs(filtered_df['size'] - size)
                        closest_row = filtered_df.loc[filtered_df['size_diff'].idxmin()]

                        # メトリクス値を変換して設定（0の場合はNaN）
                        if metric_name in closest_row:
                            value = closest_row[metric_name]
                            # 0の場合はNaNに設定
                            if value == 0:
                                metric_values[program] = np.nan
                            else:
                                value = value / divisor
                                metric_values[program] = round(value, 2)
                        else:
                            metric_values[program] = np.nan
                    else:
                        # sizeカラムがない場合
                        metric_values[program] = np.nan

            # このメトリクスのデータをプロジェクトデータに追加
            project_data[metric_display] = metric_values

        # このプロジェクトのデータをデータフレームに変換
        project_df = pd.DataFrame()

        # メトリクスごとに処理
        for metric_name, metric_title, metric_unit, _ in metrics_list:
            metric_display = f"{metric_title} ({metric_unit})"

            if metric_display in project_data:
                # このメトリクスのデータを取得
                metric_data = project_data[metric_display]

                # 各プログラムについて列を作成
                for program in PROGRAMS:
                    col_name = (metric_display, program)
                    project_df[col_name] = [metric_data.get(program, np.nan)]

        # プロジェクト名をインデックスに設定
        project_df.index = [project]

        # 結果のデータフレームに追加
        if result_df.empty:
            result_df = project_df
        else:
            result_df = pd.concat([result_df, project_df])

    return result_df

def create_cycles_vs_prover_time_plot(df, thread_count=1, projects=None):
    """
    Create a plot showing the relationship between cycles and proof_duration for selected projects

    Parameters:
    - df (DataFrame): The dataframe containing benchmark data
    - thread_count (int): The thread count to filter by. Default is 1.
    - projects (list, optional): List of project names to include in the plot.
                                 If None, all projects in PROJECT_ORDER will be used.

    This function creates a multi-panel plot with one subplot per project,
    showing cycles vs proof_duration (prover time) regardless of size or program.
    """
    if thread_count not in THREAD_COUNT:
        print(f"Warning: Thread count {thread_count} is not in {THREAD_COUNT}. Using default value 1.")
        thread_count = 1

    # Filter the dataframe based on the thread count
    if 'thread_count' in df.columns:
        df = df[df['thread_count'] == thread_count].copy()

    # Keep only rows with both cycles and proof_duration
    plot_df = df[df['cycles'].notna() & df['proof_duration'].notna()].copy()

    if plot_df.empty:
        print("No data available with both cycles and proof_duration. Cannot create plot.")
        return

    # Convert proof_duration to seconds for display
    plot_df['proof_duration_sec'] = plot_df['proof_duration'] / 1e9

    # Use specified projects or default to PROJECT_ORDER
    if projects is None:
        all_projects = PROJECT_ORDER
    else:
        all_projects = projects if isinstance(projects, list) else [projects]

    # Determine the projects that exist in the data
    projects_to_plot = [p for p in all_projects if p in plot_df['project'].unique()]

    if not projects_to_plot:
        print("No specified projects found in the data. Cannot create plot.")
        return

    # Calculate the grid dimensions based on the number of projects
    n_projects = len(projects_to_plot)
    n_cols = min(3, n_projects)  # Max 3 columns
    n_rows = (n_projects + n_cols - 1) // n_cols  # Ceiling division

    # Create the figure and subplots
    fig, axes = plt.subplots(n_rows, n_cols, figsize=(15, 4 * n_rows), squeeze=False)

    # Find global min and max for consistent axes across subplots
    min_cycles = plot_df['cycles'].min()
    max_cycles = plot_df['cycles'].max()
    min_duration = plot_df['proof_duration_sec'].min()
    max_duration = plot_df['proof_duration_sec'].max()

    # Add some padding to the limits
    min_cycles = max(1, min_cycles * 0.9)  # Avoid log scale issues with zero or negative values
    max_cycles = max_cycles * 1.1
    min_duration = max(0.1, min_duration * 0.9)  # Avoid log scale issues
    max_duration = max_duration * 1.1

    # Flatten axes array for easier indexing
    axes_flat = axes.flatten()

    # Plot each project in its own subplot
    for i, project in enumerate(projects_to_plot):
        ax = axes_flat[i]

        # Filter data for this project
        project_df = plot_df[plot_df['project'] == project]

        if project_df.empty:
            ax.text(0.5, 0.5, f"No data for {project}", ha='center', va='center', transform=ax.transAxes)
            continue

        # Group by program to use different colors and markers
        for program, group in project_df.groupby('program'):
            color = {'fib': 'blue', 'sha2': 'red', 'ecdsa': 'green', 'ethtransfer': 'purple'}.get(program, 'gray')

            # Sort by cycles for better line plotting
            group = group.sort_values('cycles')

            # Plot the line
            ax.plot(group['proof_duration_sec'], group['cycles'],
                   marker='o', linestyle='-', label=program, color=color)

        # Set the title and labels
        ax.set_title(f"{project}", fontsize=12)
        ax.set_xlabel("Prover Time (s)", fontsize=10)
        ax.set_ylabel("Cycles", fontsize=10)

        # Set log scales for both axes
        ax.set_xscale('log')
        ax.set_yscale('log')

        # Set consistent limits across subplots
        ax.set_xlim(min_duration, max_duration)
        ax.set_ylim(min_cycles, max_cycles)

        # Add grid for better readability
        ax.grid(True, which='both', alpha=0.3, linestyle='-')

        # Add legend
        ax.legend(fontsize=8, loc='best')

    # Hide any unused subplots
    for i in range(n_projects, len(axes_flat)):
        axes_flat[i].set_visible(False)

    # Create title with project information
    if len(projects_to_plot) == 1:
        plot_title = f'Cycles vs Prover Time for {projects_to_plot[0]} (Thread Count: {thread_count})'
    else:
        if len(projects_to_plot) <= 3:
            project_list = ', '.join(projects_to_plot)
            plot_title = f'Cycles vs Prover Time for {project_list} (Thread Count: {thread_count})'
        else:
            plot_title = f'Cycles vs Prover Time for {len(projects_to_plot)} Projects (Thread Count: {thread_count})'

    # Add a title for the entire figure
    fig.suptitle(plot_title, fontsize=16, y=0.99)

    # Adjust layout
    plt.tight_layout(rect=[0, 0, 1, 0.97])

    plt.show()