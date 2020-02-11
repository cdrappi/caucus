#!/usr/bin/env python
# coding: utf-8

import pandas
import datetime
from db.pgdb import Pgdb
from db.util import title_to_slug
import os


pgdb = Pgdb(autocommit=True)


""" candidates """
candidates_df = pandas.read_csv(
    '/Users/cld/Downloads/Nevada caucus - candidates.csv')
pgdb.reset_df('candidates', candidates_df,
              primary_key="candidate", reset_serial=False)

""" caucuses """
caucuses_df = pandas.read_csv(
    '/Users/cld/Downloads/Nevada caucus - caucuses.csv')
pgdb.reset_df('caucus', caucuses_df,
              primary_key="state_code", reset_serial=False)

""" counties """
counties_df = pandas.read_csv(
    '/Users/cld/Downloads/Nevada caucus - counties.csv')
pgdb.reset_df('counties', counties_df,
              primary_key="county", reset_serial=False)

""" precincts """
precincts_df = pandas.read_csv(
    '/Users/cld/Downloads/Nevada caucus - precincts.csv')
pgdb.reset_df('precincts', precincts_df,
              primary_key="precinct", reset_serial=False)
