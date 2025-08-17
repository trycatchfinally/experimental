from pathlib import Path
import tabulate
from rich import print


def file_to_scenarios(f: Path) -> list[str]:
    ret: list[str] = []
    for line in f.read_text().splitlines():
        if "Scenario Outline:" in line:
            line = line.replace("Scenario Outline:", "Scenario:")

        if "Scenario:" in line:
            ret.append(line.strip(" /"))
    return ret


def display_coverage():
    scenario_files: list[Path] = list(Path("scenarios").rglob("*.feature"))
    rust_files = list(Path("src").rglob("*.rs"))

    implemented = []
    for f in rust_files:
        implemented.extend(file_to_scenarios(f))

    unique_implemented = set(implemented)

    rows = []
    for f in sorted(scenario_files):
        for_file = file_to_scenarios(f)

        covered = [s for s in for_file if s in unique_implemented]
        uncovered = [s for s in for_file if s not in unique_implemented]

        if covered and uncovered:
            for u in uncovered:
                print(f.stem, u)

        assert covered or uncovered, f"No scenarios covered or uncovered in {f}"

        # use the rich.emoji class to distinguish between fully covered, partially covered, and uncovered
        if len(covered) == len(for_file):
            icon = ":green_circle:"  # All scenarios covered
        elif len(covered) > 0:
            icon = ":yellow_circle:"  # Some scenarios covered
        else:
            icon = ":red_square:"  # No scenarios covered

        rows.append((f.stem, len(covered), len(uncovered), icon))

    rows.sort()
    total_covered = sum(r[1] for r in rows)
    total_uncovered = sum(r[2] for r in rows)
    rows.append(("",))
    rows.append(("Total", total_covered, total_uncovered))

    table = tabulate.tabulate(rows, headers=["File", "Covered", "Uncovered", " ?"])
    table = table.replace("---------------\n", "--\n")
    print(table)
    print()
    percentage = total_covered / (total_covered + total_uncovered) * 100
    print(f"Coverage: {percentage:.2f}%")


if __name__ == "__main__":
    display_coverage()
