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

def rust_file_to_scenarios(f: Path) -> list[str]:
    ret: list[str] = []
    for i, line in  enumerate(f.read_text().splitlines(),start=1):
        if "Scenario Outline:" in line:
            line = line.replace("Scenario Outline:", "Scenario:")

        if "Scenario:" in line or "#[test]" in line:
            if  "#[test]" in line and line.strip() != "#[test]":
                raise AssertionError(f"unexpected #[test] on {f}:{i} : {line}")
            ret.append(line.strip(" /"))

    implemented = []
    # Scenario, test, Scenario, test
    for i in range(len(ret)-1):
        if ret[i].startswith("Scenario") and ret[i+1].startswith("#[test]"):
            implemented.append(ret[i])


    return implemented


def display_coverage():
    scenario_files: list[Path] = list(Path("scenarios").rglob("*.feature"))
    rust_files = list(Path("src").rglob("*.rs"))

    implemented = []
    bad = []
    for f in rust_files:
        temp = rust_file_to_scenarios(f)
        for t in temp:
            if t in implemented:
                bad.append( (f, t))
        implemented.extend(temp)

    assert not bad, bad
    unique_implemented = set(implemented)
    bad = []
    rows = []
    seen = set()
    for f in sorted(scenario_files):
        for_file = file_to_scenarios(f)

        for s in for_file:
            if s in seen or for_file.count(s) > 1:
                bad.append( (f, s))

        seen.update(for_file)
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
    assert not bad, bad
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
