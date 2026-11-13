import csv
import sys


def sort_csv(file_path: str):
    with open(file_path, "r", newline="", encoding="utf-8-sig") as f:
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
    with open(file_path, "w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=columns)
        writer.writeheader()
        writer.writerows(rows)


if __name__ == "__main__":
    sort_csv(sys.argv[1])
