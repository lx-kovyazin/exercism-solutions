#include "reverse_string.h"
#include <algorithm>

namespace reverse_string {
    auto reverse_string(string &&src) -> string {
        reverse(src.begin(), src.end());
        return src;
    }
}  // namespace reverse_string
