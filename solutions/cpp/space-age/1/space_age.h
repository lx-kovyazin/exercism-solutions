#if !defined(SPACE_AGE_H)
#define SPACE_AGE_H

namespace space_age {
    struct space_age {
        const size_t age;
        explicit space_age(const size_t &&age) : age(age) {}

        auto seconds() const -> size_t {
            return this->age;
        }

#define AGE_ON_PLANET(planet, period) \
    inline auto on_##planet() const -> float { \
        return static_cast<float>(this->age) / (period * 31557600.0f); \
    }
        AGE_ON_PLANET(mercury, 000.24084670f) // 1
        AGE_ON_PLANET(venus  , 000.61519726f) // 2
        AGE_ON_PLANET(earth  , 001.00000000f) // 3
        AGE_ON_PLANET(mars   , 001.88081580f) // 4
        AGE_ON_PLANET(jupiter, 011.86261500f) // 5
        AGE_ON_PLANET(saturn , 029.44749800f) // 6
        AGE_ON_PLANET(uranus , 084.01684600f) // 7
        AGE_ON_PLANET(neptune, 164.79132000f) // 8
    };
}  // namespace space_age

#endif // SPACE_AGE_H