using System;

static class LogLine
{
    static (string LogLevel, string Message) Split(string logLine) {
        var line = logLine.Split(':');
        return (line[0][1..^1].ToLower(), line[1].Trim());
    }

    public static string Message(string logLine) => Split(logLine).Message;

    public static string LogLevel(string logLine) => Split(logLine).LogLevel;

    public static string Reformat(string logLine) {
        (string LogLevel, string Message) = Split(logLine);
        return $"{Message} ({LogLevel})";
    }
}
