"""
Split the quest_helper migration SQL into individual batch statements
and print them for execution.
"""
import re

INPUT_FILE = r"C:\Projects\KnightOnlineRustServer\crates\ko-db\migrations\20260212000007_seed_quest_helper.sql"

def main():
    with open(INPUT_FILE, "r", encoding="utf-8") as f:
        content = f.read()

    # Split by "ON CONFLICT" to find each batch
    # Each batch is: INSERT INTO ... VALUES ... ON CONFLICT (n_index) DO NOTHING;
    statements = []
    current = []
    for line in content.split("\n"):
        if line.startswith("--") and not current:
            continue
        if line.strip() == "":
            if current:
                statements.append("\n".join(current))
                current = []
            continue
        current.append(line)
    if current:
        statements.append("\n".join(current))

    # Filter out empty/comment-only statements
    sql_statements = [s for s in statements if s.strip().startswith("INSERT")]

    print(f"Found {len(sql_statements)} batch statements")

    # Write each as a separate file for reference
    for i, stmt in enumerate(sql_statements):
        outfile = rf"C:\Projects\KnightOnlineRustServer\scripts\batch_{i+1:02d}.sql"
        with open(outfile, "w", encoding="utf-8") as f:
            f.write(stmt + "\n")

    print(f"Written {len(sql_statements)} batch files")

if __name__ == "__main__":
    main()
