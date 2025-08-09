from pathlib import Path
import tabulate

def file_to_scenarios(f:Path) -> list[str]:
    ret:list[str] = []
    for line in f.read_text().splitlines():
        if "Scenario:" in line:
            ret.append(line.strip(" /"))
    return ret

def display_coverage():
    scenario_files:list[Path] = list(Path("scenarios").rglob("*.feature"))
    rust_files = list(Path("src").rglob("*.rs"))

    implemented = []
    for f in rust_files:
        implemented.extend(file_to_scenarios(f))

    unique_implemented = set(implemented)

    rows = []
    for f in scenario_files:
        for_file = file_to_scenarios(f)

        covered = [s for s in for_file if s in unique_implemented]
        uncovered = [s for s in for_file if s not in unique_implemented]

        if covered and uncovered:
            for u in uncovered:
                print(f.stem, u)

        rows.append((f.stem, len(covered), len(uncovered) ))

    rows.sort()
    total_covered = sum(r[1] for r in rows)
    total_uncovered = sum(r[2] for r in rows)
    rows.append(("",))
    rows.append(("Total", total_covered, total_uncovered))

    print(tabulate.tabulate(rows, headers=["File", "Covered", "Uncovered"]))

    print()
    percentage = total_covered / (total_covered + total_uncovered) * 100
    print(f"Coverage: {percentage:.2f}%")

if __name__ == "__main__":
    display_coverage()
