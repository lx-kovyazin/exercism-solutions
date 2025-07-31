#include "leap.h"

namespace leap {
auto is_leap_year(int year) -> bool {
    return year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
}
}  // namespace leap
