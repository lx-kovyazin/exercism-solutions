module BookingUpForBeauty
open System

let schedule (appointmentDateDescription: string): DateTime =
    DateTime.Parse appointmentDateDescription
let hasPassed (appointmentDate: DateTime): bool =
    appointmentDate < DateTime.Now
let isAfternoonAppointment (appointmentDate: DateTime): bool =
    appointmentDate.TimeOfDay.Hours |> fun h -> 12 <= h && h < 18
let description (appointmentDate: DateTime): string =
    $"You have an appointment on {appointmentDate}."
let anniversaryDate(): DateTime =
    DateTime(DateTime.Today.Year, 9, 15)
