using System.Collections.Generic;

enum LogLevel
{
    Unknown,
    Trace,
    Debug,
    Info = 4,
    Warning,
    Error,
    Fatal = 42,
}

static class LogLine
{
    private static readonly Dictionary<string, LogLevel> logLevelMap = new() {
        ["TRC"] = LogLevel.Trace,
        ["DBG"] = LogLevel.Debug,
        ["INF"] = LogLevel.Info,
        ["WRN"] = LogLevel.Warning,
        ["ERR"] = LogLevel.Error,
        ["FTL"] = LogLevel.Fatal,
    };

    public static LogLevel ParseLogLevel(string logLine)
        => logLevelMap.GetValueOrDefault(logLine.Split(':')[0][1..^1], LogLevel.Unknown);

    public static string OutputForShortLog(LogLevel logLevel, string message)
        => $"{(int)logLevel}:{message}";
}
