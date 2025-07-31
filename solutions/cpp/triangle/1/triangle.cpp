#include "triangle.h"
#include <cstdint>
#include <array>
#include <algorithm>

namespace triangle {
    enum triangle_side : size_t {
        triangle_side_x,
        triangle_side_y,
        triangle_side_z,
        triangle_side_count
    };
    using triangle_sides = std::array<double, triangle_side_count>;

    static inline bool triangle_inequality(const triangle_sides &sides) {
        return sides[triangle_side_z] <= sides[triangle_side_x] + sides[triangle_side_y];
    }

    static inline void assert_triangle_positive(const triangle_sides &sides) {
        if (!std::all_of(sides.begin(), sides.end(), [](double side) { return side > 0; })) {
            throw std::domain_error("Triangle's sides must be positive.");
        }
    }

    static void assert_triangle_inequality(triangle_sides sides) {
        std::sort(sides.begin(), sides.end());
        do {
            if (!triangle_inequality(sides))
                throw std::domain_error("Violating triangle inequality are illegal.");
        } while (std::next_permutation(sides.begin(), sides.end()));
    }

    static inline void assert_triangle(const double x, const double y, const double z) {
        triangle_sides sides = {x, y, z};
        assert_triangle_positive(sides);
        assert_triangle_inequality(sides);
    }

    flavor kind(const double x, const double y, const double z) {
        assert_triangle(x, y, z);
        if (x == y && y == z) {
            return flavor::equilateral;
        }
        if (x == y || y == z || x == z) {
            return flavor::isosceles;
        }
        return flavor::scalene;
    }
}  // namespace triangle
