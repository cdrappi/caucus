import unittest

from clients.python.client import get_index


class IndexTest(unittest.TestCase):

    def test_index(self):
        response = get_index()
        self.assertEqual(response.status_code, 200)
        self.assertEqual(response.text, "hello, world")
