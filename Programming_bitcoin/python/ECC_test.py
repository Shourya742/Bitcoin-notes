from unittest import TestCase
from finite_element import FiniteElement
from point import Point


class ECCTest(TestCase):
    def test_on_curve(self):
        prime = 223
        a = FiniteElement(0, prime)
        b = FiniteElement(7, prime)
        valid_points = ((192, 105), (17, 56), (1, 193))
        invalid_points = ((200, 119), (42, 99))
        for x_raw, y_raw in valid_points:
            x = FiniteElement(x_raw, prime)
            y = FiniteElement(y_raw, prime)
            Point(x, y, a, b)
        for x_raw, y_raw in invalid_points:
            x = FiniteElement(x_raw, prime)
            y = FiniteElement(y_raw, prime)
            with self.assertRaises(ValueError):
                Point(x, y, a, b)

    def test_add(self):
        prime = 223
        a = FiniteElement(0, prime)
        b = FiniteElement(7, prime)
        additions = (
            (192, 105, 17, 56, 170, 142),
            (47, 71, 117, 141, 60, 139),
            (143, 98, 76, 66, 47, 71),
        )
        for x1_raw, y1_raw, x2_raw, y2_raw, x3_raw, y3_raw in additions:
            x1 = FiniteElement(x1_raw, prime)
            y1 = FiniteElement(y1_raw, prime)
            p1 = Point(x1, y1, a, b)
            x2 = FiniteElement(x2_raw, prime)
            y2 = FiniteElement(y2_raw, prime)
            p2 = Point(x2, y2, a, b)
            x3 = FiniteElement(x3_raw, prime)
            y3 = FiniteElement(y3_raw, prime)
            p3 = Point(x3, y3, a, b)
            self.assertEqual(p1 + p2, p3)
