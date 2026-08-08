class Rect:
    def __init__(self, width, height):
        self.width = width
        self.height = height

    def area(self):
        return self.width * self.height


class Circle:
    def __init__(self, radius):
        self.radius = radius

    def area(self):
        return self.radius * self.radius * 3.14


# This function accepts ANY object that has an area() method
def get_area(s):
    return s.area()


rect = Rect(10, 20)
circle = Circle(10)

# Calling directly — works
print("Direct call:")
print(f"  rect.area()   = {rect.area()}")
print(f"  circle.area() = {circle.area()}")

# Passing object to function — ALSO works (same result)
print("\nVia get_area() function:")
print(f"  get_area(rect)   = {get_area(rect)}")
print(f"  get_area(circle) = {get_area(circle)}")

# Both give the SAME answer because:
# get_area(rect)  → s = rect  → s.area() → rect.area()
# get_area(circle) → s = circle → s.area() → circle.area()
