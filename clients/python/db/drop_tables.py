#!/usr/bin/env python
# coding: utf-8

from db.pgdb import Pgdb
import os


pgdb = Pgdb(autocommit=True)

pgdb.execute_sql("""
    DO $$ DECLARE
        tabname RECORD;
    BEGIN
        FOR tabname IN (SELECT tablename 
                        FROM pg_tables 
                        WHERE schemaname = current_schema()) 
    LOOP
        EXECUTE 'DROP TABLE IF EXISTS ' || quote_ident(tabname.tablename) || ' CASCADE';
    END LOOP;
    END $$;
""")
