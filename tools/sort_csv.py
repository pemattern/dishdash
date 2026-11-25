import csv
from pathlib import Path

FOLDER = Path("server/seed_data")


def sort_csv(file_path: Path):
    with file_path.open("r", newline="", encoding="utf-8-sig") as f:
        reader = csv.DictReader(f)
        if reader.fieldnames is None:
            raise ValueError(f"CSV file has no header: {file_path}")

        columns = reader.fieldnames
        rows = list(reader)

    rows.sort(
        key=lambda row: tuple(
            (row[col] or "").casefold() for col in columns
        )
    )

    with file_path.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=columns)
        writer.writeheader()
        writer.writerows(rows)


if __name__ == "__main__":
    for file_path in sorted(FOLDER.glob("*.csv")):
        sort_csv(file_path)
        print(f"Sorted: {file_path}")
