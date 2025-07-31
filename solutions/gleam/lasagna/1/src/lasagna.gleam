pub fn expected_minutes_in_oven() -> Int {
  40
}

pub fn remaining_minutes_in_oven(actual: Int) -> Int {
  expected_minutes_in_oven() - actual
}

pub fn preparation_time_in_minutes(layers: Int) -> Int {
  2 * layers
}

pub fn total_time_in_minutes(layers: Int, minutes: Int) -> Int {
  preparation_time_in_minutes(layers) + minutes
}

pub fn alarm() {
  "Ding!"
}
