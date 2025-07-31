using System;

static class Badge
{
    public static string Print(int? id, string name, string? department)
        => (id is null ? string.Empty : $"[{id}] - ") + $"{name} - {(department ?? "owner").ToUpper()}";
}
