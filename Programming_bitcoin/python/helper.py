from unittest import TestSuite, TextTestResult

import hashlib


def run(test):
    suite = TestSuite()
    suite.addTest(test)
    TextTestResult().run(suite)


def hash256(s):
    return hashlib.sha256(hashlib.sha256(s).digest()).digest()
