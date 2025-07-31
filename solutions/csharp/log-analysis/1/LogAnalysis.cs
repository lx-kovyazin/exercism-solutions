using System;

public static class LogAnalysis
{
    public static int IndexAfterOf(this string logLine, string value) => logLine.IndexOf(value) + value.Length;
    public static string SubstringAfter(this string logLine, string value) => logLine[logLine.IndexAfterOf(value)..];
    public static string SubstringBetween(this string logLine, string left, string right) => logLine[logLine.IndexAfterOf(left)..logLine.IndexOf(right)];
    public static string Message(this string logLine) => logLine.SubstringAfter(": ");
    public static string LogLevel(this string logLine) => logLine.SubstringBetween("[", "]");
}