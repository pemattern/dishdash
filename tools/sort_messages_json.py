import json
from pathlib import Path

MESSAGES_DIR = Path("../web/messages")

for file_path in MESSAGES_DIR.glob("*.json"):
    with open(file_path, "r", encoding="utf-8") as f:
        data = json.load(f)

    # Keep $schema at the top
    schema = data.pop("$schema", None)

    # Sort remaining messages alphabetically
    sorted_data = dict(sorted(data.items(), key=lambda x: x[0].lower()))

    if schema is not None:
        sorted_data = {"$schema": schema, **sorted_data}

    with open(file_path, "w", encoding="utf-8") as f:
        json.dump(sorted_data, f, ensure_ascii=False, indent=2)
        f.write("\n")

    print(f"Sorted: {file_path}")

print("All message files sorted!")
