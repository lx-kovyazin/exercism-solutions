#if !defined(REVERSE_STRING_H)
#define REVERSE_STRING_H

#include <string>

using namespace std;

namespace reverse_string {
    auto reverse_string(string &&src) -> string;
}  // namespace reverse_string

#endif // REVERSE_STRING_H