import psycopg2
from urllib.parse import urlparse, parse_qs
import sys
import os

# --- Configuration ---

# Use your provided DB URL
DATABASE_URL = os.env("DATABASE_URL")

# New database and user configuration
NEW_DB_NAME = "socio_v2"
TARGET_USER = "postgres"  # or change to another user if needed

# --- Parse connection info ---
parsed_url = urlparse(DATABASE_URL)
host = parsed_url.hostname
port = parsed_url.port
user = parsed_url.username
password = parsed_url.password

# --- Connect to default postgres DB to issue create command ---
try:
    conn = psycopg2.connect(
        dbname="postgres",  # Connect to default DB
        user=user,
        password=password,
        host=host,
        port=port
    )
    conn.autocommit = True
    cur = conn.cursor()

    # Drop if exists (optional safety)
    print(f"Creating database '{NEW_DB_NAME}'...")
    cur.execute(f"DROP DATABASE IF EXISTS {NEW_DB_NAME};")
    cur.execute(f"CREATE DATABASE {NEW_DB_NAME};")

    print(f"Granting all privileges on '{NEW_DB_NAME}' to user '{TARGET_USER}'...")
    cur.execute(f"GRANT ALL PRIVILEGES ON DATABASE {NEW_DB_NAME} TO {TARGET_USER};")

    print("✅ Done.")

except Exception as e:
    print(f"❌ Error: {e}")
    sys.exit(1)
finally:
    if conn:
        conn.close()
