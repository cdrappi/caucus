import os
from typing import List, Dict
import psycopg2
import psycopg2.extras
import pandas


class Pgdb:
    """ psycopg2 wrapper to query postgres database """

    def __init__(self, url: str = None, autocommit: bool = False):
        """
        :param url: (str)
        """
        if url is None:
            url = os.environ['DATABASE_URL']

        self._connection = psycopg2.connect(url)
        self._connection.set_session(autocommit=autocommit)

        self._cursor = self._connection.cursor()

    def commit(self):
        self._connection.commit()

    def execute_sql(self, sql):
        self._cursor.execute(sql)

    def select_table(self, table_name: str) -> List[Dict]:
        """
        :param table_name: (str)
        :return: (List[Dict])
        """
        return self._select_table(self._cursor, table_name)

    def insert_df(self,
                  table_name: str,
                  df: pandas.DataFrame,
                  page_size: int = 100,
                  primary_key: str = "id",
                  reset_serial: bool = True):
        """
        :param table_name: (str)
        :param df: (pandas.DataFrame)
        :param page_size: (int)
        """
        self._insert_df(self._cursor, table_name, df, page_size, primary_key)

    def insert_csv(self,
                   table_name: str,
                   csv_path: str,
                   page_size: int = 100,
                   primary_key: str = "id"):
        """
        :param table_name: (str)
        :param csv_path: (str)
        :param page_size: (int)
        """
        self._insert_df(
            cursor=self._cursor,
            table_name=table_name,
            df=pandas.read_csv(csv_path),
            page_size=page_size,
            primary_key=primary_key
        )

    def reset_df(self,
                 table_name: str,
                 df: pandas.DataFrame,
                 page_size: int = 100,
                 primary_key: str = "id",
                 reset_serial: bool = True):
        """
        :param table_name: (str)
        :param df: (pandas.DataFrame)
        :param page_size: (int)
        """
        self._truncate_table(self._cursor, table_name)
        self._insert_df(self._cursor, table_name, df,
                        page_size, primary_key, reset_serial)

    @staticmethod
    def _truncate_table(cursor: psycopg2.extensions.cursor, table_name: str):
        cursor.execute(f"TRUNCATE TABLE {table_name} CASCADE;")

    @staticmethod
    def _select_table(cursor: psycopg2.extensions.cursor,
                      table_name: str):
        cursor.execute(f"SELECT * from {table_name}")
        return cursor.fetchall()

    @staticmethod
    def _insert_df(cursor: psycopg2.extensions.cursor,
                   table_name: str,
                   df: pandas.DataFrame,
                   page_size: int,
                   primary_key: str,
                   reset_serial: bool):
        columns = list(df.columns)
        comma_joined_columns = ', '.join(columns)
        sql = f"""
            INSERT INTO {table_name} ({comma_joined_columns})
            VALUES %s
            ON CONFLICT ({primary_key})
            DO NOTHING;
        """
        inner_template = ', '.join(f'%({col})s' for col in columns)
        psycopg2.extras.execute_values(
            cur=cursor,
            sql=sql,
            argslist=Pgdb.df_to_records(df),
            template=f"({inner_template})",
            page_size=page_size
        )

        if reset_serial:
            cursor.execute(
                f"SELECT "
                f"setval(pg_get_serial_sequence('{table_name}', '{primary_key}'), "
                f"(SELECT MAX({primary_key}) FROM {table_name}) + 1);"
            )

    @staticmethod
    def df_to_records(data_frame: pandas.DataFrame) -> List[Dict]:
        return [
            Pgdb.replace_nan(row)
            for row in data_frame.to_dict(orient='records')
        ]

    @staticmethod
    def replace_nan(row: Dict, replace_value=None):
        return {
            k: replace_value if pandas.isna(v) else v
            for k, v in row.items()
        }
