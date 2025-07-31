#if !defined(TRIANGLE_H)
#define TRIANGLE_H
#include <stdexcept>

namespace triangle {
    enum class flavor {
        equilateral,
        isosceles,
        scalene,
    };

    flavor kind(const double x, const double y, const double z) noexcept(false);

}  // namespace triangle

#endif // TRIANGLE_H