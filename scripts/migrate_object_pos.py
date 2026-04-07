"""Migrate K_OBJECTPOS2369 from MSSQL to PostgreSQL object_event_pos table."""
import os
import pymssql
import psycopg2

MSSQL_HOST = os.environ.get("MSSQL_HOST", "localhost")
MSSQL_PORT = int(os.environ.get("MSSQL_PORT", "1433"))
MSSQL_USER = os.environ.get("MSSQL_USER", "sa")
MSSQL_PASS = os.environ.get("MSSQL_PASS", "changeme")
MSSQL_DB = os.environ.get("MSSQL_DB", "KO_DATABASE_SERVER_00125")

PG_HOST = os.environ.get("PG_HOST", "localhost")
PG_PORT = int(os.environ.get("PG_PORT", "5432"))
PG_DB = os.environ.get("PG_DB", "ko_server")
PG_USER = os.environ.get("PG_USER", "koserver")
PG_PASS = os.environ.get("PG_PASS", "changeme")

def main():
    # Connect MSSQL
    ms = pymssql.connect(MSSQL_HOST, MSSQL_USER, MSSQL_PASS, MSSQL_DB, port=MSSQL_PORT)
    ms_cur = ms.cursor()

    # Connect PostgreSQL
    pg = psycopg2.connect(host=PG_HOST, port=PG_PORT, dbname=PG_DB, user=PG_USER, password=PG_PASS)
    pg_cur = pg.cursor()

    # Read from MSSQL
    ms_cur.execute("SELECT ZoneID, Belong, sIndex, Type, ControlNpcID, Status, PosX, PosY, PosZ, byLife FROM K_OBJECTPOS2369")
    rows = ms_cur.fetchall()
    print(f"Read {len(rows)} rows from MSSQL")

    # Insert into PostgreSQL
    insert_sql = """
        INSERT INTO object_event_pos (zone_id, belong, s_index, obj_type, control_npc, status, pos_x, pos_y, pos_z, by_life)
        VALUES (%s, %s, %s, %s, %s, %s, %s, %s, %s, %s)
        ON CONFLICT DO NOTHING
    """

    for row in rows:
        pg_cur.execute(insert_sql, row)

    pg.commit()
    print(f"Inserted {len(rows)} rows into PostgreSQL object_event_pos")

    ms.close()
    pg.close()

if __name__ == "__main__":
    main()
